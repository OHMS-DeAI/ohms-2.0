// OHMS 2.0 Shared Types
// This module contains type definitions shared across all OHMS components

pub mod communication;
pub mod registry;
pub mod llm_client;
pub mod types;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

// ==============================================================================
// Core System Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComponentHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SystemHealth {
    pub canister_id: Principal,
    pub status: ComponentHealth,
    pub uptime_seconds: u64,
    pub memory_usage_mb: f32,
    pub last_update: u64,
    pub version: String,
    pub metrics: HashMap<String, String>,
}

// ==============================================================================
// Model Repository Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuantizationFormat {
    NOVAQ,
    GGUF,
    Custom(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ModelState {
    Pending,
    Active,
    Deprecated,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelInfo {
    pub model_id: String,
    pub version: String,
    pub state: ModelState,
    pub quantization_format: QuantizationFormat,
    pub compression_ratio: Option<f32>,
    pub accuracy_retention: Option<f32>,
    pub size_bytes: u64,
    pub uploaded_at: u64,
    pub activated_at: Option<u64>,
}

// ==============================================================================
// Agent Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AgentStatus {
    Creating,
    Ready,
    Active,
    Paused,
    Completed,
    Error(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AgentType {
    GeneralAssistant,
    CodeAssistant,
    ContentCreator,
    DataAnalyst,
    ProblemSolver,
    Coordinator,
    Researcher,
    Planner,
    Executor,
    Custom(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Expert,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AgentInfo {
    pub agent_id: String,
    pub agent_type: AgentType,
    pub model_id: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub created_at: u64,
    pub last_active: u64,
    pub health_score: f32,
}

// ==============================================================================
// Economic Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum JobPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Active,
    Released,
    Refunded,
    Expired,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct JobCost {
    pub job_id: String,
    pub estimated_cost: u64,
    pub base_cost: u64,
    pub priority_multiplier: f32,
    pub protocol_fee: u64,
    pub total_cost: u64,
}

// ==============================================================================
// Coordination Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CoordinationType {
    None,
    Sequential,
    Parallel,
    Collaborative,
    Hierarchical,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CoordinationRequest {
    pub request_id: String,
    pub user_principal: String,
    pub instructions: String,
    pub coordination_type: CoordinationType,
    pub agent_requirements: Vec<AgentRequirement>,
    pub created_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AgentRequirement {
    pub agent_type: AgentType,
    pub capabilities: Vec<String>,
    pub complexity: ComplexityLevel,
    pub urgency: UrgencyLevel,
    pub model_preferences: Vec<String>,
}

// ==============================================================================
// Quantized Artifact Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct QuantizedArtifactMetadata {
    pub format: QuantizationFormat,
    pub artifact_checksum: String,
    pub compression_ratio: f32,
    pub accuracy_retention: f32,
    pub bits_per_weight: Option<f32>,
    pub notes: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ArtifactChunkInfo {
    pub chunk_id: String,
    pub offset: u64,
    pub size_bytes: u64,
    pub sha256: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelManifest {
    pub model_id: String,
    pub version: String,
    pub state: ModelState,
    pub uploaded_at: u64,
    pub activated_at: Option<u64>,
    pub total_size_bytes: u64,
    pub chunk_count: u32,
    pub checksum: String,
    pub quantization: QuantizedArtifactMetadata,
    pub metadata: HashMap<String, String>,
    pub chunks: Vec<ArtifactChunkInfo>,
}

// ==============================================================================
// Cross-Component Communication Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum IntercanisterMessage {
    HealthCheck,
    ModelActivated(String),
    AgentCreated(AgentInfo),
    JobCompleted(String),
    EscrowReleased(String),
    SystemMetrics(SystemHealth),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IntercanisterResponse {
    pub message_id: String,
    pub sender: String,
    pub recipient: String,
    pub payload: IntercanisterMessage,
    pub timestamp: u64,
}

// ==============================================================================
// Error Types
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OHMSError {
    InvalidInput(String),
    InvalidState(String),
    AlreadyExists(String),
    NotFound(String),
    Unauthorized(String),
    InternalError(String),
    NetworkError(String),
    CommunicationFailed(String),
    QuotaExceeded(String),
    InsufficientFunds(String),
    ModelNotReady(String),
    CompressionFailed(String),
}

pub type OHMSResult<T> = Result<T, OHMSError>;

// ==============================================================================
// Utility Functions
// ==============================================================================

impl std::fmt::Display for OHMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OHMSError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            OHMSError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            OHMSError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            OHMSError::NotFound(msg) => write!(f, "Not found: {}", msg),
            OHMSError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            OHMSError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            OHMSError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            OHMSError::CommunicationFailed(msg) => {
                write!(f, "Communication failed: {}", msg)
            }
            OHMSError::QuotaExceeded(msg) => write!(f, "Quota exceeded: {}", msg),
            OHMSError::InsufficientFunds(msg) => write!(f, "Insufficient funds: {}", msg),
            OHMSError::ModelNotReady(msg) => write!(f, "Model not ready: {}", msg),
            OHMSError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
        }
    }
}

impl std::error::Error for OHMSError {}

// Time utilities
// Use IC's time API which returns nanoseconds since Unix epoch
pub fn current_time_nanos() -> u64 {
    ic_cdk::api::time()
}

pub fn current_time_millis() -> u64 {
    current_time_nanos() / 1_000_000
}

pub fn current_time_seconds() -> u64 {
    current_time_nanos() / 1_000_000_000
}

// Re-export all public types and functions
pub use communication::*;
pub use registry::*;
