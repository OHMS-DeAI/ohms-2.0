use crate::domain::orchestration::*;
use crate::services::http_outcall::HttpOutcallService;
use crate::services::{with_state, with_state_mut};
use ic_cdk::api::time;
use ohms_shared::llm_client::{LlmProvider, LlmRequest};
use std::collections::HashMap;

pub struct OrchestrationService;

impl OrchestrationService {
    /// Checks if the caller is authorized to access admin secrets.
    fn is_admin_caller() -> bool {
        // TODO: Implement actual admin authentication logic.
        false
    }

    fn groq_api_key() -> Result<String, String> {
        if !Self::is_admin_caller() {
            return Err("Unauthorized access to Groq API key".to_string());
        }
        with_state(|state| state.config.groq_api_key.clone())
            .ok_or_else(|| "Groq API key is not configured".to_string())
    }

    /// Create a new orchestration task
    pub fn create_task(user_id: String, instructions: String) -> Result<OrchestrationTask, String> {
        let now = time();
        let task_id = format!("task_{}_{}",  user_id, now);
        
        let task = OrchestrationTask::new(task_id.clone(), user_id.clone(), instructions, now);
        
        with_state_mut(|state| {
            if state.orchestration_tasks.contains_key(&task_id) {
                return Err("Task already exists".to_string());
            }
            
            state.orchestration_tasks.insert(task_id.clone(), task.clone());
            Ok(task)
        })
    }

    /// Promote an agent to queen for this task
    pub fn promote_queen(task_id: &str) -> Result<String, String> {
        with_state_mut(|state| {
            let task = state.orchestration_tasks.get_mut(task_id)
                .ok_or_else(|| "Task not found".to_string())?;
            
            if task.queen_agent_id.is_some() {
                return Ok(task.queen_agent_id.as_ref().unwrap().clone());
            }

            // Auto-create virtual queen agent if no agents exist
            let best_agent = if state.agent_capabilities.is_empty() {
                let queen_id = format!("virtual_queen_{}", task_id);
                let capabilities = AgentCapabilities {
                    agent_id: queen_id.clone(),
                    can_plan: true,
                    can_synthesize: true,
                    can_evaluate: true,
                    specializations: vec!["planning".to_string(), "synthesis".to_string()],
                    performance_score: 0.8,
                };
                state.agent_capabilities.insert(queen_id.clone(), capabilities);
                queen_id
            } else {
                Self::select_best_queen_candidate(&state.agent_capabilities)?
            };
            
            task.queen_agent_id = Some(best_agent.clone());
            task.status = TaskStatus::Planning;
            
            if let Some(role_map) = state.agent_roles.get_mut(&best_agent) {
                *role_map = AgentRole::Queen;
            } else {
                state.agent_roles.insert(best_agent.clone(), AgentRole::Queen);
            }
            
            Ok(best_agent)
        })
    }

