use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Task status in the orchestration pipeline
#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TaskStatus {
    Created,
    Planning,
    Executing,
    Reviewing,
    Completed,
    Failed,
    Cancelled,
}

/// Complete orchestration task
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct OrchestrationTask {
    pub task_id: String,
    pub user_id: String,
    pub instructions: String,
    pub queen_agent_id: Option<String>,
    pub worker_agents: Vec<String>,
    pub status: TaskStatus,
    pub iterations: Vec<IterationRecord>,
    pub quality_score: f32,
    pub quality_threshold: f32,
    pub max_iterations: u32,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub error_message: Option<String>,
}

impl OrchestrationTask {
    pub fn new(task_id: String, user_id: String, instructions: String, now: u64) -> Self {
        Self {
            task_id,
            user_id,
            instructions,
            queen_agent_id: None,
            worker_agents: Vec::new(),
            status: TaskStatus::Created,
            iterations: Vec::new(),
            quality_score: 0.0,
            quality_threshold: 0.85, // 85% quality threshold
            max_iterations: 10,
            created_at: now,
            completed_at: None,
            error_message: None,
        }
    }

    pub fn should_continue(&self) -> bool {
        self.status != TaskStatus::Completed
            && self.status != TaskStatus::Failed
            && self.status != TaskStatus::Cancelled
            && (self.iterations.len() as u32) < self.max_iterations
            && self.quality_score < self.quality_threshold
    }

    pub fn duration_ms(&self) -> u64 {
        if let Some(completed) = self.completed_at {
            (completed - self.created_at) / 1_000_000
        } else {
            let now = ic_cdk::api::time();
            (now - self.created_at) / 1_000_000
        }
    }
}

/// Single iteration in the orchestration process
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct IterationRecord {
    pub iteration_num: u32,
    pub queen_plan: String,
    pub worker_executions: Vec<WorkerExecution>,
    pub peer_communications: Vec<PeerMessage>,
    pub queen_synthesis: String,
    pub quality_score: f32,
    pub timestamp: u64,
    pub duration_ms: u64,
}

/// Worker agent execution result
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct WorkerExecution {
    pub agent_id: String,
    pub assigned_subtask: String,
    pub result: String,
    pub tokens_used: u32,
    pub execution_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Peer-to-peer communication between agents
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct PeerMessage {
    pub message_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub message_type: PeerMessageType,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum PeerMessageType {
    Question,
    Answer,
    Suggestion,
    Status,
    Error,
}

/// Queen agent execution plan
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct ExecutionPlan {
    pub strategy: String,
    pub subtasks: Vec<Subtask>,
    pub estimated_duration_ms: u64,
    pub success_criteria: Vec<String>,
}

/// Individual subtask to be assigned
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct Subtask {
    pub subtask_id: String,
    pub description: String,
    pub assigned_to: Option<String>,
    pub dependencies: Vec<String>,
    pub priority: u8,
}

/// Task progress summary
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct TaskProgress {
    pub task_id: String,
    pub status: TaskStatus,
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub quality_score: f32,
    pub quality_threshold: f32,
    pub progress_percentage: f32,
    pub estimated_completion_ms: Option<u64>,
    pub queen_agent: Option<String>,
    pub active_workers: u32,
    pub total_tokens_used: u32,
}

impl TaskProgress {
    pub fn from_task(task: &OrchestrationTask) -> Self {
        let progress_percentage = if task.quality_threshold > 0.0 {
            (task.quality_score / task.quality_threshold * 100.0).min(100.0)
        } else {
            0.0
        };

        let total_tokens_used: u32 = task
            .iterations
            .iter()
            .flat_map(|iter| &iter.worker_executions)
            .map(|exec| exec.tokens_used)
            .sum();

        Self {
            task_id: task.task_id.clone(),
            status: task.status.clone(),
            current_iteration: task.iterations.len() as u32,
            max_iterations: task.max_iterations,
            quality_score: task.quality_score,
            quality_threshold: task.quality_threshold,
            progress_percentage,
            estimated_completion_ms: None,
            queen_agent: task.queen_agent_id.clone(),
            active_workers: task.worker_agents.len() as u32,
            total_tokens_used,
        }
    }
}

/// Agent role in orchestration
#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum AgentRole {
    Queen,
    Worker,
    Idle,
}

/// Agent capabilities for role selection
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentCapabilities {
    pub agent_id: String,
    pub can_plan: bool,
    pub can_synthesize: bool,
    pub can_evaluate: bool,
    pub specializations: Vec<String>,
    pub performance_score: f32,
}

impl AgentCapabilities {
    pub fn queen_score(&self) -> f32 {
        let mut score = self.performance_score;
        if self.can_plan {
            score += 0.3;
        }
        if self.can_synthesize {
            score += 0.3;
        }
        if self.can_evaluate {
            score += 0.2;
        }
        score
    }
}

