use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl TaskPriority {
    pub fn weight(&self) -> u8 {
        match self {
            TaskPriority::Low => 1,
            TaskPriority::Normal => 5,
            TaskPriority::High => 8,
            TaskPriority::Critical => 10,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentTask {
    pub task_id: String,
    pub description: String,
    pub priority: TaskPriority,
    pub deadline: Option<u64>,
    pub context: Vec<(String, String)>,
}

impl AgentTask {
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.push((key.into(), value.into()));
        self
    }
}