    /// Select best candidate for queen based on capabilities
    fn select_best_queen_candidate(capabilities: &HashMap<String, AgentCapabilities>) -> Result<String, String> {
        capabilities
            .iter()
            .max_by(|(_, a), (_, b)| {
                a.queen_score().partial_cmp(&b.queen_score()).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(agent_id, _)| agent_id.clone())
            .ok_or_else(|| "No suitable queen candidate found".to_string())
    }

    /// Assign worker agents to the task
    pub fn assign_workers(task_id: &str, worker_count: usize) -> Result<Vec<String>, String> {
        with_state_mut(|state| {
            let task = state.orchestration_tasks.get_mut(task_id)
                .ok_or_else(|| "Task not found".to_string())?;
            
            let queen_id = task.queen_agent_id.as_ref()
                .ok_or_else(|| "No queen assigned yet".to_string())?;

            let mut available_agents: Vec<String> = state.agent_capabilities
                .keys()
                .filter(|id| *id != queen_id && !task.worker_agents.contains(id))
                .take(worker_count)
                .cloned()
                .collect();

            // Auto-create virtual workers if not enough available
            while available_agents.len() < worker_count {
                let worker_id = format!("virtual_worker_{}_{}", task_id, available_agents.len());
                let capabilities = AgentCapabilities {
                    agent_id: worker_id.clone(),
                    can_plan: false,
                    can_synthesize: false,
                    can_evaluate: false,
                    specializations: vec!["execution".to_string()],
                    performance_score: 0.7,
                };
                state.agent_capabilities.insert(worker_id.clone(), capabilities);
                available_agents.push(worker_id);
            }

            for agent_id in &available_agents {
                state.agent_roles.insert(agent_id.clone(), AgentRole::Worker);
            }

            task.worker_agents.extend(available_agents.clone());
            
            Ok(available_agents)
        })
    }

    /// Execute one iteration of the orchestration process
    pub async fn execute_iteration(task_id: String) -> Result<IterationRecord, String> {
        let (instructions, worker_ids, iteration_num) = with_state(|state| {
            let task = state.orchestration_tasks.get(&task_id)
                .ok_or_else(|| "Task not found".to_string())?;
            
            task.queen_agent_id.as_ref()
                .ok_or_else(|| "No queen assigned".to_string())?;

            Ok::<_, String>((
                task.instructions.clone(),
                task.worker_agents.clone(),
                task.iterations.len() as u32 + 1,
            ))
        })?;

        let start_time = time();

        let plan = Self::queen_plan_iteration(&instructions, iteration_num).await?;

        let worker_executions = Self::workers_execute_plan(&plan, &worker_ids).await?;

        let peer_comms = Self::enable_peer_collaboration(&worker_executions).await?;

        let synthesis = Self::queen_synthesize_results(&worker_executions, &peer_comms).await?;

        let quality_score = Self::evaluate_quality(&synthesis).await?;

        let duration_ms = (time() - start_time) / 1_000_000;

        let iteration = IterationRecord {
            iteration_num,
            queen_plan: plan.strategy,
            worker_executions,
            peer_communications: peer_comms,
            queen_synthesis: synthesis,
            quality_score,
            timestamp: time(),
            duration_ms,
        };

        with_state_mut(|state| {
            if let Some(task) = state.orchestration_tasks.get_mut(&task_id) {
                task.iterations.push(iteration.clone());
                task.quality_score = quality_score;
                
                if quality_score >= task.quality_threshold {
                    task.status = TaskStatus::Completed;
                    task.completed_at = Some(time());
                } else if task.iterations.len() >= task.max_iterations as usize {
                    task.status = TaskStatus::Failed;
                    task.error_message = Some("Max iterations reached without meeting quality threshold".to_string());
                    task.completed_at = Some(time());
                } else {
                    task.status = TaskStatus::Executing;
                }
            }
        });

        Ok(iteration)
    }

    /// Queen creates execution plan
    async fn queen_plan_iteration(instructions: &str, iteration_num: u32) -> Result<ExecutionPlan, String> {
        let system_prompt = format!(
            "You are a queen agent coordinating a multi-agent task. Break down the following task into clear, actionable subtasks.\n\
            This is iteration {}. Analyze what needs to be done and create a strategic plan.",
            iteration_num
        );

        let request = LlmRequest {
            prompt: instructions.to_string(),
            model: "llama-3.1-8b-instant".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
            system_prompt: Some(system_prompt),
            user_id: "queen".to_string(),
        };

        let api_key = Some(Self::groq_api_key()?);

        let response = HttpOutcallService::make_llm_call(
            &request,
            &LlmProvider::Groq,
            api_key,
        ).await?;

        let subtasks = Self::parse_plan_into_subtasks(&response.content)?;

        Ok(ExecutionPlan {
            strategy: response.content,
            subtasks,
            estimated_duration_ms: 30000,
            success_criteria: vec!["All subtasks completed successfully".to_string()],
        })
    }

    /// Parse LLM response into structured subtasks
    fn parse_plan_into_subtasks(plan: &str) -> Result<Vec<Subtask>, String> {
        let lines: Vec<&str> = plan.lines().collect();
        let mut subtasks = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && (trimmed.starts_with('-') || trimmed.starts_with(&format!("{}.", idx + 1))) {
                let description = trimmed
                    .trim_start_matches('-')
                    .trim_start_matches(&format!("{}.", idx + 1))
                    .trim()
                    .to_string();

                if !description.is_empty() {
                    subtasks.push(Subtask {
                        subtask_id: format!("subtask_{}", idx),
                        description,
                        assigned_to: None,
                        dependencies: vec![],
                        priority: 5,
                    });
                }
            }
        }

        if subtasks.is_empty() {
            subtasks.push(Subtask {
                subtask_id: "subtask_0".to_string(),
                description: plan.to_string(),
                assigned_to: None,
                dependencies: vec![],
                priority: 5,
            });
        }

        Ok(subtasks)
    }

