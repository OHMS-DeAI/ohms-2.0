export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
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
  const Result_1 = IDL.Variant({ 'Ok' : AgentRegistration, 'Err' : IDL.Text });
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
  const Result_3 = IDL.Variant({
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
  const Result_10 = IDL.Variant({
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
  const Result_11 = IDL.Variant({
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
  const Result_13 = IDL.Variant({ 'Ok' : EconHealth, 'Err' : IDL.Text });
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
  const Result_9 = IDL.Variant({
    'Ok' : InstructionAnalysisResult,
    'Err' : IDL.Text,
  });
  const RoutingStats = IDL.Record({
    'average_response_time_ms' : IDL.Float64,
    'total_requests' : IDL.Nat64,
    'agent_id' : IDL.Text,
    'success_rate' : IDL.Float32,
  });
  const Result_7 = IDL.Variant({
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
  const Result_12 = IDL.Variant({
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
  const Result_4 = IDL.Variant({ 'Ok' : QuotaCheckResult, 'Err' : IDL.Text });
  const CoordinatorHealth = IDL.Record({
    'total_routes_processed' : IDL.Nat64,
    'total_agents' : IDL.Nat32,
    'active_agents' : IDL.Nat32,
    'dedup_cache_size' : IDL.Nat32,
    'active_instructions' : IDL.Nat32,
    'total_agent_creations' : IDL.Nat32,
    'average_routing_time_ms' : IDL.Float64,
  });
  const Result_5 = IDL.Variant({
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
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(InstructionRequest),
    'Err' : IDL.Text,
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
  const Result_2 = IDL.Variant({ 'Ok' : RouteResponse, 'Err' : IDL.Text });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
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
  const Result_14 = IDL.Variant({ 'Ok' : QuotaValidation, 'Err' : IDL.Text });
  return IDL.Service({
    'create_agents_from_instructions' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Nat32)],
        [Result],
        [],
      ),
    'get_agent' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'get_agent_creation_status' : IDL.Func([IDL.Text], [Result_3], ['query']),
    'get_agent_spawning_metrics' : IDL.Func([], [Result_10], ['query']),
    'get_coordination_networks' : IDL.Func([], [Result_11], ['query']),
    'get_economics_health' : IDL.Func([], [Result_13], []),
    'get_instruction_analysis' : IDL.Func([IDL.Text], [Result_9], ['query']),
    'get_routing_stats' : IDL.Func([IDL.Opt(IDL.Text)], [Result_7], ['query']),
    'get_subscription_tier_info' : IDL.Func([], [Result_12], ['query']),
    'get_swarm_policy' : IDL.Func([], [SwarmPolicy], ['query']),
    'get_user_quota_status' : IDL.Func([], [Result_4], []),
    'health' : IDL.Func([], [CoordinatorHealth], ['query']),
    'list_agents' : IDL.Func([], [Result_5], ['query']),
    'list_instruction_requests' : IDL.Func([], [Result_6], ['query']),
    'list_user_agents' : IDL.Func([], [Result_5], ['query']),
    'register_agent' : IDL.Func([AgentRegistration], [Result], []),
    'route_best_result' : IDL.Func(
        [RouteRequest, IDL.Nat32, IDL.Nat64],
        [Result_2],
        [],
      ),
    'route_request' : IDL.Func([RouteRequest], [Result_2], []),
    'set_swarm_policy' : IDL.Func([SwarmPolicy], [Result_8], []),
    'update_agent_health' : IDL.Func([IDL.Text, IDL.Float32], [Result_8], []),
    'update_agent_status' : IDL.Func([IDL.Text, IDL.Text], [Result_8], []),
    'upgrade_subscription_tier' : IDL.Func([IDL.Text], [Result_8], []),
    'validate_token_usage_quota' : IDL.Func([IDL.Nat64], [Result_14], []),
  });
};
export const init = ({ IDL }) => { return []; };
