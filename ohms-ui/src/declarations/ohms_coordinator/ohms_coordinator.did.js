export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const TaskStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Executing' : IDL.Null,
    'Reviewing' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Planning' : IDL.Null,
    'Created' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const PeerMessageType = IDL.Variant({
    'Question' : IDL.Null,
    'Error' : IDL.Null,
    'Status' : IDL.Null,
    'Suggestion' : IDL.Null,
    'Answer' : IDL.Null,
  });
  const PeerMessage = IDL.Record({
    'to_agent' : IDL.Text,
    'content' : IDL.Text,
    'from_agent' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'message_type' : PeerMessageType,
    'message_id' : IDL.Text,
  });
  const WorkerExecution = IDL.Record({
    'result' : IDL.Text,
    'error_message' : IDL.Opt(IDL.Text),
    'tokens_used' : IDL.Nat32,
    'agent_id' : IDL.Text,
    'execution_time_ms' : IDL.Nat64,
    'success' : IDL.Bool,
    'assigned_subtask' : IDL.Text,
  });
  const IterationRecord = IDL.Record({
    'queen_synthesis' : IDL.Text,
    'queen_plan' : IDL.Text,
    'quality_score' : IDL.Float32,
    'peer_communications' : IDL.Vec(PeerMessage),
    'timestamp' : IDL.Nat64,
    'iteration_num' : IDL.Nat32,
    'worker_executions' : IDL.Vec(WorkerExecution),
    'duration_ms' : IDL.Nat64,
  });
  const OrchestrationTask = IDL.Record({
    'status' : TaskStatus,
    'task_id' : IDL.Text,
    'worker_agents' : IDL.Vec(IDL.Text),
    'quality_threshold' : IDL.Float32,
    'error_message' : IDL.Opt(IDL.Text),
    'quality_score' : IDL.Float32,
    'max_iterations' : IDL.Nat32,
    'iterations' : IDL.Vec(IterationRecord),
    'created_at' : IDL.Nat64,
    'instructions' : IDL.Text,
    'user_id' : IDL.Text,
    'completed_at' : IDL.Opt(IDL.Nat64),
    'queen_agent_id' : IDL.Opt(IDL.Text),
  });
  const Result_2 = IDL.Variant({ 'Ok' : OrchestrationTask, 'Err' : IDL.Text });
  const AgentRegistration = IDL.Record({
    'capabilities' : IDL.Vec(IDL.Text),
    'canister_id' : IDL.Text,
    'agent_principal' : IDL.Text,
    'agent_id' : IDL.Text,
    'health_score' : IDL.Float32,
    'last_seen' : IDL.Nat64,
    'registered_at' : IDL.Nat64,
    'model_id' : IDL.Text,
  });
  const Result_3 = IDL.Variant({ 'Ok' : AgentRegistration, 'Err' : IDL.Text });
  const AgentCreationStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'InProgress' : IDL.Null,
    'Completed' : IDL.Null,
    'QuotaExceeded' : IDL.Null,
  });
  const AgentCreationResult = IDL.Record({
    'request_id' : IDL.Text,
    'status' : AgentCreationStatus,
    'creation_time_ms' : IDL.Nat64,
    'created_agents' : IDL.Vec(IDL.Text),
  });
  const Result_4 = IDL.Variant({
    'Ok' : AgentCreationResult,
    'Err' : IDL.Text,
  });
  const AgentSpawningMetrics = IDL.Record({
    'user_active_agents' : IDL.Nat32,
    'average_creation_time_ms' : IDL.Nat64,
    'user_agents_created' : IDL.Nat32,
    'success_rate' : IDL.Float32,
    'total_agent_creations' : IDL.Nat32,
    'total_instruction_requests' : IDL.Nat32,
  });
  const Result_5 = IDL.Variant({
    'Ok' : AgentSpawningMetrics,
    'Err' : IDL.Text,
  });
  const CoordinationNetworkInfo = IDL.Record({
    'status' : IDL.Text,
    'network_id' : IDL.Text,
    'created_at' : IDL.Nat64,
    'last_activity' : IDL.Nat64,
    'coordinator_agent' : IDL.Text,
    'participant_count' : IDL.Nat32,
  });
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(CoordinationNetworkInfo),
    'Err' : IDL.Text,
  });
  const EconHealth = IDL.Record({
    'active_escrows' : IDL.Nat32,
    'total_escrows' : IDL.Nat32,
    'total_receipts' : IDL.Nat32,
    'protocol_fees_collected' : IDL.Nat64,
    'total_volume' : IDL.Nat64,
    'average_job_cost' : IDL.Float64,
    'pending_settlements' : IDL.Nat32,
  });
  const Result_7 = IDL.Variant({ 'Ok' : EconHealth, 'Err' : IDL.Text });
  const QuotaCheckResult = IDL.Record({
    'tier' : IDL.Text,
    'quota_available' : IDL.Bool,
    'remaining_agents' : IDL.Nat32,
    'monthly_limit' : IDL.Nat32,
  });
  const AgentSpec = IDL.Record({
    'model_requirements' : IDL.Vec(IDL.Text),
    'required_capabilities' : IDL.Vec(IDL.Text),
    'agent_type' : IDL.Text,
    'specialization' : IDL.Text,
  });
  const InstructionAnalysisResult = IDL.Record({
    'request_id' : IDL.Text,
    'quota_check' : QuotaCheckResult,
    'suggested_agents' : IDL.Vec(AgentSpec),
    'parsed_requirements' : IDL.Vec(IDL.Text),
    'coordination_plan' : IDL.Text,
  });
  const Result_8 = IDL.Variant({
    'Ok' : InstructionAnalysisResult,
    'Err' : IDL.Text,
  });
  const TaskProgress = IDL.Record({
    'status' : TaskStatus,
    'progress_percentage' : IDL.Float32,
    'active_workers' : IDL.Nat32,
    'task_id' : IDL.Text,
    'total_tokens_used' : IDL.Nat32,
    'quality_threshold' : IDL.Float32,
    'estimated_completion_ms' : IDL.Opt(IDL.Nat64),
    'quality_score' : IDL.Float32,
    'max_iterations' : IDL.Nat32,
    'queen_agent' : IDL.Opt(IDL.Text),
    'current_iteration' : IDL.Nat32,
  });
  const Result_9 = IDL.Variant({ 'Ok' : TaskProgress, 'Err' : IDL.Text });
  const RoutingStats = IDL.Record({
    'average_response_time_ms' : IDL.Float64,
    'total_requests' : IDL.Nat64,
    'capability_scores' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float32)),
    'agent_id' : IDL.Text,
    'success_rate' : IDL.Float32,
  });
  const Result_10 = IDL.Variant({
    'Ok' : IDL.Vec(RoutingStats),
    'Err' : IDL.Text,
  });
  const SubscriptionTierInfo = IDL.Record({
    'max_agents' : IDL.Nat32,
    'tokens_used_this_month' : IDL.Nat64,
    'current_tier' : IDL.Text,
    'agents_created_this_month' : IDL.Nat32,
    'inference_rate' : IDL.Text,
    'token_limit' : IDL.Nat64,
    'last_reset_date' : IDL.Nat64,
    'monthly_creations' : IDL.Nat32,
  });
  const Result_11 = IDL.Variant({
    'Ok' : SubscriptionTierInfo,
    'Err' : IDL.Text,
  });
  const OrchestrationMode = IDL.Variant({
    'Adaptive' : IDL.Null,
    'Parallel' : IDL.Null,
    'Sequential' : IDL.Null,
  });
  const SwarmTopology = IDL.Variant({
    'Hierarchical' : IDL.Null,
    'Mesh' : IDL.Null,
    'Ring' : IDL.Null,
    'Star' : IDL.Null,
  });
  const SwarmPolicy = IDL.Record({
    'top_k' : IDL.Nat32,
    'mode' : OrchestrationMode,
    'window_ms' : IDL.Nat64,
    'topology' : SwarmTopology,
  });
  const ComponentHealth = IDL.Variant({
    'Unhealthy' : IDL.Null,
    'Healthy' : IDL.Null,
    'Degraded' : IDL.Null,
    'Unknown' : IDL.Null,
  });
  const SystemHealth = IDL.Record({
    'status' : ComponentHealth,
    'memory_usage_mb' : IDL.Float32,
    'metrics' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'canister_id' : IDL.Principal,
    'version' : IDL.Text,
    'uptime_seconds' : IDL.Nat64,
    'last_update' : IDL.Nat64,
  });
  const Result_12 = IDL.Variant({ 'Ok' : QuotaCheckResult, 'Err' : IDL.Text });
  const Result_13 = IDL.Variant({ 'Ok' : IterationRecord, 'Err' : IDL.Text });
  const Result_14 = IDL.Variant({
    'Ok' : IDL.Vec(AgentRegistration),
    'Err' : IDL.Text,
  });
  const InstructionRequest = IDL.Record({
    'request_id' : IDL.Text,
    'user_principal' : IDL.Text,
    'agent_count' : IDL.Opt(IDL.Nat32),
    'model_preferences' : IDL.Vec(IDL.Text),
    'created_at' : IDL.Nat64,
    'instructions' : IDL.Text,
  });
  const Result_15 = IDL.Variant({
    'Ok' : IDL.Vec(InstructionRequest),
    'Err' : IDL.Text,
  });
  const Result_16 = IDL.Variant({
    'Ok' : IDL.Vec(OrchestrationTask),
    'Err' : IDL.Text,
  });
  const QuantizationFormat = IDL.Variant({
    'NOVAQ' : IDL.Null,
    'GGUF' : IDL.Null,
    'Custom' : IDL.Text,
  });
  const QuantizedArtifactMetadata = IDL.Record({
    'bits_per_weight' : IDL.Opt(IDL.Float32),
    'artifact_checksum' : IDL.Text,
    'notes' : IDL.Opt(IDL.Text),
    'accuracy_retention' : IDL.Float32,
    'compression_ratio' : IDL.Float32,
    'format' : QuantizationFormat,
  });
  const ModelState = IDL.Variant({
    'Active' : IDL.Null,
    'Deprecated' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ArtifactChunkInfo = IDL.Record({
    'sha256' : IDL.Text,
    'size_bytes' : IDL.Nat64,
    'offset' : IDL.Nat64,
    'chunk_id' : IDL.Text,
  });
  const ModelManifest = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'activated_at' : IDL.Opt(IDL.Nat64),
    'quantization' : QuantizedArtifactMetadata,
    'total_size_bytes' : IDL.Nat64,
    'version' : IDL.Text,
    'state' : ModelState,
    'chunk_count' : IDL.Nat32,
    'checksum' : IDL.Text,
    'chunks' : IDL.Vec(ArtifactChunkInfo),
    'model_id' : IDL.Text,
    'uploaded_at' : IDL.Nat64,
  });
  const RoutingMode = IDL.Variant({
    'Unicast' : IDL.Null,
    'Broadcast' : IDL.Null,
    'AgentSpawning' : IDL.Null,
  });
  const RouteRequest = IDL.Record({
    'request_id' : IDL.Text,
    'requester' : IDL.Text,
    'routing_mode' : RoutingMode,
    'capabilities_required' : IDL.Vec(IDL.Text),
    'payload' : IDL.Vec(IDL.Nat8),
  });
  const RouteResponse = IDL.Record({
    'request_id' : IDL.Text,
    'selection_criteria' : IDL.Text,
    'selected_agents' : IDL.Vec(IDL.Text),
    'routing_time_ms' : IDL.Nat64,
  });
  const Result_17 = IDL.Variant({ 'Ok' : RouteResponse, 'Err' : IDL.Text });
  const QuotaRemaining = IDL.Record({
    'inferences_remaining' : IDL.Nat32,
    'agents_remaining' : IDL.Nat32,
    'tokens_remaining' : IDL.Nat64,
  });
  const QuotaValidation = IDL.Record({
    'allowed' : IDL.Bool,
    'remaining_quota' : IDL.Opt(QuotaRemaining),
    'reason' : IDL.Opt(IDL.Text),
  });
  const Result_18 = IDL.Variant({ 'Ok' : QuotaValidation, 'Err' : IDL.Text });
  return IDL.Service({
    'cancel_orchestration_task' : IDL.Func([IDL.Text], [Result], []),
    'create_agents_from_instructions' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Nat32)],
        [Result_1],
        [],
      ),
    'create_orchestration_task' : IDL.Func([IDL.Text], [Result_2], []),
    'get_agent' : IDL.Func([IDL.Text], [Result_3], ['query']),
    'get_agent_creation_status' : IDL.Func([IDL.Text], [Result_4], ['query']),
    'get_agent_spawning_metrics' : IDL.Func([], [Result_5], ['query']),
    'get_coordination_networks' : IDL.Func([], [Result_6], ['query']),
    'get_economics_health' : IDL.Func([], [Result_7], []),
    'get_instruction_analysis' : IDL.Func([IDL.Text], [Result_8], ['query']),
    'get_orchestration_task_progress' : IDL.Func(
        [IDL.Text],
        [Result_9],
        ['query'],
      ),
    'get_orchestration_task_status' : IDL.Func(
        [IDL.Text],
        [Result_2],
        ['query'],
      ),
    'get_routing_stats' : IDL.Func([IDL.Opt(IDL.Text)], [Result_10], ['query']),
    'get_subscription_tier_info' : IDL.Func([], [Result_11], ['query']),
    'get_swarm_policy' : IDL.Func([], [SwarmPolicy], ['query']),
    'get_system_health' : IDL.Func([], [SystemHealth], ['query']),
    'get_user_quota_status' : IDL.Func([], [Result_12], []),
    'health' : IDL.Func([], [SystemHealth], ['query']),
    'iterate_orchestration_task' : IDL.Func([IDL.Text], [Result_13], []),
    'list_agents' : IDL.Func([], [Result_14], ['query']),
    'list_instruction_requests' : IDL.Func([], [Result_15], ['query']),
    'list_orchestration_tasks' : IDL.Func([], [Result_16], ['query']),
    'list_user_agents' : IDL.Func([], [Result_14], ['query']),
    'notify_model_deletion' : IDL.Func([IDL.Text], [Result], []),
    'notify_model_upload' : IDL.Func([ModelManifest], [Result], []),
    'register_agent' : IDL.Func([AgentRegistration], [Result_1], []),
    'route_best_result' : IDL.Func(
        [RouteRequest, IDL.Nat32, IDL.Nat64],
        [Result_17],
        [],
      ),
    'route_request' : IDL.Func([RouteRequest], [Result_17], []),
    'set_swarm_policy' : IDL.Func([SwarmPolicy], [Result], []),
    'update_agent_health' : IDL.Func([IDL.Text, IDL.Float32], [Result], []),
    'update_agent_status' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'upgrade_subscription_tier' : IDL.Func([IDL.Text], [Result], []),
    'validate_token_usage_quota' : IDL.Func([IDL.Nat64], [Result_18], []),
  });
};
export const init = ({ IDL }) => { return []; };
