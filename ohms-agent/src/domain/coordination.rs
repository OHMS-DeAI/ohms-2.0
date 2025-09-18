use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::collections::HashMap;
use super::agent::*;
use super::task::*;
use super::instruction::*;

// Core coordination types
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationSession {
    pub session_id: String,
    pub coordination_type: CoordinationType,
    pub participants: Vec<CoordinationParticipant>,
    pub coordinator: Option<String>,
    pub status: CoordinationSessionStatus,
    pub objectives: Vec<CoordinationObjective>,
    pub protocols: CoordinationProtocols,
    pub state: CoordinationSessionState,
    pub metrics: CoordinationSessionMetrics,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationParticipant {
    pub agent_id: String,
    pub role: CoordinationRole,
    pub capabilities: Vec<String>,
    pub authority_level: AuthorityLevel,
    pub trust_score: f64,
    pub participation_history: ParticipationHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ParticipationHistory {
    pub sessions_participated: u32,
    pub success_rate: f64,
    pub average_contribution_quality: f64,
    pub collaboration_score: f64,
    pub conflict_involvement: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationSessionStatus {
    Initializing,
    Planning,
    Active,
    Negotiating,
    Synchronizing,
    Completing,
    Completed,
    Failed { reason: String },
    Suspended { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationObjective {
    pub objective_id: String,
    pub description: String,
    pub priority: ObjectivePriority,
    pub success_criteria: Vec<SuccessCriteria>,
    pub responsible_agents: Vec<String>,
    pub deadline: Option<u64>,
    pub status: ObjectiveStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ObjectivePriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SuccessCriteria {
    pub criteria_id: String,
    pub description: String,
    pub measurement_method: MeasurementMethod,
    pub threshold: f64,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MeasurementMethod {
    Quantitative { metric: String, unit: String },
    Qualitative { assessment_method: String },
    Boolean { condition: String },
    Comparative { baseline: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ObjectiveStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationProtocols {
    pub communication_protocol: CommunicationProtocol,
    pub decision_making_protocol: DecisionMakingProtocol,
    pub conflict_resolution_protocol: ConflictResolutionProtocol,
    pub synchronization_protocol: SynchronizationProtocol,
    pub reporting_protocol: ReportingProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct DecisionMakingProtocol {
    pub decision_method: DecisionMethod,
    pub voting_mechanism: Option<VotingMechanism>,
    pub consensus_threshold: f64,
    pub timeout_handling: DecisionTimeoutHandling,
    pub escalation_procedure: EscalationProcedure,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DecisionMethod {
    Consensus,        // Require consensus
    Majority,         // Simple majority
    WeightedVoting,   // Authority-weighted voting
    LeaderDecision,   // Leader decides
    ExpertPanel,      // Expert group decides
    Democratic,       // Equal voting
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct VotingMechanism {
    pub voting_type: VotingType,
    pub vote_weights: HashMap<String, f64>,
    pub anonymous_voting: bool,
    pub multiple_rounds_allowed: bool,
    pub vote_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum VotingType {
    Binary,           // Yes/No
    Ranked,           // Ranked preferences
    Scored,           // Numerical scores
    Approval,         // Approve/Disapprove multiple options
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DecisionTimeoutHandling {
    DefaultToNo,
    DefaultToYes,
    ExtendDeadline,
    EscalateToAuthority,
    RequireParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct EscalationProcedure {
    pub escalation_triggers: Vec<EscalationTrigger>,
    pub escalation_hierarchy: Vec<String>,
    pub escalation_timeout: u64,
    pub fallback_decision: FallbackDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum FallbackDecision {
    AbortOperation,
    DefaultOption,
    HumanIntervention,
    RandomSelection,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConflictResolutionProtocol {
    pub detection_methods: Vec<ConflictDetectionMethod>,
    pub resolution_strategies: Vec<ConflictResolutionStrategy>,
    pub mediation_process: MediationProcess,
    pub escalation_path: ConflictEscalationPath,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConflictDetectionMethod {
    DisagreementThreshold,    // Track disagreement levels
    PerformanceDeviation,     // Monitor performance differences
    ResourceContention,       // Detect resource conflicts
    GoalInconsistency,       // Check for conflicting goals
    BehavioralAnomalies,     // Monitor unusual behavior
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MediationProcess {
    pub mediator_selection: MediatorSelection,
    pub mediation_stages: Vec<MediationStage>,
    pub resolution_criteria: Vec<ResolutionCriteria>,
    pub timeout_handling: MediationTimeoutHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MediatorSelection {
    NeutralThirdParty,
    RotatingMediator,
    ExpertMediator { domain: String },
    RandomSelection,
    UserDesignated,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MediationStage {
    pub stage_name: String,
    pub stage_type: MediationStageType,
    pub duration_limit: u64,
    pub success_criteria: Vec<String>,
    pub fallback_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MediationStageType {
    FactFinding,
    IssueIdentification,
    OptionGeneration,
    Evaluation,
    Negotiation,
    Agreement,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResolutionCriteria {
    pub criteria_name: String,
    pub importance_weight: f64,
    pub measurement_method: String,
    pub minimum_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MediationTimeoutHandling {
    ForceResolution,
    EscalateConflict,
    SuspendOperation,
    ApplyDefaultSolution,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConflictEscalationPath {
    pub escalation_levels: Vec<EscalationLevel>,
    pub automatic_escalation_triggers: Vec<AutoEscalationTrigger>,
    pub manual_escalation_criteria: Vec<String>,
    pub final_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct EscalationLevel {
    pub level_id: String,
    pub authority: String,
    pub powers: Vec<EscalationPower>,
    pub time_limit: u64,
    pub decision_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum EscalationPower {
    OverrideDecision,
    ReassignResources,
    ChangeObjectives,
    RemoveParticipants,
    TerminateSession,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AutoEscalationTrigger {
    pub trigger_condition: String,
    pub escalation_target_level: String,
    pub trigger_threshold: f64,
    pub cooldown_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SynchronizationProtocol {
    pub sync_method: SynchronizationMethod,
    pub sync_frequency: SynchronizationFrequency,
    pub checkpoint_intervals: Vec<u64>,
    pub coordination_points: Vec<CoordinationPoint>,
    pub deadlock_prevention: DeadlockPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SynchronizationMethod {
    Barrier,          // Wait for all participants
    Phased,          // Sequential phases
    EventDriven,     // Event-based synchronization
    Periodic,        // Regular intervals
    Adaptive,        // Dynamically adjust timing
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SynchronizationFrequency {
    Continuous,
    HighFrequency,    // Every few seconds
    MediumFrequency,  // Every minute
    LowFrequency,     // Every few minutes
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationPoint {
    pub point_id: String,
    pub point_type: CoordinationPointType,
    pub required_participants: Vec<String>,
    pub timeout: u64,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationPointType {
    Checkpoint,       // Progress verification
    Decision,         // Decision point
    Synchronization,  // Sync point
    Handoff,         // Task handoff
    Validation,      // Quality check
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct DeadlockPrevention {
    pub detection_interval: u64,
    pub prevention_strategies: Vec<DeadlockStrategy>,
    pub resolution_timeout: u64,
    pub fallback_actions: Vec<DeadlockFallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DeadlockStrategy {
    ResourceOrdering,
    TimeoutMechanism,
    PriorityAssignment,
    ResourcePreemption,
    CircularWaitPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DeadlockFallback {
    ForceProgress,
    RestartCoordination,
    RemoveBlockingAgent,
    ChangeResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ReportingProtocol {
    pub reporting_frequency: ReportingFrequency,
    pub report_types: Vec<ReportType>,
    pub reporting_format: ReportingFormat,
    pub report_distribution: ReportDistribution,
    pub performance_indicators: Vec<PerformanceIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ReportingFrequency {
    RealTime,
    PerMinute,
    PerHour,
    Daily,
    OnCompletion,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ReportType {
    Progress,
    Performance,
    Issues,
    Quality,
    Resource,
    Coordination,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ReportingFormat {
    Summary,
    Detailed,
    Dashboard,
    Alerts,
    Raw,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ReportDistribution {
    pub internal_recipients: Vec<String>,
    pub external_recipients: Vec<String>,
    pub notification_channels: Vec<NotificationChannel>,
    pub access_controls: Vec<AccessControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum NotificationChannel {
    Direct,
    Broadcast,
    Email,
    Dashboard,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AccessControl {
    pub principal: String,
    pub access_level: AccessLevel,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceIndicator {
    pub indicator_name: String,
    pub measurement_method: String,
    pub target_value: f64,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationSessionState {
    pub current_phase: CoordinationPhase,
    pub active_decisions: Vec<ActiveDecision>,
    pub resource_allocations: Vec<CoordinationResourceAllocation>,
    pub communication_state: CommunicationState,
    pub synchronization_state: SynchronizationState,
    pub conflict_state: ConflictState,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationPhase {
    Initialization,
    Planning,
    ResourceAllocation,
    Execution,
    Monitoring,
    Synchronization,
    Resolution,
    Completion,
    Cleanup,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ActiveDecision {
    pub decision_id: String,
    pub decision_type: DecisionType,
    pub status: DecisionStatus,
    pub participants: Vec<String>,
    pub options: Vec<DecisionOption>,
    pub votes: HashMap<String, Vote>,
    pub deadline: u64,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DecisionType {
    ResourceAllocation,
    TaskAssignment,
    QualityThreshold,
    DeadlineAdjustment,
    ConflictResolution,
    StrategyChange,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DecisionStatus {
    Open,
    Voting,
    Decided,
    Implementing,
    Completed,
    Overturned,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct DecisionOption {
    pub option_id: String,
    pub description: String,
    pub impact_assessment: ImpactAssessment,
    pub resource_requirements: ResourceRequirements,
    pub risk_assessment: RiskAssessment,
    pub proposed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ImpactAssessment {
    pub performance_impact: f64,
    pub resource_impact: f64,
    pub timeline_impact: f64,
    pub quality_impact: f64,
    pub stakeholder_impact: Vec<StakeholderImpact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct StakeholderImpact {
    pub stakeholder: String,
    pub impact_type: ImpactType,
    pub impact_magnitude: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ImpactType {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Vote {
    pub voter_id: String,
    pub option_id: String,
    pub vote_value: VoteValue,
    pub reasoning: Option<String>,
    pub confidence: f64,
    pub cast_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum VoteValue {
    Binary { support: bool },
    Ranked { rank: u32 },
    Scored { score: f64 },
    Weighted { weight: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationResourceAllocation {
    pub allocation_id: String,
    pub resource_type: String,
    pub allocated_to: String,
    pub quantity: f64,
    pub allocation_start: u64,
    pub allocation_end: Option<u64>,
    pub status: AllocationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AllocationStatus {
    Requested,
    Approved,
    Active,
    Released,
    Disputed,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CommunicationState {
    pub active_channels: Vec<ActiveChannel>,
    pub message_queues: HashMap<String, Vec<QueuedMessage>>,
    pub communication_metrics: CommunicationMetrics,
    pub protocol_violations: Vec<ProtocolViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ActiveChannel {
    pub channel_id: String,
    pub channel_type: ChannelType,
    pub participants: Vec<String>,
    pub status: ChannelStatus,
    pub message_count: u32,
    pub last_activity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ChannelStatus {
    Active,
    Idle,
    Congested,
    Error,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct QueuedMessage {
    pub message_id: String,
    pub sender: String,
    pub recipient: String,
    pub message_type: MessageType,
    pub priority: MessagePriority,
    pub content: String,
    pub queued_at: u64,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Urgent,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CommunicationMetrics {
    pub message_throughput: f64,
    pub average_response_time: f64,
    pub communication_efficiency: f64,
    pub protocol_compliance: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ProtocolViolation {
    pub violation_id: String,
    pub violation_type: ViolationType,
    pub violator: String,
    pub description: String,
    pub severity: ViolationSeverity,
    pub occurred_at: u64,
    pub action_taken: Option<ViolationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ViolationType {
    UnauthorizedAccess,
    MessageFormatError,
    ProtocolDeviation,
    TimeoutViolation,
    ResourceMisuse,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ViolationSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ViolationAction {
    Warning,
    Throttling,
    Suspension,
    Termination,
    Escalation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SynchronizationState {
    pub sync_points: Vec<SyncPoint>,
    pub agent_readiness: HashMap<String, ReadinessState>,
    pub global_state: GlobalSyncState,
    pub sync_metrics: SynchronizationMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SyncPoint {
    pub point_id: String,
    pub point_type: SyncPointType,
    pub required_agents: Vec<String>,
    pub ready_agents: Vec<String>,
    pub status: SyncPointStatus,
    pub deadline: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SyncPointType {
    Barrier,
    Checkpoint,
    Handoff,
    Decision,
    Validation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SyncPointStatus {
    Waiting,
    Ready,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ReadinessState {
    NotReady,
    Preparing,
    Ready,
    Busy,
    Blocked,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum GlobalSyncState {
    Synchronized,
    Synchronizing,
    Desynchronized,
    ConflictingStates,
    RecoveryMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SynchronizationMetrics {
    pub sync_efficiency: f64,
    pub average_sync_time: f64,
    pub sync_failure_rate: f64,
    pub deadlock_incidents: u32,
    pub recovery_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConflictState {
    pub active_conflicts: Vec<ActiveConflict>,
    pub resolved_conflicts: Vec<ResolvedConflict>,
    pub conflict_metrics: ConflictMetrics,
    pub mediation_sessions: Vec<MediationSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ActiveConflict {
    pub conflict_id: String,
    pub conflict_type: ConflictType,
    pub involved_parties: Vec<String>,
    pub conflict_description: String,
    pub severity: ConflictSeverity,
    pub status: ConflictStatus,
    pub created_at: u64,
    pub escalation_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConflictType {
    ResourceContention,
    GoalMisalignment,
    MethodologyDisagreement,
    QualityDispute,
    PriorityConflict,
    AuthorityChallenge,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConflictStatus {
    Detected,
    Analyzing,
    Mediating,
    Escalated,
    Resolved,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResolvedConflict {
    pub conflict_id: String,
    pub resolution_method: ResolutionMethod,
    pub resolution_outcome: ResolutionOutcome,
    pub resolution_time: u64,
    pub satisfaction_scores: HashMap<String, f64>,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ResolutionMethod {
    Negotiation,
    Mediation,
    Arbitration,
    Compromise,
    Escalation,
    ForceDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ResolutionOutcome {
    WinWin,
    WinLose,
    Compromise,
    NoResolution,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConflictMetrics {
    pub conflict_rate: f64,
    pub average_resolution_time: f64,
    pub resolution_success_rate: f64,
    pub escalation_rate: f64,
    pub participant_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MediationSession {
    pub session_id: String,
    pub mediator: String,
    pub participants: Vec<String>,
    pub conflict_id: String,
    pub status: MediationStatus,
    pub stages_completed: Vec<String>,
    pub agreements_reached: Vec<Agreement>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MediationStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Agreement {
    pub agreement_id: String,
    pub agreement_type: AgreementType,
    pub terms: Vec<AgreementTerm>,
    pub signatories: Vec<String>,
    pub enforcement_mechanism: EnforcementMechanism,
    pub validity_period: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AgreementType {
    Interim,      // Temporary agreement
    Final,        // Final resolution
    Conditional,  // Subject to conditions
    Partial,      // Covers some issues
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AgreementTerm {
    pub term_id: String,
    pub description: String,
    pub responsible_party: String,
    pub deadline: Option<u64>,
    pub verification_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum EnforcementMechanism {
    SelfEnforcement,
    PeerMonitoring,
    AutomaticCompliance,
    EscalationOnViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationSessionMetrics {
    pub session_duration: u64,
    pub participation_metrics: ParticipationMetrics,
    pub efficiency_metrics: CoordinationEfficiencyMetrics,
    pub quality_metrics: CoordinationQualityMetrics,
    pub outcome_metrics: OutcomeMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ParticipationMetrics {
    pub active_participation_rate: f64,
    pub communication_balance: f64,
    pub decision_participation_rate: f64,
    pub conflict_involvement_rate: f64,
    pub satisfaction_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationEfficiencyMetrics {
    pub coordination_overhead: f64,
    pub decision_making_speed: f64,
    pub resource_utilization_efficiency: f64,
    pub communication_efficiency: f64,
    pub synchronization_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationQualityMetrics {
    pub decision_quality: f64,
    pub conflict_resolution_quality: f64,
    pub outcome_coherence: f64,
    pub process_adherence: f64,
    pub stakeholder_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OutcomeMetrics {
    pub objective_achievement_rate: f64,
    pub quality_of_outcomes: f64,
    pub timeliness_score: f64,
    pub resource_efficiency_score: f64,
    pub innovation_score: f64,
    pub sustainability_score: f64,
}

// Coordination management types
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationPlan {
    pub plan_id: String,
    pub coordination_type: CoordinationType,
    pub agent_roles: Vec<AgentRole>,
    pub communication_pattern: CommunicationPattern,
    pub consensus_mechanism: ConsensusMechanism,
    pub failure_handling: FailureHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationRequest {
    pub request_id: String,
    pub requester: String,
    pub coordination_type: CoordinationType,
    pub objective: String,
    pub required_participants: Vec<String>,
    pub preferred_participants: Vec<String>,
    pub deadline: Option<u64>,
    pub resource_requirements: ResourceRequirements,
    pub priority: CoordinationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationPriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationResponse {
    pub response_id: String,
    pub request_id: String,
    pub responder: String,
    pub response_type: CoordinationResponseType,
    pub availability: AvailabilityInfo,
    pub capability_match: CapabilityMatch,
    pub resource_offer: Option<ResourceOffer>,
    pub conditions: Vec<ParticipationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationResponseType {
    Accept,
    Decline,
    Conditional,
    CounterProposal,
    NeedMoreInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AvailabilityInfo {
    pub immediate_availability: bool,
    pub available_from: Option<u64>,
    pub available_until: Option<u64>,
    pub capacity_percentage: f64,
    pub competing_priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CapabilityMatch {
    pub required_capabilities_covered: Vec<String>,
    pub missing_capabilities: Vec<String>,
    pub additional_capabilities: Vec<String>,
    pub proficiency_scores: HashMap<String, f64>,
    pub overall_match_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResourceOffer {
    pub offered_resources: Vec<OfferedResource>,
    pub resource_constraints: Vec<ResourceConstraint>,
    pub offer_duration: u64,
    pub offer_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OfferedResource {
    pub resource_type: String,
    pub quantity: f64,
    pub quality_level: String,
    pub availability_schedule: AvailabilitySchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AvailabilitySchedule {
    pub schedule_type: ScheduleType,
    pub time_slots: Vec<TimeSlot>,
    pub recurring_pattern: Option<RecurringPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ScheduleType {
    Continuous,
    Scheduled,
    OnDemand,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TimeSlot {
    pub start_time: u64,
    pub end_time: u64,
    pub capacity: f64,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RecurringPattern {
    pub pattern_type: PatternType,
    pub interval: u64,
    pub duration: u64,
    pub exceptions: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum PatternType {
    Daily,
    Weekly,
    Monthly,
    Custom { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResourceConstraint {
    pub constraint_type: ConstraintType,
    pub description: String,
    pub impact: ConstraintImpact,
    pub mitigation_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConstraintType {
    Temporal,
    Capacity,
    Quality,
    Dependency,
    Regulatory,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConstraintImpact {
    Blocking,
    Limiting,
    Conditional,
    Preference,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ParticipationCondition {
    pub condition_type: ConditionType,
    pub description: String,
    pub negotiable: bool,
    pub impact_on_participation: ConditionImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConditionImpact {
    MustBeMet,
    PreferredToBeMet,
    NegotiableRequirement,
    OptionalRequirement,
}

// Advanced coordination patterns
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationPattern {
    pub pattern_id: String,
    pub pattern_name: String,
    pub pattern_type: CoordinationPatternType,
    pub description: String,
    pub applicability_conditions: Vec<ApplicabilityCondition>,
    pub implementation_steps: Vec<ImplementationStep>,
    pub expected_outcomes: Vec<ExpectedOutcome>,
    pub success_metrics: Vec<SuccessMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CoordinationPatternType {
    Hierarchical,
    Flat,
    Network,
    Pipeline,
    StarTopology,
    RingTopology,
    MeshTopology,
    HybridTopology,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ApplicabilityCondition {
    pub condition_description: String,
    pub condition_type: ApplicabilityType,
    pub evaluation_method: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ApplicabilityType {
    ParticipantCount,
    TaskComplexity,
    ResourceAvailability,
    TimeConstraints,
    QualityRequirements,
    RiskTolerance,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ImplementationStep {
    pub step_id: String,
    pub step_name: String,
    pub step_description: String,
    pub prerequisites: Vec<String>,
    pub actions: Vec<CoordinationAction>,
    pub validation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationAction {
    pub action_type: ActionType,
    pub target_participants: Vec<String>,
    pub action_parameters: HashMap<String, String>,
    pub expected_duration: u64,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ActionType {
    EstablishCommunication,
    AllocateResources,
    AssignRoles,
    SetObjectives,
    InitiateNegotiation,
    SynchronizeState,
    ValidateResults,
    ResolveConflict,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ExpectedOutcome {
    pub outcome_description: String,
    pub outcome_type: OutcomeType,
    pub measurement_criteria: Vec<String>,
    pub target_value: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum OutcomeType {
    PerformanceImprovement,
    EfficiencyGain,
    QualityEnhancement,
    ResourceOptimization,
    ConflictReduction,
    SatisfactionIncrease,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SuccessMetric {
    pub metric_name: String,
    pub measurement_method: String,
    pub target_threshold: f64,
    pub weight: f64,
    pub trend_direction: TrendDirection,
}
