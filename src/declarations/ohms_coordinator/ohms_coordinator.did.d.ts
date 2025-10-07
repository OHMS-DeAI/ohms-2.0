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
export interface ArtifactChunkInfo {
  'sha256' : string,
  'size_bytes' : bigint,
  'offset' : bigint,
  'chunk_id' : string,
}
export type ComponentHealth = { 'Unhealthy' : null } |
  { 'Healthy' : null } |
  { 'Degraded' : null } |
  { 'Unknown' : null };
export interface CoordinationNetworkInfo {
  'status' : string,
  'network_id' : string,
  'created_at' : bigint,
  'last_activity' : bigint,
  'coordinator_agent' : string,
  'participant_count' : number,
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
export interface ModelManifest {
  'metadata' : Array<[string, string]>,
  'activated_at' : [] | [bigint],
  'quantization' : QuantizedArtifactMetadata,
  'total_size_bytes' : bigint,
  'version' : string,
  'state' : ModelState,
  'chunk_count' : number,
  'checksum' : string,
  'chunks' : Array<ArtifactChunkInfo>,
  'model_id' : string,
  'uploaded_at' : bigint,
}
export type ModelState = { 'Active' : null } |
  { 'Deprecated' : null } |
  { 'Pending' : null };
export type OrchestrationMode = { 'Adaptive' : null } |
  { 'Parallel' : null } |
  { 'Sequential' : null };
export type QuantizationFormat = { 'NOVAQ' : null } |
  { 'GGUF' : null } |
  { 'Custom' : string };
export interface QuantizedArtifactMetadata {
  'bits_per_weight' : [] | [number],
  'artifact_checksum' : string,
  'notes' : [] | [string],
  'accuracy_retention' : number,
  'compression_ratio' : number,
  'format' : QuantizationFormat,
}
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
export type Result_10 = { 'Ok' : Array<AgentRegistration> } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : Array<InstructionRequest> } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : RouteResponse } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : QuotaValidation } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : AgentCreationResult } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : AgentSpawningMetrics } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : Array<CoordinationNetworkInfo> } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : EconHealth } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : InstructionAnalysisResult } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<RoutingStats> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : SubscriptionTierInfo } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : QuotaCheckResult } |
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
  'capability_scores' : Array<[string, number]>,
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
export interface SystemHealth {
  'status' : ComponentHealth,
  'memory_usage_mb' : number,
  'metrics' : Array<[string, string]>,
  'canister_id' : Principal,
  'version' : string,
  'uptime_seconds' : bigint,
  'last_update' : bigint,
}
export interface _SERVICE {
  'create_agents_from_instructions' : ActorMethod<
    [string, [] | [number]],
    Result
  >,
  'get_agent' : ActorMethod<[string], Result_1>,
  'get_agent_creation_status' : ActorMethod<[string], Result_2>,
  'get_agent_spawning_metrics' : ActorMethod<[], Result_3>,
  'get_coordination_networks' : ActorMethod<[], Result_4>,
  'get_economics_health' : ActorMethod<[], Result_5>,
  'get_instruction_analysis' : ActorMethod<[string], Result_6>,
  'get_routing_stats' : ActorMethod<[[] | [string]], Result_7>,
  'get_subscription_tier_info' : ActorMethod<[], Result_8>,
  'get_swarm_policy' : ActorMethod<[], SwarmPolicy>,
  'get_user_quota_status' : ActorMethod<[], Result_9>,
  'health' : ActorMethod<[], SystemHealth>,
  'list_agents' : ActorMethod<[], Result_10>,
  'list_instruction_requests' : ActorMethod<[], Result_11>,
  'list_user_agents' : ActorMethod<[], Result_10>,
  'notify_model_deletion' : ActorMethod<[string], Result_12>,
  'notify_model_upload' : ActorMethod<[ModelManifest], Result_12>,
  'register_agent' : ActorMethod<[AgentRegistration], Result>,
  'route_best_result' : ActorMethod<[RouteRequest, number, bigint], Result_13>,
  'route_request' : ActorMethod<[RouteRequest], Result_13>,
  'set_swarm_policy' : ActorMethod<[SwarmPolicy], Result_12>,
  'update_agent_health' : ActorMethod<[string, number], Result_12>,
  'update_agent_status' : ActorMethod<[string, string], Result_12>,
  'upgrade_subscription_tier' : ActorMethod<[string], Result_12>,
  'validate_token_usage_quota' : ActorMethod<[bigint], Result_14>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
