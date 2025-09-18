export const idlFactory = ({ IDL }) => {
  const ComplexityLevel = IDL.Variant({
    'Complex' : IDL.Null,
    'Moderate' : IDL.Null,
    'Simple' : IDL.Null,
    'Expert' : IDL.Null,
  });
  const UrgencyLevel = IDL.Variant({
    'Low' : IDL.Null,
    'High' : IDL.Null,
    'Normal' : IDL.Null,
    'Critical' : IDL.Null,
  });
  const InstructionContext = IDL.Record({
    'complexity' : IDL.Opt(ComplexityLevel),
    'urgency' : IDL.Opt(UrgencyLevel),
    'domain' : IDL.Opt(IDL.Text),
    'external_tools_required' : IDL.Vec(IDL.Text),
    'collaboration_needed' : IDL.Bool,
  });
  const DetailLevel = IDL.Variant({
    'Comprehensive' : IDL.Null,
    'Summary' : IDL.Null,
    'Standard' : IDL.Null,
    'Expert' : IDL.Null,
  });
  const SafetyLevel = IDL.Variant({
    'Experimental' : IDL.Null,
    'Strict' : IDL.Null,
    'Standard' : IDL.Null,
    'Flexible' : IDL.Null,
  });
  const CreativityLevel = IDL.Variant({
    'Creative' : IDL.Null,
    'Experimental' : IDL.Null,
    'Balanced' : IDL.Null,
    'Conservative' : IDL.Null,
  });
  const ResponseStyle = IDL.Variant({
    'Technical' : IDL.Null,
    'Detailed' : IDL.Null,
    'Conversational' : IDL.Null,
    'Concise' : IDL.Null,
  });
  const AgentPreferences = IDL.Record({
    'detail_level' : DetailLevel,
    'safety_level' : SafetyLevel,
    'creativity_level' : CreativityLevel,
    'language' : IDL.Text,
    'response_style' : ResponseStyle,
  });
  const SubscriptionTier = IDL.Variant({
    'Pro' : IDL.Null,
    'Enterprise' : IDL.Null,
    'Basic' : IDL.Null,
  });
  const UserInstruction = IDL.Record({
    'context' : IDL.Opt(InstructionContext),
    'instruction_text' : IDL.Text,
    'user_id' : IDL.Text,
    'preferences' : IDL.Opt(AgentPreferences),
    'subscription_tier' : SubscriptionTier,
  });
  const ModelPrecision = IDL.Variant({
    'FP16' : IDL.Null,
    'FP32' : IDL.Null,
    'INT4' : IDL.Null,
    'INT8' : IDL.Null,
    'Mixed' : IDL.Null,
  });
  const ReasoningLevel = IDL.Variant({
    'Advanced' : IDL.Null,
    'Basic' : IDL.Null,
    'Intermediate' : IDL.Null,
    'Expert' : IDL.Null,
  });
  const CreativityRequirement = IDL.Variant({
    'Low' : IDL.Null,
    'High' : IDL.Null,
    'Medium' : IDL.Null,
    'None' : IDL.Null,
  });
  const ModelRequirements = IDL.Record({
    'preferred_precision' : ModelPrecision,
    'minimum_context_length' : IDL.Nat32,
    'recommended_models' : IDL.Vec(IDL.Text),
    'specialized_requirements' : IDL.Vec(IDL.Text),
    'reasoning_capability' : ReasoningLevel,
    'creativity_requirement' : CreativityRequirement,
  });
  const AgentPersonality = IDL.Record({
    'helpfulness' : IDL.Float32,
    'efficiency' : IDL.Float32,
    'formality' : IDL.Float32,
    'assertiveness' : IDL.Float32,
    'thoroughness' : IDL.Float32,
    'creativity' : IDL.Float32,
  });
  const RetentionPolicy = IDL.Variant({
    'Weekly' : IDL.Null,
    'Session' : IDL.Null,
    'Daily' : IDL.Null,
    'Persistent' : IDL.Null,
  });
  const MemoryConfiguration = IDL.Record({
    'long_term_capacity' : IDL.Nat32,
    'retention_policy' : RetentionPolicy,
    'sharing_enabled' : IDL.Bool,
    'short_term_capacity' : IDL.Nat32,
  });
  const AgentType = IDL.Variant({
    'DataAnalyst' : IDL.Null,
    'ContentCreator' : IDL.Null,
    'ProblemSolver' : IDL.Null,
    'Planner' : IDL.Null,
    'CodeAssistant' : IDL.Null,
    'Custom' : IDL.Text,
    'Researcher' : IDL.Null,
    'Executor' : IDL.Null,
    'Coordinator' : IDL.Null,
    'GeneralAssistant' : IDL.Null,
  });
  const DecisionMakingStyle = IDL.Variant({
    'Aggressive' : IDL.Null,
    'Balanced' : IDL.Null,
    'Collaborative' : IDL.Null,
    'Conservative' : IDL.Null,
  });
  const CommunicationStyle = IDL.Variant({
    'Technical' : IDL.Null,
    'Conversational' : IDL.Null,
    'Professional' : IDL.Null,
    'Friendly' : IDL.Null,
    'Direct' : IDL.Null,
  });
  const AgentConfiguration = IDL.Record({
    'personality' : AgentPersonality,
    'memory_configuration' : MemoryConfiguration,
    'agent_type' : AgentType,
    'safety_constraints' : IDL.Vec(IDL.Text),
    'tool_access' : IDL.Vec(IDL.Text),
    'decision_making' : DecisionMakingStyle,
    'communication_style' : CommunicationStyle,
    'behavior_rules' : IDL.Vec(IDL.Text),
  });
  const CapabilityCategory = IDL.Variant({
    'DataAnalysis' : IDL.Null,
    'Research' : IDL.Null,
    'ContentCreation' : IDL.Null,
    'Communication' : IDL.Null,
    'Execution' : IDL.Null,
    'Custom' : IDL.Text,
    'ProblemSolving' : IDL.Null,
    'Planning' : IDL.Null,
    'TextGeneration' : IDL.Null,
    'Coordination' : IDL.Null,
    'CodeGeneration' : IDL.Null,
  });
  const CapabilityPriority = IDL.Variant({
    'Important' : IDL.Null,
    'Essential' : IDL.Null,
    'Helpful' : IDL.Null,
    'Optional' : IDL.Null,
  });
  const Capability = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'category' : CapabilityCategory,
    'priority' : CapabilityPriority,
    'required_tools' : IDL.Vec(IDL.Text),
    'estimated_tokens' : IDL.Nat32,
  });
  const CoordinationType = IDL.Variant({
    'Hierarchical' : IDL.Null,
    'None' : IDL.Null,
    'Parallel' : IDL.Null,
    'Collaborative' : IDL.Null,
    'Sequential' : IDL.Null,
  });
  const CommunicationProtocol = IDL.Variant({
    'Hierarchical' : IDL.Null,
    'Broadcast' : IDL.Null,
    'Centralized' : IDL.Null,
    'Direct' : IDL.Null,
  });
  const TaskDistributionStrategy = IDL.Variant({
    'PriorityBased' : IDL.Null,
    'CapabilityBased' : IDL.Null,
    'LoadBalanced' : IDL.Null,
    'RoundRobin' : IDL.Null,
  });
  const CoordinationRequirements = IDL.Record({
    'agent_count' : IDL.Nat32,
    'coordination_type' : CoordinationType,
    'communication_protocol' : CommunicationProtocol,
    'requires_coordination' : IDL.Bool,
    'task_distribution' : TaskDistributionStrategy,
  });
  const DurationEstimate = IDL.Record({
    'min_duration_seconds' : IDL.Nat64,
    'confidence' : IDL.Float32,
    'expected_duration_seconds' : IDL.Nat64,
    'max_duration_seconds' : IDL.Nat64,
  });
  const AnalyzedInstruction = IDL.Record({
    'model_requirements' : ModelRequirements,
    'agent_configuration' : AgentConfiguration,
    'estimated_complexity' : ComplexityLevel,
    'extracted_capabilities' : IDL.Vec(Capability),
    'coordination_requirements' : CoordinationRequirements,
    'confidence_score' : IDL.Float32,
    'estimated_duration' : DurationEstimate,
    'original_instruction' : UserInstruction,
  });
  const Result_Analyzed = IDL.Variant({
    'Ok' : AnalyzedInstruction,
    'Err' : IDL.Text,
  });
  const Result_Empty = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_Text = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const AgentCreationRequest = IDL.Record({
    'agent_count' : IDL.Opt(IDL.Nat32),
    'capabilities' : IDL.Opt(IDL.Vec(IDL.Text)),
    'priority' : IDL.Opt(IDL.Text),
    'instruction' : IDL.Text,
  });
  const AgentCreationResult = IDL.Record({
    'status' : IDL.Text,
    'capabilities' : IDL.Vec(IDL.Text),
    'estimated_completion' : IDL.Opt(IDL.Nat64),
    'agent_id' : IDL.Text,
  });
  const Result_AgentCreation = IDL.Variant({
    'Ok' : AgentCreationResult,
    'Err' : IDL.Text,
  });
  const Result_VecText = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Text),
    'Err' : IDL.Text,
  });
  const AgentTaskResult = IDL.Record({
    'result' : IDL.Text,
    'task_id' : IDL.Text,
    'error_message' : IDL.Opt(IDL.Text),
    'tokens_used' : IDL.Nat64,
    'execution_time_ms' : IDL.Nat64,
    'success' : IDL.Bool,
  });
  const Result_TaskResult = IDL.Variant({
    'Ok' : AgentTaskResult,
    'Err' : IDL.Text,
  });
  const NOVAQModelMeta = IDL.Record({
    'codebook_size_l1' : IDL.Nat32,
    'codebook_size_l2' : IDL.Nat32,
    'quality_score' : IDL.Float32,
    'bit_accuracy' : IDL.Float32,
    'parameter_count' : IDL.Nat32,
    'checksum' : IDL.Text,
    'num_subspaces' : IDL.Nat32,
    'target_bits' : IDL.Float32,
    'compression_ratio' : IDL.Float32,
  });
  const Result_NOVAQMeta = IDL.Variant({
    'Ok' : NOVAQModelMeta,
    'Err' : IDL.Text,
  });
  const AgentStatus = IDL.Variant({
    'Creating' : IDL.Null,
    'Error' : IDL.Text,
    'Paused' : IDL.Null,
    'Active' : IDL.Null,
    'Ready' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const AgentPerformanceMetrics = IDL.Record({
    'average_response_time_ms' : IDL.Float64,
    'total_tokens_used' : IDL.Nat64,
    'last_task_timestamp' : IDL.Nat64,
    'success_rate' : IDL.Float32,
    'tasks_completed' : IDL.Nat32,
  });
  const AgentStatusInfo = IDL.Record({
    'status' : AgentStatus,
    'performance_metrics' : AgentPerformanceMetrics,
    'created_at' : IDL.Nat64,
    'last_active' : IDL.Nat64,
    'agent_id' : IDL.Text,
    'model_bound' : IDL.Bool,
  });
  const Result_Status = IDL.Variant({
    'Ok' : AgentStatusInfo,
    'Err' : IDL.Text,
  });
  const AgentConfig = IDL.Record({
    'ttl_seconds' : IDL.Nat64,
    'warm_set_target' : IDL.Float32,
    'model_repo_canister_id' : IDL.Text,
    'concurrency_limit' : IDL.Nat32,
    'max_tokens' : IDL.Nat32,
    'prefetch_depth' : IDL.Nat32,
  });
  const Result_Config = IDL.Variant({ 'Ok' : AgentConfig, 'Err' : IDL.Text });
  const Result_float64 = IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : IDL.Text });
  const AgentHealth = IDL.Record({
    'cache_hit_rate' : IDL.Float32,
    'queue_depth' : IDL.Nat32,
    'model_bound' : IDL.Bool,
    'last_inference_timestamp' : IDL.Nat64,
    'warm_set_utilization' : IDL.Float32,
  });
  const DecodeParams = IDL.Record({
    'top_k' : IDL.Opt(IDL.Nat32),
    'top_p' : IDL.Opt(IDL.Float32),
    'temperature' : IDL.Opt(IDL.Float32),
    'max_tokens' : IDL.Opt(IDL.Nat32),
    'repetition_penalty' : IDL.Opt(IDL.Float32),
  });
  const InferenceRequest = IDL.Record({
    'msg_id' : IDL.Text,
    'seed' : IDL.Nat64,
    'prompt' : IDL.Text,
    'decode_params' : DecodeParams,
  });
  const InferenceResponse = IDL.Record({
    'inference_time_ms' : IDL.Nat64,
    'tokens' : IDL.Vec(IDL.Text),
    'cache_misses' : IDL.Nat32,
    'cache_hits' : IDL.Nat32,
    'generated_text' : IDL.Text,
  });
  const Result_Inference = IDL.Variant({
    'Ok' : InferenceResponse,
    'Err' : IDL.Text,
  });
  const AgentSummary = IDL.Record({
    'status' : AgentStatus,
    'created_at' : IDL.Nat64,
    'last_active' : IDL.Nat64,
    'agent_id' : IDL.Text,
    'agent_type' : AgentType,
  });
  const Result_Summaries = IDL.Variant({
    'Ok' : IDL.Vec(AgentSummary),
    'Err' : IDL.Text,
  });
  const Result_Nat32 = IDL.Variant({ 'Ok' : IDL.Nat32, 'Err' : IDL.Text });
  const NOVAQValidationResult = IDL.Record({
    'quality_score' : IDL.Float32,
    'bit_accuracy' : IDL.Float32,
    'issues' : IDL.Vec(IDL.Text),
    'passed' : IDL.Bool,
    'compression_ratio' : IDL.Float32,
  });
  const Result_NOVAQValidation = IDL.Variant({
    'Ok' : NOVAQValidationResult,
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'analyze_instruction' : IDL.Func([UserInstruction], [Result_Analyzed], []),
    'bind_model' : IDL.Func([IDL.Text], [Result_Empty], []),
    'clear_memory' : IDL.Func([], [Result_Empty], []),
    'create_agent' : IDL.Func([UserInstruction], [Result_Text], []),
    'create_agent_from_instruction' : IDL.Func(
        [AgentCreationRequest],
        [Result_AgentCreation],
        [],
      ),
    'create_coordinated_agents' : IDL.Func(
        [UserInstruction],
        [Result_VecText],
        [],
      ),
    'execute_agent_task' : IDL.Func(
        [IDL.Text, IDL.Text],
        [Result_TaskResult],
        [],
      ),
    'extract_novaq_metadata' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [Result_NOVAQMeta],
        ['query'],
      ),
    'get_agent_status' : IDL.Func([IDL.Text], [Result_Status], ['query']),
    'get_config' : IDL.Func([], [Result_Config], ['query']),
    'get_loader_stats' : IDL.Func([], [Result_Text], ['query']),
    'get_memory_stats' : IDL.Func([], [Result_Text], ['query']),
    'get_novaq_quality_score' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [Result_float64],
        ['query'],
      ),
    'health' : IDL.Func([], [AgentHealth], ['query']),
    'infer' : IDL.Func([InferenceRequest], [Result_Inference], []),
    'is_novaq_model' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Bool], ['query']),
    'list_user_agents' : IDL.Func([IDL.Text], [Result_Summaries], ['query']),
    'prefetch_next' : IDL.Func([IDL.Nat32], [Result_Nat32], []),
    'repo_canister' : IDL.Func([], [Result_Text], ['query']),
    'set_config' : IDL.Func([AgentConfig], [Result_Empty], []),
    'validate_novaq_model' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result_NOVAQValidation],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
