use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum SubscriptionTier {
    Basic,
    Pro,
    Enterprise,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Expert,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum ResponseStyle {
    Concise,
    Detailed,
    Conversational,
    Technical,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum DetailLevel {
    Summary,
    Standard,
    Comprehensive,
    Expert,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CreativityLevel {
    Conservative,
    Balanced,
    Creative,
    Experimental,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum SafetyLevel {
    Strict,
    Standard,
    Flexible,
    Experimental,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct InstructionContext {
    pub domain: Option<String>,
    pub complexity: Option<ComplexityLevel>,
    pub urgency: Option<UrgencyLevel>,
    pub collaboration_needed: bool,
    pub external_tools_required: Vec<String>,
}

impl Default for InstructionContext {
    fn default() -> Self {
        Self {
            domain: None,
            complexity: None,
            urgency: Some(UrgencyLevel::Normal),
            collaboration_needed: false,
            external_tools_required: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentPreferences {
    pub response_style: ResponseStyle,
    pub detail_level: DetailLevel,
    pub creativity_level: CreativityLevel,
    pub safety_level: SafetyLevel,
    pub language: String,
}

impl Default for AgentPreferences {
    fn default() -> Self {
        Self {
            response_style: ResponseStyle::Conversational,
            detail_level: DetailLevel::Standard,
            creativity_level: CreativityLevel::Balanced,
            safety_level: SafetyLevel::Standard,
            language: "en".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct UserInstruction {
    pub instruction_text: String,
    pub user_id: String,
    pub subscription_tier: SubscriptionTier,
    pub context: Option<InstructionContext>,
    pub preferences: Option<AgentPreferences>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum CapabilityCategory {
    TextGeneration,
    CodeGeneration,
    DataAnalysis,
    ContentCreation,
    ProblemSolving,
    Coordination,
    Communication,
    Research,
    Planning,
    Execution,
    Custom(String),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CapabilityPriority {
    Essential,
    Important,
    Helpful,
    Optional,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub category: CapabilityCategory,
    pub priority: CapabilityPriority,
    pub required_tools: Vec<String>,
    pub estimated_tokens: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum ModelPrecision {
    FP32,
    FP16,
    INT8,
    INT4,
    Mixed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum ReasoningLevel {
    Basic,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CreativityRequirement {
    None,
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct ModelRequirements {
    pub recommended_models: Vec<String>,
    pub minimum_context_length: u32,
    pub preferred_precision: ModelPrecision,
    pub specialized_requirements: Vec<String>,
    pub reasoning_capability: ReasoningLevel,
    pub creativity_requirement: CreativityRequirement,
}

impl Default for ModelRequirements {
    fn default() -> Self {
        Self {
            recommended_models: Vec::new(),
            minimum_context_length: 2048,
            preferred_precision: ModelPrecision::Mixed,
            specialized_requirements: Vec::new(),
            reasoning_capability: ReasoningLevel::Intermediate,
            creativity_requirement: CreativityRequirement::Low,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CommunicationStyle {
    Direct,
    Friendly,
    Professional,
    Technical,
    Conversational,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum DecisionMakingStyle {
    Conservative,
    Balanced,
    Aggressive,
    Collaborative,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum RetentionPolicy {
    Session,
    Daily,
    Weekly,
    Persistent,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct MemoryConfiguration {
    pub short_term_capacity: u32,
    pub long_term_capacity: u32,
    pub retention_policy: RetentionPolicy,
    pub sharing_enabled: bool,
}

impl Default for MemoryConfiguration {
    fn default() -> Self {
        Self {
            short_term_capacity: 12,
            long_term_capacity: 120,
            retention_policy: RetentionPolicy::Session,
            sharing_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentPersonality {
    pub helpfulness: f32,
    pub creativity: f32,
    pub thoroughness: f32,
    pub efficiency: f32,
    pub formality: f32,
    pub assertiveness: f32,
}

impl Default for AgentPersonality {
    fn default() -> Self {
        Self {
            helpfulness: 0.7,
            creativity: 0.5,
            thoroughness: 0.7,
            efficiency: 0.6,
            formality: 0.5,
            assertiveness: 0.4,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AgentConfiguration {
    pub agent_type: AgentType,
    pub personality: AgentPersonality,
    pub behavior_rules: Vec<String>,
    pub communication_style: CommunicationStyle,
    pub decision_making: DecisionMakingStyle,
    pub memory_configuration: MemoryConfiguration,
    pub tool_access: Vec<String>,
    pub safety_constraints: Vec<String>,
}

impl Default for AgentConfiguration {
    fn default() -> Self {
        Self {
            agent_type: AgentType::GeneralAssistant,
            personality: AgentPersonality::default(),
            behavior_rules: vec!["Always confirm critical actions".to_string()],
            communication_style: CommunicationStyle::Friendly,
            decision_making: DecisionMakingStyle::Balanced,
            memory_configuration: MemoryConfiguration::default(),
            tool_access: Vec::new(),
            safety_constraints: vec![
                "Respect ICP security and data governance policies".to_string()
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CoordinationType {
    None,
    Sequential,
    Parallel,
    Collaborative,
    Hierarchical,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum CommunicationProtocol {
    Direct,
    Centralized,
    Broadcast,
    Hierarchical,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub enum TaskDistributionStrategy {
    RoundRobin,
    CapabilityBased,
    LoadBalanced,
    PriorityBased,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct CoordinationRequirements {
    pub requires_coordination: bool,
    pub coordination_type: CoordinationType,
    pub agent_count: u32,
    pub communication_protocol: CommunicationProtocol,
    pub task_distribution: TaskDistributionStrategy,
}

impl Default for CoordinationRequirements {
    fn default() -> Self {
        Self {
            requires_coordination: false,
            coordination_type: CoordinationType::None,
            agent_count: 1,
            communication_protocol: CommunicationProtocol::Direct,
            task_distribution: TaskDistributionStrategy::RoundRobin,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct DurationEstimate {
    pub min_duration_seconds: u64,
    pub expected_duration_seconds: u64,
    pub max_duration_seconds: u64,
    pub confidence: f32,
}

impl Default for DurationEstimate {
    fn default() -> Self {
        Self {
            min_duration_seconds: 5,
            expected_duration_seconds: 25,
            max_duration_seconds: 60,
            confidence: 0.6,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AnalyzedInstruction {
    pub original_instruction: UserInstruction,
    pub extracted_capabilities: Vec<Capability>,
    pub model_requirements: ModelRequirements,
    pub agent_configuration: AgentConfiguration,
    pub coordination_requirements: CoordinationRequirements,
    pub estimated_complexity: ComplexityLevel,
    pub estimated_duration: DurationEstimate,
    pub confidence_score: f32,
}

impl AnalyzedInstruction {
    pub fn dominant_capabilities(&self) -> Vec<String> {
        self.extracted_capabilities
            .iter()
            .filter(|cap| {
                matches!(
                    cap.priority,
                    CapabilityPriority::Essential | CapabilityPriority::Important
                )
            })
            .map(|cap| cap.name.clone())
            .collect()
    }
}