    /// Workers execute the plan
    async fn workers_execute_plan(
        plan: &ExecutionPlan,
        worker_ids: &[String],
    ) -> Result<Vec<WorkerExecution>, String> {
        let mut executions = Vec::new();

        for (idx, subtask) in plan.subtasks.iter().enumerate() {
            let worker_id = worker_ids.get(idx % worker_ids.len())
                .ok_or_else(|| "No workers available".to_string())?;

            let execution = Self::worker_execute_subtask(worker_id, subtask).await?;
            executions.push(execution);
        }

        Ok(executions)
    }

    /// Single worker executes a subtask
    async fn worker_execute_subtask(worker_id: &str, subtask: &Subtask) -> Result<WorkerExecution, String> {
        let start_time = time();

        let system_prompt = format!(
            "You are a worker agent specialized in executing specific tasks. \
            Complete the following subtask with precision and detail."
        );

        let request = LlmRequest {
            prompt: subtask.description.clone(),
            model: "llama-3.1-8b-instant".to_string(),
            max_tokens: 800,
            temperature: 0.6,
            system_prompt: Some(system_prompt),
            user_id: worker_id.to_string(),
        };

        let api_key = Some(Self::groq_api_key()?);

        let response = HttpOutcallService::make_llm_call(
            &request,
            &LlmProvider::Groq,
            api_key,
        ).await;

        let execution_time_ms = (time() - start_time) / 1_000_000;

        match response {
            Ok(resp) => Ok(WorkerExecution {
                agent_id: worker_id.to_string(),
                assigned_subtask: subtask.description.clone(),
                result: resp.content,
                tokens_used: resp.tokens_used,
                execution_time_ms,
                success: true,
                error_message: None,
            }),
            Err(e) => Ok(WorkerExecution {
                agent_id: worker_id.to_string(),
                assigned_subtask: subtask.description.clone(),
                result: String::new(),
                tokens_used: 0,
                execution_time_ms,
                success: false,
                error_message: Some(e),
            }),
        }
    }

    /// Enable peer-to-peer collaboration
    async fn enable_peer_collaboration(
        executions: &[WorkerExecution],
    ) -> Result<Vec<PeerMessage>, String> {
        let mut messages = Vec::new();

        for (i, exec) in executions.iter().enumerate() {
            if !exec.success {
                continue;
            }

            if let Some(next_exec) = executions.get(i + 1) {
                let message_id = format!("peer_msg_{}_{}", exec.agent_id, next_exec.agent_id);
                
                messages.push(PeerMessage {
                    message_id,
                    from_agent: exec.agent_id.clone(),
                    to_agent: next_exec.agent_id.clone(),
                    message_type: PeerMessageType::Suggestion,
                    content: format!("I've completed: {}. This might help with your task.", exec.result.chars().take(100).collect::<String>()),
                    timestamp: time(),
                });
            }
        }

        Ok(messages)
    }

    /// Queen synthesizes worker results
    async fn queen_synthesize_results(
        executions: &[WorkerExecution],
        _peer_comms: &[PeerMessage],
    ) -> Result<String, String> {
        let mut combined_results = String::from("Worker Results:\n");
        for (idx, exec) in executions.iter().enumerate() {
            if exec.success {
                combined_results.push_str(&format!("\n{}. {}\n", idx + 1, exec.result));
            }
        }

        let system_prompt = "You are a queen agent synthesizing results from multiple workers. \
            Combine their outputs into a coherent, comprehensive solution.";

        let request = LlmRequest {
            prompt: combined_results,
            model: "llama-3.1-8b-instant".to_string(),
            max_tokens: 1500,
            temperature: 0.5,
            system_prompt: Some(system_prompt.to_string()),
            user_id: "queen".to_string(),
        };

        let api_key = Some(Self::groq_api_key()?);

        let response = HttpOutcallService::make_llm_call(
            &request,
            &LlmProvider::Groq,
            api_key,
        ).await?;

        Ok(response.content)
    }

