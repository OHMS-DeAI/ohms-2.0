export const idlFactory = ({ IDL }) => {
  const OHMSError = IDL.Variant({
    'InvalidInput' : IDL.Text,
    'NetworkError' : IDL.Text,
    'NotFound' : IDL.Text,
    'Unauthorized' : IDL.Text,
    'AlreadyExists' : IDL.Text,
    'CompressionFailed' : IDL.Text,
    'InternalError' : IDL.Text,
    'CommunicationFailed' : IDL.Text,
    'InvalidState' : IDL.Text,
    'QuotaExceeded' : IDL.Text,
    'InsufficientFunds' : IDL.Text,
    'ModelNotReady' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : OHMSError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : OHMSError });
  const ModelCanisterMetrics = IDL.Record({
    'deleted_models' : IDL.Nat32,
    'total_models' : IDL.Nat32,
    'total_chunks' : IDL.Nat64,
    'active_inference_sessions' : IDL.Nat32,
    'last_updated' : IDL.Nat64,
    'total_storage_used_bytes' : IDL.Nat64,
    'ready_models' : IDL.Nat32,
    'deployed_models' : IDL.Nat32,
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
  const ModelInfo = IDL.Record({
    'quantization_format' : QuantizationFormat,
    'size_bytes' : IDL.Nat64,
    'activated_at' : IDL.Opt(IDL.Nat64),
    'version' : IDL.Text,
    'state' : ModelState,
    'model_id' : IDL.Text,
    'accuracy_retention' : IDL.Opt(IDL.Float32),
    'compression_ratio' : IDL.Opt(IDL.Float32),
    'uploaded_at' : IDL.Nat64,
  });
  const Result_2 = IDL.Variant({ 'Ok' : ModelInfo, 'Err' : OHMSError });
  const ModelStatus = IDL.Variant({
    'Uploading' : IDL.Null,
    'Failed' : IDL.Null,
    'Ready' : IDL.Null,
    'Deployed' : IDL.Null,
    'Deleted' : IDL.Null,
  });
  const Result_3 = IDL.Variant({ 'Ok' : ModelStatus, 'Err' : OHMSError });
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
  const InferenceRequest = IDL.Record({
    'session_id' : IDL.Opt(IDL.Text),
    'temperature' : IDL.Opt(IDL.Float32),
    'timeout_ms' : IDL.Opt(IDL.Nat64),
    'max_tokens' : IDL.Opt(IDL.Nat32),
    'model_id' : IDL.Text,
    'input_data' : IDL.Vec(IDL.Nat8),
  });
  const InferenceResponse = IDL.Record({
    'tokens_processed' : IDL.Nat32,
    'model_version' : IDL.Text,
    'session_id' : IDL.Text,
    'processing_time_ms' : IDL.Nat64,
    'output_data' : IDL.Vec(IDL.Nat8),
    'confidence_score' : IDL.Opt(IDL.Float32),
  });
  const Result_4 = IDL.Variant({ 'Ok' : InferenceResponse, 'Err' : OHMSError });
  const ArtifactChunkUpload = IDL.Record({
    'sha256' : IDL.Text,
    'order' : IDL.Nat32,
    'data' : IDL.Vec(IDL.Nat8),
    'chunk_id' : IDL.Text,
  });
  const ModelUploadRequest = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'model_type' : IDL.Text,
    'name' : IDL.Text,
    'quantization' : QuantizedArtifactMetadata,
    'description' : IDL.Text,
    'version' : IDL.Text,
    'chunks' : IDL.Vec(ArtifactChunkUpload),
  });
  const ModelUploadResponse = IDL.Record({
    'total_size_bytes' : IDL.Nat64,
    'upload_time' : IDL.Nat64,
    'chunk_count' : IDL.Nat32,
    'checksum' : IDL.Text,
    'model_id' : IDL.Text,
  });
  const Result_5 = IDL.Variant({
    'Ok' : ModelUploadResponse,
    'Err' : OHMSError,
  });
  return IDL.Service({
    'create_inference_session' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result],
        [],
      ),
    'delete_model' : IDL.Func([IDL.Text], [Result_1], []),
    'deploy_model' : IDL.Func([IDL.Text], [Result_1], []),
    'get_canister_metrics' : IDL.Func([], [ModelCanisterMetrics], ['query']),
    'get_chunk' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(IDL.Vec(IDL.Nat8))],
        ['query'],
      ),
    'get_manifest' : IDL.Func([IDL.Text], [IDL.Opt(ModelManifest)], ['query']),
    'get_model_info' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_model_status' : IDL.Func([IDL.Text], [Result_3], ['query']),
    'health_check' : IDL.Func([], [SystemHealth], ['query']),
    'list_active_models' : IDL.Func([], [IDL.Vec(ModelInfo)], ['query']),
    'list_models' : IDL.Func(
        [IDL.Opt(IDL.Principal)],
        [IDL.Vec(ModelInfo)],
        ['query'],
      ),
    'run_inference' : IDL.Func([InferenceRequest], [Result_4], []),
    'terminate_inference_session' : IDL.Func([IDL.Text], [Result_1], []),
    'upload_model' : IDL.Func([ModelUploadRequest], [Result_5], []),
  });
};
export const init = ({ IDL }) => { return []; };
