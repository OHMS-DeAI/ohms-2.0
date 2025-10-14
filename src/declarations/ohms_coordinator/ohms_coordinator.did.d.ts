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
export interface IterationRecord {
  'queen_synthesis' : string,
  'queen_plan' : string,
  'quality_score' : number,
  'peer_communications' : Array<PeerMessage>,
  'timestamp' : bigint,
  'iteration_num' : number,
  'worker_executions' : Array<WorkerExecution>,
  'duration_ms' : bigint,
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
export interface OrchestrationTask {
  'status' : TaskStatus,
  'task_id' : string,
  'worker_agents' : Array<string>,
  'quality_threshold' : number,
  'error_message' : [] | [string],
  'quality_score' : number,
  'max_iterations' : number,
  'iterations' : Array<IterationRecord>,
  'created_at' : bigint,
  'instructions' : string,
  'user_id' : string,
  'completed_at' : [] | [bigint],
  'queen_agent_id' : [] | [string],
}
export interface PeerMessage {
  'to_agent' : string,
  'content' : string,
  'from_agent' : string,
  'timestamp' : bigint,
  'message_type' : PeerMessageType,
  'message_id' : string,
}
export type PeerMessageType = { 'Question' : null } |
  { 'Error' : null } |
  { 'Status' : null } |
  { 'Suggestion' : null } |
  { 'Answer' : null };
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
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_10 = { 'Ok' : Array<RoutingStats> } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : SubscriptionTierInfo } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : QuotaCheckResult } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : IterationRecord } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : Array<AgentRegistration> } |
  { 'Err' : string };
export type Result_15 = { 'Ok' : Array<InstructionRequest> } |
  { 'Err' : string };
export type Result_16 = { 'Ok' : Array<OrchestrationTask> } |
  { 'Err' : string };
export type Result_17 = { 'Ok' : RouteResponse } |
  { 'Err' : string };
export type Result_18 = { 'Ok' : QuotaValidation } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : OrchestrationTask } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : AgentRegistration } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : AgentCreationResult } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : AgentSpawningMetrics } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<CoordinationNetworkInfo> } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : EconHealth } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : InstructionAnalysisResult } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : TaskProgress } |
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
export interface TaskProgress {
  'status' : TaskStatus,
  'progress_percentage' : number,
  'active_workers' : number,
  'task_id' : string,
  'total_tokens_used' : number,
  'quality_threshold' : number,
  'estimated_completion_ms' : [] | [bigint],
  'quality_score' : number,
  'max_iterations' : number,
  'queen_agent' : [] | [string],
  'current_iteration' : number,
}
export type TaskStatus = { 'Failed' : null } |
  { 'Executing' : null } |
  { 'Reviewing' : null } |
  { 'Cancelled' : null } |
  { 'Planning' : null } |
  { 'Created' : null } |
  { 'Completed' : null };
export interface WorkerExecution {
  'result' : string,
  'error_message' : [] | [string],
  'tokens_used' : number,
  'agent_id' : string,
  'execution_time_ms' : bigint,
  'success' : boolean,
  'assigned_subtask' : string,
}
export interface _SERVICE {
  /**
   * Cancel orchestration task
   */
  'cancel_orchestration_task' : ActorMethod<[string], Result>,
  'create_agents_from_instructions' : ActorMethod<
    [string, [] | [number]],
    Result_1
  >,
  /**
   * Create orchestration task
   */
  'create_orchestration_task' : ActorMethod<[string], Result_2>,
  'get_agent' : ActorMethod<[string], Result_3>,
  'get_agent_creation_status' : ActorMethod<[string], Result_4>,
  'get_agent_spawning_metrics' : ActorMethod<[], Result_5>,
  'get_coordination_networks' : ActorMethod<[], Result_6>,
  'get_economics_health' : ActorMethod<[], Result_7>,
  'get_instruction_analysis' : ActorMethod<[string], Result_8>,
  /**
   * Get task progress
   */
  'get_orchestration_task_progress' : ActorMethod<[string], Result_9>,
  /**
   * Get task status
   */
  'get_orchestration_task_status' : ActorMethod<[string], Result_2>,
  'get_routing_stats' : ActorMethod<[[] | [string]], Result_10>,
  'get_subscription_tier_info' : ActorMethod<[], Result_11>,
  'get_swarm_policy' : ActorMethod<[], SwarmPolicy>,
  'get_user_quota_status' : ActorMethod<[], Result_12>,
  'health' : ActorMethod<[], SystemHealth>,
  /**
   * Execute one iteration of the task
   */
  'iterate_orchestration_task' : ActorMethod<[string], Result_13>,
  'list_agents' : ActorMethod<[], Result_14>,
  'list_instruction_requests' : ActorMethod<[], Result_15>,
  /**
   * List user's orchestration tasks
   */
  'list_orchestration_tasks' : ActorMethod<[], Result_16>,
  'list_user_agents' : ActorMethod<[], Result_14>,
  'notify_model_deletion' : ActorMethod<[string], Result>,
  'notify_model_upload' : ActorMethod<[ModelManifest], Result>,
  'register_agent' : ActorMethod<[AgentRegistration], Result_1>,
  'route_best_result' : ActorMethod<[RouteRequest, number, bigint], Result_17>,
  'route_request' : ActorMethod<[RouteRequest], Result_17>,
  'set_swarm_policy' : ActorMethod<[SwarmPolicy], Result>,
  'update_agent_health' : ActorMethod<[string, number], Result>,
  'update_agent_status' : ActorMethod<[string, string], Result>,
  'upgrade_subscription_tier' : ActorMethod<[string], Result>,
  'validate_token_usage_quota' : ActorMethod<[bigint], Result_18>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