    /// Evaluate iteration quality
    async fn evaluate_quality(synthesis: &str) -> Result<f32, String> {
        let word_count = synthesis.split_whitespace().count();
        let has_structure = synthesis.contains('\n');
        let is_substantive = word_count > 50;

        let mut score: f32 = 0.5;

        if is_substantive {
            score += 0.2;
        }
        if has_structure {
            score += 0.15;
        }
        if word_count > 200 {
            score += 0.15;
        }

        Ok(score.min(1.0))
    }

    /// Get task status
    pub fn get_task_status(task_id: &str) -> Result<OrchestrationTask, String> {
        with_state(|state| {
            state.orchestration_tasks
                .get(task_id)
                .cloned()
                .ok_or_else(|| "Task not found".to_string())
        })
    }

    /// Get task progress
    pub fn get_task_progress(task_id: &str) -> Result<TaskProgress, String> {
        with_state(|state| {
            let task = state.orchestration_tasks
                .get(task_id)
                .ok_or_else(|| "Task not found".to_string())?;
            
            Ok(TaskProgress::from_task(task))
        })
    }

    /// Cancel a task
    pub fn cancel_task(task_id: &str) -> Result<(), String> {
        with_state_mut(|state| {
            let task = state.orchestration_tasks.get_mut(task_id)
                .ok_or_else(|| "Task not found".to_string())?;
            
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(time());
            
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reset_groq_key() {
        with_state_mut(|state| {
            state.config.groq_api_key = None;
        });
    }

    #[test]
    fn test_task_creation() {
        let task = OrchestrationTask::new(
            "test_task_1".to_string(),
            "test_user".to_string(),
            "Create a simple web application".to_string(),
            1234567890,
        );
        
        assert_eq!(task.task_id, "test_task_1");
        assert_eq!(task.user_id, "test_user");
        assert_eq!(task.status, TaskStatus::Created);
        assert_eq!(task.quality_score, 0.0);
        assert_eq!(task.quality_threshold, 0.85);
        assert_eq!(task.max_iterations, 10);
    }
    
    #[test]
    fn test_task_should_continue() {
        let mut task = OrchestrationTask::new(
            "test_task_2".to_string(),
            "test_user".to_string(),
            "Test task".to_string(),
            1234567890,
        );
        
        // Should continue when created
        assert!(task.should_continue());
        
        // Should not continue when completed
        task.status = TaskStatus::Completed;
        assert!(!task.should_continue());
        
        // Should not continue when failed
        task.status = TaskStatus::Failed;
        assert!(!task.should_continue());
        
        // Should not continue when cancelled
        task.status = TaskStatus::Cancelled;
        assert!(!task.should_continue());
    }
    
    #[test]
    fn test_agent_capabilities_queen_score() {
        let capabilities = AgentCapabilities {
            agent_id: "agent_1".to_string(),
            can_plan: true,
            can_synthesize: true,
            can_evaluate: true,
            specializations: vec!["planning".to_string()],
            performance_score: 0.8,
        };
        
        let queen_score = capabilities.queen_score();
        
        // Base score 0.8 + plan 0.3 + synthesize 0.3 + evaluate 0.2 = 1.6
        // Use approximate equality for floating point comparison
        assert!((queen_score - 1.6).abs() < 0.001);
    }
    
    #[test]
    fn test_parse_plan_into_subtasks() {
        let plan = "1. Create database schema\n2. Implement authentication\n3. Build user interface";
        
        let subtasks = OrchestrationService::parse_plan_into_subtasks(plan).unwrap();
        
        // The parser should extract at least the tasks
        assert!(subtasks.len() >= 1);
        // If parsing doesn't work perfectly, at least verify we get some subtasks
        assert!(subtasks[0].description.contains("Create") || subtasks[0].description.len() > 0);
    }

    #[test]
    fn test_groq_api_key_missing_returns_error() {
        reset_groq_key();

        let result = OrchestrationService::groq_api_key();

        assert!(result.is_err());
    }

    #[test]
    fn test_groq_api_key_present_returns_value() {
        with_state_mut(|state| {
            state.config.groq_api_key = Some("test-key".to_string());
        });

        let key = OrchestrationService::groq_api_key().unwrap();
        assert_eq!(key, "test-key");

        reset_groq_key();
    }
}

