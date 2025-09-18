import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AgentConfig {
  'ttl_seconds' : bigint,
  'warm_set_target' : number,
  'model_repo_canister_id' : string,
  'concurrency_limit' : number,
  'max_tokens' : number,
  'prefetch_depth' : number,
}
export interface AgentConfiguration {
  'personality' : AgentPersonality,
  'memory_configuration' : MemoryConfiguration,
  'agent_type' : AgentType,
  'safety_constraints' : Array<string>,
  'tool_access' : Array<string>,
  'decision_making' : DecisionMakingStyle,
  'communication_style' : CommunicationStyle,
  'behavior_rules' : Array<string>,
}
export interface AgentCreationRequest {
  'agent_count' : [] | [number],
  'capabilities' : [] | [Array<string>],
  'priority' : [] | [string],
  'instruction' : string,
}
export interface AgentCreationResult {
  'status' : string,
  'capabilities' : Array<string>,
  'estimated_completion' : [] | [bigint],
  'agent_id' : string,
}
export interface AgentHealth {
  'cache_hit_rate' : number,
  'queue_depth' : number,
  'model_bound' : boolean,
  'last_inference_timestamp' : bigint,
  'warm_set_utilization' : number,
}
export interface AgentPerformanceMetrics {
  'average_response_time_ms' : number,
  'total_tokens_used' : bigint,
  'last_task_timestamp' : bigint,
  'success_rate' : number,
  'tasks_completed' : number,
}
export interface AgentPersonality {
  'helpfulness' : number,
  'efficiency' : number,
  'formality' : number,
  'assertiveness' : number,
  'thoroughness' : number,
  'creativity' : number,
}
export interface AgentPreferences {
  'detail_level' : DetailLevel,
  'safety_level' : SafetyLevel,
  'creativity_level' : CreativityLevel,
  'language' : string,
  'response_style' : ResponseStyle,
}
export type AgentStatus = { 'Creating' : null } |
  { 'Error' : string } |
  { 'Paused' : null } |
  { 'Active' : null } |
  { 'Ready' : null } |
  { 'Completed' : null };
export interface AgentStatusInfo {
  'status' : AgentStatus,
  'performance_metrics' : AgentPerformanceMetrics,
  'created_at' : bigint,
  'last_active' : bigint,
  'agent_id' : string,
  'model_bound' : boolean,
}
export interface AgentSummary {
  'status' : AgentStatus,
  'created_at' : bigint,
  'last_active' : bigint,
  'agent_id' : string,
  'agent_type' : AgentType,
}
export interface AgentTask {
  'context' : Array<[string, string]>,
  'task_id' : string,
  'description' : string,
  'deadline' : [] | [bigint],
  'priority' : TaskPriority,
}
export interface AgentTaskResult {
  'result' : string,
  'task_id' : string,
  'error_message' : [] | [string],
  'tokens_used' : bigint,
  'execution_time_ms' : bigint,
  'success' : boolean,
}
export type AgentType = { 'DataAnalyst' : null } |
  { 'ContentCreator' : null } |
  { 'ProblemSolver' : null } |
  { 'Planner' : null } |
  { 'CodeAssistant' : null } |
  { 'Custom' : string } |
  { 'Researcher' : null } |
  { 'Executor' : null } |
  { 'Coordinator' : null } |
  { 'GeneralAssistant' : null };
export interface AnalyzedInstruction {
  'model_requirements' : ModelRequirements,
  'agent_configuration' : AgentConfiguration,
  'estimated_complexity' : ComplexityLevel,
  'extracted_capabilities' : Array<Capability>,
  'coordination_requirements' : CoordinationRequirements,
  'confidence_score' : number,
  'estimated_duration' : DurationEstimate,
  'original_instruction' : UserInstruction,
}
export interface Capability {
  'name' : string,
  'description' : string,
  'category' : CapabilityCategory,
  'priority' : CapabilityPriority,
  'required_tools' : Array<string>,
  'estimated_tokens' : number,
}
export type CapabilityCategory = { 'DataAnalysis' : null } |
  { 'Research' : null } |
  { 'ContentCreation' : null } |
  { 'Communication' : null } |
  { 'Execution' : null } |
  { 'Custom' : string } |
  { 'ProblemSolving' : null } |
  { 'Planning' : null } |
  { 'TextGeneration' : null } |
  { 'Coordination' : null } |
  { 'CodeGeneration' : null };
