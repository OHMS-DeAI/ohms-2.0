pub mod agent;
pub mod inference;
pub mod instruction;
pub mod task;
pub mod validation;

pub use agent::*;
pub use inference::*;
pub use instruction::*;
pub use task::*;
pub use validation::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Runtime configuration for the agent canister. Mirrors the public candid type
/// exposed in `ohms_agent.did` so the API remains stable for the UI clients.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub struct AgentConfig {
    pub model_repo_canister_id: String,
    pub warm_set_target: f32,
    pub prefetch_depth: u32,
    pub max_tokens: u32,
    pub concurrency_limit: u32,
    pub ttl_seconds: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model_repo_canister_id: String::new(),
            warm_set_target: 0.6,
            prefetch_depth: 2,
            max_tokens: 2048,
            concurrency_limit: 4,
            ttl_seconds: 120,
        }
    }
}

/// Health snapshot returned to external callers.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentHealth {
    pub model_bound: bool,
    pub cache_hit_rate: f32,
    pub warm_set_utilization: f32,
    pub queue_depth: u32,
    pub last_inference_timestamp: u64,
}

impl Default for AgentHealth {
    fn default() -> Self {
        Self {
            model_bound: false,
            cache_hit_rate: 0.0,
            warm_set_utilization: 0.0,
            queue_depth: 0,
            last_inference_timestamp: 0,
        }
    }
}

/// Aggregated system metrics tracked internally for observability.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    pub total_inference_requests: u64,
    pub total_cache_hits: u64,
    pub total_cache_misses: u64,
    pub average_latency_ms: f64,
    pub active_agents: u32,
}

/// High level error used throughout the agent domain. Converting it to a
/// string ensures candid compatibility without losing important context.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("model is not bound")]
    ModelNotBound,
    #[error("inference request timed out")]
    InferenceTimeout,
    #[error("invalid instruction: {0}")]
    InvalidInstruction(String),
    #[error("validation failed: {0}")]
    ValidationFailed(String),
    #[error("resource limit exceeded: {0}")]
    ResourceLimit(String),
    #[error("cache error: {0}")]
    CacheError(String),
    #[error("external service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },
}

pub type AgentResult<T> = Result<T, AgentError>;
