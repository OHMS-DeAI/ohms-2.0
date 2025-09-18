use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::{instruction::AgentType, task::TaskPriority};
use ohms_shared::ModelManifest;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum AgentStatus {
    Creating,
    Ready,
    Active,
    Paused,
    Completed,
    Error(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentPerformanceMetrics {
    pub tasks_completed: u32,
    pub total_tokens_used: u64,
    pub average_response_time_ms: f64,
    pub success_rate: f32,
    pub last_task_timestamp: u64,
}

impl Default for AgentPerformanceMetrics {
    fn default() -> Self {
        Self {
            tasks_completed: 0,
            total_tokens_used: 0,
            average_response_time_ms: 0.0,
            success_rate: 1.0,
            last_task_timestamp: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentTaskResult {
    pub task_id: String,
    pub success: bool,
    pub result: String,
    pub tokens_used: u64,
    pub execution_time_ms: u64,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentStatusInfo {
    pub agent_id: String,
    pub status: AgentStatus,
    pub performance_metrics: AgentPerformanceMetrics,
    pub model_bound: bool,
    pub created_at: u64,
    pub last_active: u64,
    pub model_manifest: Option<ModelManifest>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentSummary {
    pub agent_id: String,
    pub agent_type: AgentType,
    pub status: AgentStatus,
    pub created_at: u64,
    pub last_active: u64,
}

/// Internal runtime representation for an agent stored inside the canister
/// state. This augments the public types with additional bookkeeping fields
/// that never leave the canister.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRecord {
    pub agent_id: String,
    pub user_id: String,
    pub agent_type: AgentType,
    pub status: AgentStatus,
    pub model_id: Option<String>,
    pub model_manifest: Option<ModelManifest>,
    pub capabilities: Vec<String>,
    pub coordination_role: Option<String>,
    pub tasks_completed: u32,
    pub tokens_consumed: u64,
    pub created_at: u64,
    pub last_active: u64,
    pub last_error: Option<String>,
}

impl AgentRecord {
    pub fn summary(&self) -> AgentSummary {
        AgentSummary {
            agent_id: self.agent_id.clone(),
            agent_type: self.agent_type.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            last_active: self.last_active,
        }
    }

    pub fn status_info(
        &self,
        performance: AgentPerformanceMetrics,
        model_bound: bool,
    ) -> AgentStatusInfo {
        AgentStatusInfo {
            agent_id: self.agent_id.clone(),
            status: self.status.clone(),
            performance_metrics: performance,
            model_bound,
            created_at: self.created_at,
            last_active: self.last_active,
            model_manifest: self.model_manifest.clone(),
        }
    }

    pub fn record_success(&mut self, tokens_used: u64, now: u64) {
        self.tasks_completed += 1;
        self.tokens_consumed += tokens_used;
        self.last_active = now;
        self.status = AgentStatus::Active;
        self.last_error = None;
    }

    pub fn record_failure(&mut self, error: String, now: u64) {
        self.last_active = now;
        self.status = AgentStatus::Error(error.clone());
        self.last_error = Some(error);
    }

    pub fn set_status(&mut self, status: AgentStatus, now: u64) {
        self.status = status;
        self.last_active = now;
    }
}

/// Compact representation of a queued task used by the agent factory when
/// distributing workloads between the spawned agents.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedAgentTask {
    pub task_id: String,
    pub agent_id: String,
    pub priority: TaskPriority,
    pub enqueued_at: u64,
}
