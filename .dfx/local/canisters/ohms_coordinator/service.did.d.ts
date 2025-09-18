import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AgentCreationResult {
  'request_id' : string,
  'status' : AgentCreationStatus,
  'creation_time_ms' : bigint,
  'created_agents' : Array<string>,
}
export type AgentCreationStatus = { 'Failed' : null } |
  { 'InProgress' : null } |
  { 'Completed' : null } |
  { 'QuotaExceeded' : null };
export interface AgentRegistration {
  'capabilities' : Array<string>,
  'canister_id' : string,
  'agent_principal' : string,
  'agent_id' : string,
  'health_score' : number,
  'last_seen' : bigint,
  'registered_at' : bigint,
  'model_id' : string,
}
export interface AgentSpawningMetrics {
  'user_active_agents' : number,
  'average_creation_time_ms' : bigint,
  'user_agents_created' : number,
  'success_rate' : number,
  'total_agent_creations' : number,
  'total_instruction_requests' : number,
}
export interface AgentSpec {
  'model_requirements' : Array<string>,
  'required_capabilities' : Array<string>,
  'agent_type' : string,
  'specialization' : string,
}
export interface CoordinationNetworkInfo {
  'status' : string,
  'network_id' : string,
  'created_at' : bigint,
  'last_activity' : bigint,
  'coordinator_agent' : string,
  'participant_count' : number,
}
export interface CoordinatorHealth {
  'total_routes_processed' : bigint,
  'total_agents' : number,
  'active_agents' : number,
  'dedup_cache_size' : number,
  'active_instructions' : number,
  'total_agent_creations' : number,
  'average_routing_time_ms' : number,
}
export interface EconHealth {
  'active_escrows' : number,
  'total_escrows' : number,
  'total_receipts' : number,
  'protocol_fees_collected' : bigint,
  'total_volume' : bigint,
  'average_job_cost' : number,
  'pending_settlements' : number,
}
export interface InstructionAnalysisResult {
  'request_id' : string,
  'quota_check' : QuotaCheckResult,
  'suggested_agents' : Array<AgentSpec>,
  'parsed_requirements' : Array<string>,
  'coordination_plan' : string,
}
export interface InstructionRequest {
  'request_id' : string,
  'user_principal' : string,
  'agent_count' : [] | [number],
  'model_preferences' : Array<string>,
  'created_at' : bigint,
  'instructions' : string,
}
export type OrchestrationMode = { 'Adaptive' : null } |
  { 'Parallel' : null } |
  { 'Sequential' : null };
export interface QuotaCheckResult {
  'tier' : string,
  'quota_available' : boolean,
  'remaining_agents' : number,
  'monthly_limit' : number,
}
export interface QuotaRemaining {
  'inferences_remaining' : number,
  'agents_remaining' : number,
  'tokens_remaining' : bigint,
}
export interface QuotaValidation {
  'allowed' : boolean,
  'remaining_quota' : [] | [QuotaRemaining],
  'reason' : [] | [string],
}
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : AgentRegistration } |
  { 'Err' : string };
export type Result_10 = { 'Ok' : AgentSpawningMetrics } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : Array<CoordinationNetworkInfo> } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : SubscriptionTierInfo } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : EconHealth } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : QuotaValidation } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : RouteResponse } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : AgentCreationResult } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : QuotaCheckResult } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<AgentRegistration> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<InstructionRequest> } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<RoutingStats> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : InstructionAnalysisResult } |
  { 'Err' : string };
export interface RouteRequest {
  'request_id' : string,
  'requester' : string,
  'routing_mode' : RoutingMode,
  'capabilities_required' : Array<string>,
  'payload' : Uint8Array | number[],
}
export interface RouteResponse {
  'request_id' : string,
  'selection_criteria' : string,
  'selected_agents' : Array<string>,
  'routing_time_ms' : bigint,
}
export type RoutingMode = { 'Unicast' : null } |
  { 'Broadcast' : null } |
  { 'AgentSpawning' : null };
export interface RoutingStats {
  'average_response_time_ms' : number,
  'total_requests' : bigint,
  'agent_id' : string,
  'success_rate' : number,
}
export interface SubscriptionTierInfo {
  'max_agents' : number,
  'tokens_used_this_month' : bigint,
  'current_tier' : string,
  'agents_created_this_month' : number,
  'inference_rate' : string,
  'token_limit' : bigint,
  'last_reset_date' : bigint,
  'monthly_creations' : number,
}
export interface SwarmPolicy {
  'top_k' : number,
  'mode' : OrchestrationMode,
  'window_ms' : bigint,
  'topology' : SwarmTopology,
}
export type SwarmTopology = { 'Hierarchical' : null } |
  { 'Mesh' : null } |
  { 'Ring' : null } |
  { 'Star' : null };
export interface _SERVICE {
  /**
   * OHMS 2.0: Instruction-based agent creation
   */
  'create_agents_from_instructions' : ActorMethod<
    [string, [] | [number]],
    Result
  >,
  'get_agent' : ActorMethod<[string], Result_1>,
  'get_agent_creation_status' : ActorMethod<[string], Result_3>,
  /**
   * OHMS 2.0: Agent spawning metrics and coordination
   */
  'get_agent_spawning_metrics' : ActorMethod<[], Result_10>,
  'get_coordination_networks' : ActorMethod<[], Result_11>,
  'get_economics_health' : ActorMethod<[], Result_13>,
  'get_instruction_analysis' : ActorMethod<[string], Result_9>,
  'get_routing_stats' : ActorMethod<[[] | [string]], Result_7>,
  'get_subscription_tier_info' : ActorMethod<[], Result_12>,
  'get_swarm_policy' : ActorMethod<[], SwarmPolicy>,
  /**
   * Quota and subscription management
   */
  'get_user_quota_status' : ActorMethod<[], Result_4>,
  /**
   * System management
   */
  'health' : ActorMethod<[], CoordinatorHealth>,
  'list_agents' : ActorMethod<[], Result_5>,
  'list_instruction_requests' : ActorMethod<[], Result_6>,
  'list_user_agents' : ActorMethod<[], Result_5>,
  /**
   * Agent management
   */
  'register_agent' : ActorMethod<[AgentRegistration], Result>,
  'route_best_result' : ActorMethod<[RouteRequest, number, bigint], Result_2>,
  /**
   * Routing and coordination
   */
  'route_request' : ActorMethod<[RouteRequest], Result_2>,
  'set_swarm_policy' : ActorMethod<[SwarmPolicy], Result_8>,
  'update_agent_health' : ActorMethod<[string, number], Result_8>,
  'update_agent_status' : ActorMethod<[string, string], Result_8>,
  'upgrade_subscription_tier' : ActorMethod<[string], Result_8>,
  'validate_token_usage_quota' : ActorMethod<[bigint], Result_14>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
