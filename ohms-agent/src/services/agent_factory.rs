use ic_cdk::api::time;

use crate::domain::{
    instruction::AgentType, AgentError, AgentPerformanceMetrics, AgentRecord, AgentStatus,
    AgentStatusInfo, AgentSummary, AgentTask, AgentTaskResult,
};

use super::{generate_id, with_state, with_state_mut, InferenceService, MemoryService};

pub struct AgentFactory;

impl AgentFactory {
    pub async fn create_agent(
        user_id: String,
        instruction: crate::domain::instruction::UserInstruction,
        analysis: crate::domain::instruction::AnalyzedInstruction,
    ) -> Result<AgentRecord, String> {
        if with_state(|state| state.binding.is_none()) {
            return Err(AgentError::ModelNotBound.to_string());
        }

        let binding_state = with_state(|state| state.binding.clone())
            .ok_or_else(|| AgentError::ModelNotBound.to_string())?;
        let binding_model_id = binding_state.model_id.clone();
        let bound_manifest = binding_state.manifest.clone();
        let created_at = time();
        let requires_coordination = analysis.coordination_requirements.requires_coordination;
        let agent = with_state_mut(|state| {
            let agent_id = generate_id(state, "agent");
            let mut record = AgentRecord {
                agent_id: agent_id.clone(),
                user_id: user_id.clone(),
                agent_type: analysis.agent_configuration.agent_type.clone(),
                status: AgentStatus::Ready,
                model_id: Some(binding_model_id.clone()),
                model_manifest: Some(bound_manifest.clone()),
                capabilities: analysis
                    .extracted_capabilities
                    .iter()
                    .map(|cap| cap.name.clone())
                    .collect(),
                coordination_role: None,
                tasks_completed: 0,
                tokens_consumed: 0,
                created_at,
                last_active: created_at,
                last_error: None,
            };
            if requires_coordination {
                record.coordination_role = Some("coordinator".to_string());
            }
            state.metrics.active_agents = state.agents.len() as u32 + 1;
            state
                .performance
                .insert(agent_id.clone(), AgentPerformanceMetrics::default());
            state.agents.insert(agent_id, record.clone());
            record
        });
        MemoryService::record_interaction(
            &format!("CREATE_AGENT:{}", instruction.instruction_text),
            &format!("AGENT:{} READY", agent.agent_id),
            0.4,
        );

        Ok(agent)
    }

    pub async fn create_coordinated_agents(
        user_id: String,
        instruction: crate::domain::instruction::UserInstruction,
        analysis: crate::domain::instruction::AnalyzedInstruction,
    ) -> Result<Vec<AgentRecord>, String> {
        let mut agents = Vec::new();
        let coordinator =
            Self::create_agent(user_id.clone(), instruction.clone(), analysis.clone()).await?;
        agents.push(coordinator.clone());

        let required_agents = analysis
            .coordination_requirements
            .agent_count
            .max(1)
            .saturating_sub(1);

        for idx in 0..required_agents {
            let mut worker_analysis = analysis.clone();
            worker_analysis.agent_configuration.agent_type = match idx % 3 {
                0 => AgentType::ProblemSolver,
                1 => AgentType::Executor,
                _ => AgentType::Researcher,
            };
            let worker =
                Self::create_agent(user_id.clone(), instruction.clone(), worker_analysis).await?;
            with_state_mut(|state| {
                if let Some(agent) = state.agents.get_mut(&worker.agent_id) {
                    agent.coordination_role = Some("worker".to_string());
                }
            });
            agents.push(worker);
        }

        Ok(agents)
    }

    pub async fn execute_task(agent_id: &str, task: AgentTask) -> Result<AgentTaskResult, String> {
        let result = InferenceService::execute_task(agent_id, &task).await?;

        with_state_mut(|state| {
            if let Some(agent) = state.agents.get_mut(agent_id) {
                let now = time();
                if result.success {
                    agent.record_success(result.tokens_used, now);
                } else if let Some(error) = result.error_message.clone() {
                    agent.record_failure(error, now);
                }
            }

            let perf = state
                .performance
                .entry(agent_id.to_string())
                .or_insert_with(AgentPerformanceMetrics::default);
            perf.tasks_completed += 1;
            perf.total_tokens_used += result.tokens_used;
            perf.last_task_timestamp = time();
            perf.average_response_time_ms = ((perf.average_response_time_ms
                * (perf.tasks_completed as f64 - 1.0))
                + result.execution_time_ms as f64)
                / perf.tasks_completed as f64;
            if result.success {
                perf.success_rate = ((perf.success_rate * (perf.tasks_completed as f32 - 1.0))
                    + 1.0)
                    / perf.tasks_completed as f32;
            } else {
                perf.success_rate = ((perf.success_rate * (perf.tasks_completed as f32 - 1.0))
                    + 0.1)
                    / perf.tasks_completed as f32;
            }
        });

        MemoryService::record_interaction(&task.description, &result.result, 0.5);

        Ok(result)
    }

    pub async fn get_agent_status(agent_id: &str) -> Result<AgentStatusInfo, String> {
        with_state(|state| {
            let record = state
                .agents
                .get(agent_id)
                .ok_or_else(|| "agent not found".to_string())?;
            let perf = state
                .performance
                .get(agent_id)
                .cloned()
                .unwrap_or_else(AgentPerformanceMetrics::default);
            let model_bound = state.binding.as_ref().map(|b| b.ready).unwrap_or(false);
            Ok(record.status_info(perf, model_bound))
        })
    }

    pub async fn list_user_agents(user_id: &str) -> Result<Vec<AgentSummary>, String> {
        with_state(|state| {
            let summaries = state
                .agents
                .values()
                .filter(|record| record.user_id == user_id)
                .map(|record| record.summary())
                .collect();
            Ok(summaries)
        })
    }
}
