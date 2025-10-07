import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ArtifactChunkInfo {
  'sha256' : string,
  'size_bytes' : bigint,
  'offset' : bigint,
  'chunk_id' : string,
}
export interface ArtifactChunkUpload {
  'sha256' : string,
  'order' : number,
  'data' : Uint8Array | number[],
  'chunk_id' : string,
}
export type ComponentHealth = { 'Unhealthy' : null } |
  { 'Healthy' : null } |
  { 'Degraded' : null } |
  { 'Unknown' : null };
export interface InferenceRequest {
  'session_id' : [] | [string],
  'temperature' : [] | [number],
  'timeout_ms' : [] | [bigint],
  'max_tokens' : [] | [number],
  'model_id' : string,
  'input_data' : Uint8Array | number[],
}
export interface InferenceResponse {
  'tokens_processed' : number,
  'model_version' : string,
  'session_id' : string,
  'processing_time_ms' : bigint,
  'output_data' : Uint8Array | number[],
  'confidence_score' : [] | [number],
}
export interface ModelCanisterMetrics {
  'deleted_models' : number,
  'total_models' : number,
  'total_chunks' : bigint,
  'active_inference_sessions' : number,
  'last_updated' : bigint,
  'total_storage_used_bytes' : bigint,
  'ready_models' : number,
  'deployed_models' : number,
}
export interface ModelInfo {
  'quantization_format' : QuantizationFormat,
  'size_bytes' : bigint,
  'activated_at' : [] | [bigint],
  'version' : string,
  'state' : ModelState,
  'model_id' : string,
  'accuracy_retention' : [] | [number],
  'compression_ratio' : [] | [number],
  'uploaded_at' : bigint,
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
export type ModelStatus = { 'Uploading' : null } |
  { 'Failed' : null } |
  { 'Ready' : null } |
  { 'Deployed' : null } |
  { 'Deleted' : null };
export interface ModelUploadRequest {
  'metadata' : Array<[string, string]>,
  'model_type' : string,
  'name' : string,
  'quantization' : QuantizedArtifactMetadata,
  'description' : string,
  'version' : string,
  'chunks' : Array<ArtifactChunkUpload>,
}
export interface ModelUploadResponse {
  'total_size_bytes' : bigint,
  'upload_time' : bigint,
  'chunk_count' : number,
  'checksum' : string,
  'model_id' : string,
}
export type OHMSError = { 'InvalidInput' : string } |
  { 'NetworkError' : string } |
  { 'NotFound' : string } |
  { 'Unauthorized' : string } |
  { 'AlreadyExists' : string } |
  { 'CompressionFailed' : string } |
  { 'InternalError' : string } |
  { 'CommunicationFailed' : string } |
  { 'InvalidState' : string } |
  { 'QuotaExceeded' : string } |
  { 'InsufficientFunds' : string } |
  { 'ModelNotReady' : string };
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
export type Result = { 'Ok' : string } |
  { 'Err' : OHMSError };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : OHMSError };
export type Result_2 = { 'Ok' : ModelInfo } |
  { 'Err' : OHMSError };
export type Result_3 = { 'Ok' : ModelStatus } |
  { 'Err' : OHMSError };
export type Result_4 = { 'Ok' : InferenceResponse } |
  { 'Err' : OHMSError };
export type Result_5 = { 'Ok' : ModelUploadResponse } |
  { 'Err' : OHMSError };
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
  'create_inference_session' : ActorMethod<[string, Principal], Result>,
  'delete_model' : ActorMethod<[string], Result_1>,
  'deploy_model' : ActorMethod<[string], Result_1>,
  'get_canister_metrics' : ActorMethod<[], ModelCanisterMetrics>,
  'get_chunk' : ActorMethod<[string, string], [] | [Uint8Array | number[]]>,
  'get_manifest' : ActorMethod<[string], [] | [ModelManifest]>,
  'get_model_info' : ActorMethod<[string], Result_2>,
  'get_model_status' : ActorMethod<[string], Result_3>,
  'health_check' : ActorMethod<[], SystemHealth>,
  'list_active_models' : ActorMethod<[], Array<ModelInfo>>,
  'list_models' : ActorMethod<[[] | [Principal]], Array<ModelInfo>>,
  'run_inference' : ActorMethod<[InferenceRequest], Result_4>,
  'terminate_inference_session' : ActorMethod<[string], Result_1>,
  'upload_model' : ActorMethod<[ModelUploadRequest], Result_5>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