export type CapabilityPriority = { 'Important' : null } |
  { 'Essential' : null } |
  { 'Helpful' : null } |
  { 'Optional' : null };
export type CommunicationProtocol = { 'Hierarchical' : null } |
  { 'Broadcast' : null } |
  { 'Centralized' : null } |
  { 'Direct' : null };
export type CommunicationStyle = { 'Technical' : null } |
  { 'Conversational' : null } |
  { 'Professional' : null } |
  { 'Friendly' : null } |
  { 'Direct' : null };
export type ComplexityLevel = { 'Complex' : null } |
  { 'Moderate' : null } |
  { 'Simple' : null } |
  { 'Expert' : null };
export interface CoordinationRequirements {
  'agent_count' : number,
  'coordination_type' : CoordinationType,
  'communication_protocol' : CommunicationProtocol,
  'requires_coordination' : boolean,
  'task_distribution' : TaskDistributionStrategy,
}
export type CoordinationType = { 'Hierarchical' : null } |
  { 'None' : null } |
  { 'Parallel' : null } |
  { 'Collaborative' : null } |
  { 'Sequential' : null };
export type CreativityLevel = { 'Creative' : null } |
  { 'Experimental' : null } |
  { 'Balanced' : null } |
  { 'Conservative' : null };
export type CreativityRequirement = { 'Low' : null } |
  { 'High' : null } |
  { 'Medium' : null } |
  { 'None' : null };
export type DecisionMakingStyle = { 'Aggressive' : null } |
  { 'Balanced' : null } |
  { 'Collaborative' : null } |
  { 'Conservative' : null };
export interface DecodeParams {
  'top_k' : [] | [number],
  'top_p' : [] | [number],
  'temperature' : [] | [number],
  'max_tokens' : [] | [number],
  'repetition_penalty' : [] | [number],
}
export type DetailLevel = { 'Comprehensive' : null } |
  { 'Summary' : null } |
  { 'Standard' : null } |
  { 'Expert' : null };
export interface DurationEstimate {
  'min_duration_seconds' : bigint,
  'confidence' : number,
  'expected_duration_seconds' : bigint,
  'max_duration_seconds' : bigint,
}
export interface InferenceRequest {
  'msg_id' : string,
  'seed' : bigint,
  'prompt' : string,
  'decode_params' : DecodeParams,
}
export interface InferenceResponse {
  'inference_time_ms' : bigint,
  'tokens' : Array<string>,
  'cache_misses' : number,
  'cache_hits' : number,
  'generated_text' : string,
}
export interface InstructionContext {
  'complexity' : [] | [ComplexityLevel],
  'urgency' : [] | [UrgencyLevel],
  'domain' : [] | [string],
  'external_tools_required' : Array<string>,
  'collaboration_needed' : boolean,
}
export interface MemoryConfiguration {
  'long_term_capacity' : number,
  'retention_policy' : RetentionPolicy,
  'sharing_enabled' : boolean,
  'short_term_capacity' : number,
}
export type ModelPrecision = { 'FP16' : null } |
  { 'FP32' : null } |
  { 'INT4' : null } |
  { 'INT8' : null } |
  { 'Mixed' : null };
export interface ModelRequirements {
  'preferred_precision' : ModelPrecision,
  'minimum_context_length' : number,
  'recommended_models' : Array<string>,
  'specialized_requirements' : Array<string>,
  'reasoning_capability' : ReasoningLevel,
  'creativity_requirement' : CreativityRequirement,
}
export interface NOVAQModelMeta {
  'codebook_size_l1' : number,
  'codebook_size_l2' : number,
  'quality_score' : number,
  'bit_accuracy' : number,
  'parameter_count' : number,
  'checksum' : string,
  'num_subspaces' : number,
  'target_bits' : number,
  'compression_ratio' : number,
}
export interface NOVAQValidationResult {
  'quality_score' : number,
  'bit_accuracy' : number,
  'issues' : Array<string>,
  'passed' : boolean,
  'compression_ratio' : number,
}
export type ReasoningLevel = { 'Advanced' : null } |
  { 'Basic' : null } |
  { 'Intermediate' : null } |
  { 'Expert' : null };
export type ResponseStyle = { 'Technical' : null } |
  { 'Detailed' : null } |
  { 'Conversational' : null } |
  { 'Concise' : null };
