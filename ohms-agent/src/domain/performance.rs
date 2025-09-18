use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::collections::HashMap;

// Core performance types
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceProfile {
    pub profile_id: String,
    pub agent_id: String,
    pub performance_history: PerformanceHistory,
    pub current_metrics: CurrentPerformanceMetrics,
    pub benchmarks: Vec<BenchmarkResult>,
    pub trends: PerformanceTrends,
    pub predictions: PerformancePredictions,
    pub optimizations: Vec<PerformanceOptimization>,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceHistory {
    pub historical_snapshots: Vec<PerformanceSnapshot>,
    pub performance_events: Vec<PerformanceEvent>,
    pub regression_incidents: Vec<RegressionIncident>,
    pub improvement_milestones: Vec<ImprovementMilestone>,
    pub baseline_measurements: Vec<BaselineMeasurement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceSnapshot {
    pub snapshot_id: String,
    pub timestamp: u64,
    pub metrics: PerformanceMetrics,
    pub context: PerformanceContext,
    pub anomalies: Vec<PerformanceAnomaly>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceMetrics {
    // Response time metrics
    pub response_time: ResponseTimeMetrics,
    
    // Throughput metrics
    pub throughput: ThroughputMetrics,
    
    // Quality metrics
    pub quality: QualityMetrics,
    
    // Resource efficiency metrics
    pub resource_efficiency: ResourceEfficiencyMetrics,
    
    // Reliability metrics
    pub reliability: ReliabilityMetrics,
    
    // User satisfaction metrics
    pub user_satisfaction: UserSatisfactionMetrics,
    
    // Coordination performance
    pub coordination_performance: CoordinationPerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResponseTimeMetrics {
    pub average_response_time: f64,
    pub median_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub min_response_time: f64,
    pub max_response_time: f64,
    pub response_time_variance: f64,
    pub response_time_std_dev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ThroughputMetrics {
    pub requests_per_second: f64,
    pub tasks_completed_per_hour: f64,
    pub peak_throughput: f64,
    pub sustained_throughput: f64,
    pub throughput_efficiency: f64,
    pub capacity_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct QualityMetrics {
    pub overall_quality_score: f64,
    pub accuracy: f64,
    pub completeness: f64,
    pub relevance: f64,
    pub coherence: f64,
    pub consistency: f64,
    pub innovation: f64,
    pub user_perceived_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResourceEfficiencyMetrics {
    pub compute_efficiency: f64,
    pub memory_efficiency: f64,
    pub energy_efficiency: f64,
    pub cost_efficiency: f64,
    pub resource_waste_percentage: f64,
    pub optimization_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ReliabilityMetrics {
    pub uptime_percentage: f64,
    pub error_rate: f64,
    pub failure_rate: f64,
    pub mean_time_between_failures: f64,
    pub mean_time_to_recovery: f64,
    pub availability: f64,
    pub fault_tolerance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct UserSatisfactionMetrics {
    pub overall_satisfaction: f64,
    pub task_completion_satisfaction: f64,
    pub response_quality_satisfaction: f64,
    pub interaction_satisfaction: f64,
    pub recommendation_likelihood: f64,
    pub complaint_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CoordinationPerformanceMetrics {
    pub coordination_efficiency: f64,
    pub consensus_achievement_rate: f64,
    pub conflict_resolution_speed: f64,
    pub communication_effectiveness: f64,
    pub team_synergy_score: f64,
    pub coordination_overhead: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceContext {
    pub workload_characteristics: WorkloadCharacteristics,
    pub system_state: SystemState,
    pub environmental_factors: EnvironmentalFactors,
    pub user_context: UserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct WorkloadCharacteristics {
    pub request_volume: f64,
    pub complexity_distribution: ComplexityDistribution,
    pub task_type_distribution: HashMap<String, f64>,
    pub concurrency_level: f64,
    pub data_size_characteristics: DataSizeCharacteristics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ComplexityDistribution {
    pub simple_tasks_percentage: f64,
    pub moderate_tasks_percentage: f64,
    pub complex_tasks_percentage: f64,
    pub expert_tasks_percentage: f64,
    pub average_complexity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct DataSizeCharacteristics {
    pub average_input_size: f64,
    pub average_output_size: f64,
    pub max_input_size: f64,
    pub max_output_size: f64,
    pub size_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SystemState {
    pub resource_availability: ResourceAvailability,
    pub system_load: SystemLoad,
    pub active_agents: u32,
    pub queue_lengths: HashMap<String, u32>,
    pub cache_hit_rates: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ResourceAvailability {
    pub cpu_availability: f64,
    pub memory_availability: f64,
    pub storage_availability: f64,
    pub network_availability: f64,
    pub external_service_availability: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SystemLoad {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub network_utilization: f64,
    pub storage_utilization: f64,
    pub queue_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct EnvironmentalFactors {
    pub time_of_day: TimeOfDay,
    pub day_of_week: DayOfWeek,
    pub seasonal_factors: SeasonalFactors,
    pub external_load_factors: Vec<ExternalLoadFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum TimeOfDay {
    EarlyMorning,
    Morning,
    Midday,
    Afternoon,
    Evening,
    Night,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SeasonalFactors {
    pub season: Season,
    pub holiday_impact: HolidayImpact,
    pub academic_calendar_impact: AcademicImpact,
    pub business_cycle_impact: BusinessCycleImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum HolidayImpact {
    None,
    Light,
    Moderate,
    Heavy,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AcademicImpact {
    None,
    StartOfTerm,
    MidTerm,
    Finals,
    Break,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum BusinessCycleImpact {
    None,
    QuarterEnd,
    YearEnd,
    BudgetCycle,
    PlanningCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ExternalLoadFactor {
    pub factor_name: String,
    pub factor_type: LoadFactorType,
    pub impact_magnitude: f64,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum LoadFactorType {
    MarketingCampaign,
    ProductLaunch,
    MediaCoverage,
    CompetitorAction,
    TechnicalIssue,
    Viral,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct UserContext {
    pub user_type_distribution: HashMap<String, f64>,
    pub experience_level_distribution: ExperienceLevelDistribution,
    pub geographic_distribution: GeographicDistribution,
    pub device_type_distribution: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ExperienceLevelDistribution {
    pub novice_percentage: f64,
    pub intermediate_percentage: f64,
    pub advanced_percentage: f64,
    pub expert_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GeographicDistribution {
    pub regions: HashMap<String, f64>,
    pub time_zones: HashMap<String, f64>,
    pub languages: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceAnomaly {
    pub anomaly_id: String,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub affected_metrics: Vec<String>,
    pub detected_at: u64,
    pub root_cause: Option<RootCause>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AnomalyType {
    PerformanceDegradation,
    UnexpectedSpike,
    ResourceLeakage,
    QualityDrop,
    ErrorRateIncrease,
    ThroughputDrop,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RootCause {
    pub cause_category: CauseCategory,
    pub description: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub corrective_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CauseCategory {
    SystemResource,
    AlgorithmIssue,
    DataQuality,
    ExternalDependency,
    ConfigurationError,
    WorkloadChange,
    EnvironmentalFactor,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceEvent {
    pub event_id: String,
    pub event_type: PerformanceEventType,
    pub timestamp: u64,
    pub description: String,
    pub impact: PerformanceImpact,
    pub duration: Option<u64>,
    pub affected_agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum PerformanceEventType {
    Improvement,
    Degradation,
    Baseline,
    Benchmark,
    Optimization,
    Anomaly,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceImpact {
    pub impact_magnitude: f64,
    pub impact_direction: ImpactDirection,
    pub affected_dimensions: Vec<PerformanceDimension>,
    pub cascade_effects: Vec<CascadeEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ImpactDirection {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum PerformanceDimension {
    ResponseTime,
    Throughput,
    Quality,
    Reliability,
    Efficiency,
    Satisfaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CascadeEffect {
    pub effect_description: String,
    pub affected_component: String,
    pub delay: u64,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RegressionIncident {
    pub incident_id: String,
    pub detected_at: u64,
    pub regression_magnitude: f64,
    pub affected_metrics: Vec<String>,
    pub potential_causes: Vec<PotentialCause>,
    pub mitigation_actions: Vec<MitigationAction>,
    pub resolution_time: Option<u64>,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PotentialCause {
    pub cause_description: String,
    pub likelihood: f64,
    pub investigation_priority: InvestigationPriority,
    pub supporting_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum InvestigationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MitigationAction {
    pub action_description: String,
    pub action_type: MitigationActionType,
    pub expected_impact: f64,
    pub implementation_effort: f64,
    pub risk_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MitigationActionType {
    Immediate,
    ShortTerm,
    LongTerm,
    Preventive,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ImprovementMilestone {
    pub milestone_id: String,
    pub milestone_name: String,
    pub achieved_at: u64,
    pub improvement_metrics: HashMap<String, f64>,
    pub contributing_factors: Vec<ContributingFactor>,
    pub significance: MilestoneSignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ContributingFactor {
    pub factor_name: String,
    pub contribution_percentage: f64,
    pub factor_type: FactorType,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum FactorType {
    AlgorithmImprovement,
    ResourceOptimization,
    ConfigurationChange,
    TrainingImprovement,
    InfrastructureUpgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum MilestoneSignificance {
    Minor,
    Moderate,
    Major,
    Breakthrough,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BaselineMeasurement {
    pub measurement_id: String,
    pub baseline_type: BaselineType,
    pub measurement_timestamp: u64,
    pub baseline_metrics: PerformanceMetrics,
    pub measurement_conditions: MeasurementConditions,
    pub validity_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum BaselineType {
    Initial,
    Updated,
    Benchmark,
    Control,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MeasurementConditions {
    pub workload_type: String,
    pub system_configuration: String,
    pub environmental_conditions: String,
    pub measurement_methodology: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CurrentPerformanceMetrics {
    pub real_time_metrics: RealTimeMetrics,
    pub rolling_averages: RollingAverages,
    pub performance_indicators: Vec<PerformanceIndicator>,
    pub alerts: Vec<PerformanceAlert>,
    pub recommendations: Vec<PerformanceRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RealTimeMetrics {
    pub current_response_time: f64,
    pub current_throughput: f64,
    pub current_error_rate: f64,
    pub current_resource_utilization: f64,
    pub current_queue_length: u32,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RollingAverages {
    pub one_minute_averages: PerformanceMetrics,
    pub five_minute_averages: PerformanceMetrics,
    pub fifteen_minute_averages: PerformanceMetrics,
    pub one_hour_averages: PerformanceMetrics,
    pub one_day_averages: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceIndicator {
    pub indicator_name: String,
    pub current_value: f64,
    pub target_value: f64,
    pub threshold_values: ThresholdValues,
    pub status: IndicatorStatus,
    pub trend: IndicatorTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ThresholdValues {
    pub green_threshold: f64,
    pub yellow_threshold: f64,
    pub red_threshold: f64,
    pub critical_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum IndicatorStatus {
    Excellent,
    Good,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum IndicatorTrend {
    Improving,
    Stable,
    Degrading,
    Volatile,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceAlert {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub affected_metrics: Vec<String>,
    pub triggered_at: u64,
    pub acknowledged: bool,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AlertType {
    ThresholdBreach,
    AnomalyDetected,
    TrendChange,
    Prediction,
    SLA,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceRecommendation {
    pub recommendation_id: String,
    pub recommendation_type: RecommendationType,
    pub priority: RecommendationPriority,
    pub description: String,
    pub expected_impact: ExpectedImpact,
    pub implementation_effort: ImplementationEffort,
    pub risk_assessment: RecommendationRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RecommendationType {
    ResourceAdjustment,
    ConfigurationChange,
    AlgorithmOptimization,
    WorkloadBalancing,
    CacheOptimization,
    InfrastructureUpgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ExpectedImpact {
    pub performance_improvement: f64,
    pub cost_impact: f64,
    pub risk_reduction: f64,
    pub affected_dimensions: Vec<PerformanceDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ImplementationEffort {
    Minimal,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RecommendationRisk {
    pub risk_level: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RiskFactor {
    pub factor_name: String,
    pub probability: f64,
    pub impact: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BenchmarkResult {
    pub benchmark_id: String,
    pub benchmark_name: String,
    pub benchmark_version: String,
    pub executed_at: u64,
    pub results: BenchmarkResults,
    pub comparison: BenchmarkComparison,
    pub ranking: BenchmarkRanking,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BenchmarkResults {
    pub overall_score: f64,
    pub dimension_scores: HashMap<String, f64>,
    pub raw_measurements: HashMap<String, f64>,
    pub normalized_scores: HashMap<String, f64>,
    pub confidence_intervals: HashMap<String, ConfidenceInterval>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BenchmarkComparison {
    pub baseline_comparison: BaselineComparison,
    pub peer_comparison: PeerComparison,
    pub historical_comparison: HistoricalComparison,
    pub industry_comparison: IndustryComparison,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BaselineComparison {
    pub baseline_score: f64,
    pub improvement_percentage: f64,
    pub significant_improvements: Vec<String>,
    pub regressions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PeerComparison {
    pub peer_group: String,
    pub rank_in_group: u32,
    pub group_size: u32,
    pub percentile: f64,
    pub performance_gaps: Vec<PerformanceGap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceGap {
    pub metric_name: String,
    pub gap_magnitude: f64,
    pub gap_direction: GapDirection,
    pub improvement_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum GapDirection {
    Ahead,
    Behind,
    AtParity,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct HistoricalComparison {
    pub time_period: TimePeriod,
    pub trend_analysis: TrendAnalysis,
    pub cyclical_patterns: Vec<CyclicalPattern>,
    pub growth_metrics: GrowthMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom { days: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TrendAnalysis {
    pub overall_trend: TrendDirection,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    pub inflection_points: Vec<InflectionPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum TrendDirection {
    StronglyIncreasing,
    ModeratelyIncreasing,
    SlightlyIncreasing,
    Stable,
    SlightlyDecreasing,
    ModeratelyDecreasing,
    StronglyDecreasing,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct InflectionPoint {
    pub timestamp: u64,
    pub change_type: ChangeType,
    pub magnitude: f64,
    pub potential_cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ChangeType {
    ImprovementStart,
    ImprovementEnd,
    DegradationStart,
    DegradationEnd,
    PlateauStart,
    PlateauEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CyclicalPattern {
    pub pattern_name: String,
    pub cycle_length: u64,
    pub amplitude: f64,
    pub confidence: f64,
    pub next_peak: u64,
    pub next_trough: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GrowthMetrics {
    pub compound_annual_growth_rate: f64,
    pub period_over_period_growth: f64,
    pub growth_acceleration: f64,
    pub growth_consistency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct IndustryComparison {
    pub industry_segment: String,
    pub industry_percentile: f64,
    pub industry_leaders: Vec<IndustryLeader>,
    pub best_practices: Vec<BestPractice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct IndustryLeader {
    pub leader_name: String,
    pub performance_metrics: HashMap<String, f64>,
    pub competitive_advantages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BestPractice {
    pub practice_name: String,
    pub description: String,
    pub adoption_rate: f64,
    pub effectiveness_score: f64,
    pub implementation_difficulty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BenchmarkRanking {
    pub global_rank: u32,
    pub category_rank: u32,
    pub total_participants: u32,
    pub ranking_criteria: Vec<RankingCriterion>,
    pub ranking_history: Vec<HistoricalRank>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RankingCriterion {
    pub criterion_name: String,
    pub weight: f64,
    pub performance_score: f64,
    pub rank_contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct HistoricalRank {
    pub timestamp: u64,
    pub rank: u32,
    pub score: f64,
    pub participants: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceTrends {
    pub short_term_trends: ShortTermTrends,
    pub long_term_trends: LongTermTrends,
    pub predictive_trends: PredictiveTrends,
    pub trend_correlations: Vec<TrendCorrelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ShortTermTrends {
    pub last_hour_trend: TrendData,
    pub last_day_trend: TrendData,
    pub last_week_trend: TrendData,
    pub momentum_indicators: MomentumIndicators,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct LongTermTrends {
    pub last_month_trend: TrendData,
    pub last_quarter_trend: TrendData,
    pub last_year_trend: TrendData,
    pub lifecycle_trends: LifecycleTrends,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TrendData {
    pub direction: TrendDirection,
    pub magnitude: f64,
    pub consistency: f64,
    pub acceleration: f64,
    pub volatility: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MomentumIndicators {
    pub velocity: f64,
    pub acceleration: f64,
    pub jerk: f64,
    pub momentum_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct LifecycleTrends {
    pub current_phase: LifecyclePhase,
    pub phase_duration: u64,
    pub phase_progression: f64,
    pub next_phase_prediction: PhasePrediction,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum LifecyclePhase {
    Introduction,
    Growth,
    Maturity,
    Optimization,
    Decline,
    Renewal,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PhasePrediction {
    pub predicted_phase: LifecyclePhase,
    pub predicted_transition_time: u64,
    pub confidence: f64,
    pub influencing_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PredictiveTrends {
    pub short_term_predictions: Vec<PerformancePrediction>,
    pub long_term_predictions: Vec<PerformancePrediction>,
    pub scenario_analyses: Vec<ScenarioAnalysis>,
    pub prediction_accuracy: PredictionAccuracy,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformancePrediction {
    pub prediction_horizon: u64,
    pub predicted_metrics: HashMap<String, f64>,
    pub confidence_intervals: HashMap<String, ConfidenceInterval>,
    pub prediction_model: String,
    pub model_accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ScenarioAnalysis {
    pub scenario_name: String,
    pub scenario_description: String,
    pub probability: f64,
    pub predicted_outcomes: HashMap<String, f64>,
    pub impact_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PredictionAccuracy {
    pub overall_accuracy: f64,
    pub metric_accuracies: HashMap<String, f64>,
    pub model_performance: HashMap<String, f64>,
    pub accuracy_trends: TrendData,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TrendCorrelation {
    pub metric_pair: (String, String),
    pub correlation_strength: f64,
    pub correlation_type: CorrelationType,
    pub lag_time: i64,
    pub statistical_significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum CorrelationType {
    Positive,
    Negative,
    NonLinear,
    Cyclical,
    Causal,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformancePredictions {
    pub prediction_models: Vec<PredictionModel>,
    pub current_predictions: Vec<PerformancePrediction>,
    pub prediction_confidence: PredictionConfidence,
    pub prediction_validation: PredictionValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PredictionModel {
    pub model_id: String,
    pub model_name: String,
    pub model_type: ModelType,
    pub training_data: TrainingDataInfo,
    pub model_performance: ModelPerformance,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ModelType {
    LinearRegression,
    TimeSeriesAnalysis,
    MachineLearning,
    NeuralNetwork,
    EnsembleMethod,
    HybridModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TrainingDataInfo {
    pub data_points: u64,
    pub time_range: u64,
    pub data_quality_score: f64,
    pub feature_count: u32,
    pub last_training: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ModelPerformance {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub mean_absolute_error: f64,
    pub root_mean_square_error: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PredictionConfidence {
    pub overall_confidence: f64,
    pub metric_confidence: HashMap<String, f64>,
    pub horizon_confidence: HashMap<String, f64>,
    pub confidence_factors: Vec<ConfidenceFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ConfidenceFactor {
    pub factor_name: String,
    pub impact_on_confidence: f64,
    pub current_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PredictionValidation {
    pub validation_methods: Vec<ValidationMethod>,
    pub historical_accuracy: HashMap<String, f64>,
    pub validation_results: Vec<ValidationResult>,
    pub model_reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ValidationMethod {
    Backtesting,
    CrossValidation,
    OutOfSample,
    RealTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ValidationResult {
    pub validation_date: u64,
    pub predicted_value: f64,
    pub actual_value: f64,
    pub error: f64,
    pub error_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PerformanceOptimization {
    pub optimization_id: String,
    pub optimization_type: OptimizationType,
    pub target_metrics: Vec<String>,
    pub optimization_strategy: OptimizationStrategy,
    pub implementation_plan: OptimizationPlan,
    pub expected_results: OptimizationResults,
    pub status: OptimizationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum OptimizationType {
    Algorithm,
    Resource,
    Configuration,
    Architecture,
    Workflow,
    DataStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationStrategy {
    pub strategy_name: String,
    pub approach: OptimizationApproach,
    pub optimization_techniques: Vec<OptimizationTechnique>,
    pub constraints: Vec<OptimizationConstraint>,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum OptimizationApproach {
    Incremental,
    Radical,
    Experimental,
    Conservative,
    Aggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationTechnique {
    pub technique_name: String,
    pub technique_type: TechniqueType,
    pub expected_impact: f64,
    pub complexity: f64,
    pub risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum TechniqueType {
    Caching,
    Parallelization,
    Batching,
    Pipelining,
    Compression,
    Indexing,
    LoadBalancing,
    MemoryOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationConstraint {
    pub constraint_name: String,
    pub constraint_type: ConstraintType,
    pub constraint_value: f64,
    pub flexibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ConstraintType {
    ResourceLimit,
    QualityThreshold,
    TimeLimit,
    CostLimit,
    SafetyRequirement,
    ComplianceRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationPlan {
    pub phases: Vec<OptimizationPhase>,
    pub timeline: OptimizationTimeline,
    pub resource_requirements: OptimizationResources,
    pub risk_mitigation: Vec<RiskMitigation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationPhase {
    pub phase_name: String,
    pub phase_objectives: Vec<String>,
    pub phase_activities: Vec<OptimizationActivity>,
    pub phase_deliverables: Vec<String>,
    pub estimated_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationActivity {
    pub activity_name: String,
    pub activity_type: ActivityType,
    pub dependencies: Vec<String>,
    pub estimated_effort: f64,
    pub required_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum ActivityType {
    Analysis,
    Design,
    Implementation,
    Testing,
    Validation,
    Deployment,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationTimeline {
    pub planned_start: u64,
    pub planned_end: u64,
    pub milestones: Vec<OptimizationMilestone>,
    pub critical_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationMilestone {
    pub milestone_name: String,
    pub planned_date: u64,
    pub success_criteria: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationResources {
    pub required_expertise: Vec<String>,
    pub computational_resources: ComputationalResourceRequirements,
    pub tool_requirements: Vec<String>,
    pub budget_requirements: BudgetRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ComputationalResourceRequirements {
    pub cpu_hours: f64,
    pub memory_gb_hours: f64,
    pub storage_gb: f64,
    pub network_bandwidth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BudgetRequirements {
    pub development_cost: f64,
    pub infrastructure_cost: f64,
    pub operational_cost: f64,
    pub contingency_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RiskMitigation {
    pub risk_description: String,
    pub probability: f64,
    pub impact: f64,
    pub mitigation_strategy: String,
    pub contingency_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct OptimizationResults {
    pub expected_improvements: HashMap<String, f64>,
    pub confidence_levels: HashMap<String, f64>,
    pub implementation_timeline: u64,
    pub return_on_investment: f64,
    pub success_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum OptimizationStatus {
    Planned,
    InProgress,
    Testing,
    Validating,
    Deploying,
    Completed,
    Failed,
    Cancelled,
}
