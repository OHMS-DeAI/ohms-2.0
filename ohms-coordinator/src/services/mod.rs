use crate::domain::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod agent_spawning;
pub mod autonomous_coord;
pub mod dedup;
pub mod econ_integration;
pub mod http_outcall;
pub mod instruction_analyzer;
pub mod orchestration;
pub mod quota_manager;
pub mod registry;
pub mod routing;

pub use agent_spawning::AgentSpawningService;
pub use autonomous_coord::AutonomousCoordinationService;
pub use dedup::DedupService;
pub use econ_integration::EconIntegrationService;
pub use http_outcall::HttpOutcallService;
pub use instruction_analyzer::InstructionAnalyzerService;
pub use orchestration::OrchestrationService;
pub use quota_manager::QuotaManager;
pub use registry::RegistryService;
pub use routing::RoutingService;

#[derive(Debug)]
pub struct CoordinatorState {
    pub config: CoordinatorConfig,
    pub agents: HashMap<String, AgentRegistration>,
    pub models: HashMap<String, RegisteredModel>,
    pub instruction_requests: HashMap<String, InstructionRequest>,
    pub agent_creation_results: HashMap<String, AgentCreationResult>,
    pub user_quotas: HashMap<String, quota_manager::UserQuota>,
    pub dedup_cache: HashMap<String, DedupEntry>,
    pub metrics: SystemMetrics,
    pub coordination_sessions: Option<HashMap<String, autonomous_coord::CoordinationSession>>,
    pub orchestration_tasks: HashMap<String, OrchestrationTask>,
    pub agent_capabilities: HashMap<String, AgentCapabilities>,
    pub agent_roles: HashMap<String, AgentRole>,
    pub routing_stats: HashMap<String, RoutingStats>,
    pub agent_message_queues: Option<HashMap<String, Vec<autonomous_coord::AgentMessage>>>,
    pub agent_capability_profiles: HashMap<String, autonomous_coord::AgentCapabilityProfile>,
}

impl Default for CoordinatorState {
    fn default() -> Self {
        Self::new()
    }
}

impl CoordinatorState {
    pub fn new() -> Self {
        Self {
            config: CoordinatorConfig::default(),
            agents: HashMap::new(),
            models: HashMap::new(),
            instruction_requests: HashMap::new(),
            agent_creation_results: HashMap::new(),
            user_quotas: HashMap::new(),
            dedup_cache: HashMap::new(),
            metrics: SystemMetrics::default(),
            coordination_sessions: Some(HashMap::new()),
            orchestration_tasks: HashMap::new(),
            agent_capabilities: HashMap::new(),
            agent_roles: HashMap::new(),
            routing_stats: HashMap::new(),
            agent_message_queues: Some(HashMap::new()),
            agent_capability_profiles: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemMetrics {
    pub total_agents: u32,
    pub ready_agents: u32,
    pub total_models: u32,
    pub ready_models: u32,
    pub total_requests: u64,
    pub last_activity: u64,
    pub total_routes: u64,
    pub average_routing_time_ms: f64,
    pub total_agent_creations: u32,
}

thread_local! {
    static STATE: RefCell<CoordinatorState> = RefCell::new(CoordinatorState::new());
}

pub fn with_state<R>(f: impl FnOnce(&CoordinatorState) -> R) -> R {
    STATE.with(|state| f(&state.borrow()))
}

pub fn with_state_mut<R>(f: impl FnOnce(&mut CoordinatorState) -> R) -> R {
    STATE.with(|state| f(&mut state.borrow_mut()))
}