export type Result_AgentCreation = { 'Ok' : AgentCreationResult } |
  { 'Err' : string };
export type Result_Analyzed = { 'Ok' : AnalyzedInstruction } |
  { 'Err' : string };
export type Result_Config = { 'Ok' : AgentConfig } |
  { 'Err' : string };
export type Result_Empty = { 'Ok' : null } |
  { 'Err' : string };
export type Result_Inference = { 'Ok' : InferenceResponse } |
  { 'Err' : string };
export type Result_NOVAQMeta = { 'Ok' : NOVAQModelMeta } |
  { 'Err' : string };
export type Result_NOVAQValidation = { 'Ok' : NOVAQValidationResult } |
  { 'Err' : string };
export type Result_Nat32 = { 'Ok' : number } |
  { 'Err' : string };
export type Result_Status = { 'Ok' : AgentStatusInfo } |
  { 'Err' : string };
export type Result_Summaries = { 'Ok' : Array<AgentSummary> } |
  { 'Err' : string };
export type Result_TaskResult = { 'Ok' : AgentTaskResult } |
  { 'Err' : string };
export type Result_Text = { 'Ok' : string } |
  { 'Err' : string };
export type Result_VecText = { 'Ok' : Array<string> } |
  { 'Err' : string };
export type Result_float64 = { 'Ok' : number } |
  { 'Err' : string };
export type RetentionPolicy = { 'Weekly' : null } |
  { 'Session' : null } |
  { 'Daily' : null } |
  { 'Persistent' : null };
export type SafetyLevel = { 'Experimental' : null } |
  { 'Strict' : null } |
  { 'Standard' : null } |
  { 'Flexible' : null };
/**
 * Instruction Analysis and Agent Factory Types
 */
export type SubscriptionTier = { 'Pro' : null } |
  { 'Enterprise' : null } |
  { 'Basic' : null };
export type TaskDistributionStrategy = { 'PriorityBased' : null } |
  { 'CapabilityBased' : null } |
  { 'LoadBalanced' : null } |
  { 'RoundRobin' : null };
export type TaskPriority = { 'Low' : null } |
  { 'High' : null } |
  { 'Normal' : null } |
  { 'Critical' : null };
export type UrgencyLevel = { 'Low' : null } |
  { 'High' : null } |
  { 'Normal' : null } |
  { 'Critical' : null };
export interface UserInstruction {
  'context' : [] | [InstructionContext],
  'instruction_text' : string,
  'user_id' : string,
  'preferences' : [] | [AgentPreferences],
  'subscription_tier' : SubscriptionTier,
}
export interface _SERVICE {
  'analyze_instruction' : ActorMethod<[UserInstruction], Result_Analyzed>,
  'bind_model' : ActorMethod<[string], Result_Empty>,
  'clear_memory' : ActorMethod<[], Result_Empty>,
  'create_agent' : ActorMethod<[UserInstruction], Result_Text>,
  'create_agent_from_instruction' : ActorMethod<
    [AgentCreationRequest],
    Result_AgentCreation
  >,
  'create_coordinated_agents' : ActorMethod<[UserInstruction], Result_VecText>,
  'execute_agent_task' : ActorMethod<[string, string], Result_TaskResult>,
  'extract_novaq_metadata' : ActorMethod<
    [Uint8Array | number[]],
    Result_NOVAQMeta
  >,
  'get_agent_status' : ActorMethod<[string], Result_Status>,
  'get_config' : ActorMethod<[], Result_Config>,
  'get_loader_stats' : ActorMethod<[], Result_Text>,
  'get_memory_stats' : ActorMethod<[], Result_Text>,
  'get_novaq_quality_score' : ActorMethod<
    [Uint8Array | number[]],
    Result_float64
  >,
  'health' : ActorMethod<[], AgentHealth>,
  'infer' : ActorMethod<[InferenceRequest], Result_Inference>,
  'is_novaq_model' : ActorMethod<[Uint8Array | number[]], boolean>,
  'list_user_agents' : ActorMethod<[string], Result_Summaries>,
  'prefetch_next' : ActorMethod<[number], Result_Nat32>,
  'repo_canister' : ActorMethod<[], Result_Text>,
  'set_config' : ActorMethod<[AgentConfig], Result_Empty>,
  'validate_novaq_model' : ActorMethod<
    [string, Uint8Array | number[]],
    Result_NOVAQValidation
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
