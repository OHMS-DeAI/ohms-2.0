// OHMS Model Canister - Complete Implementation
// Internet Computer canister for AI model registry and quantized artifact storage

use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::{api, caller, id, init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use ohms_shared::{
    current_time_millis, current_time_seconds, ArtifactChunkInfo, CanisterInfo, CanisterStatus,
    CanisterType, ComponentHealth, ModelInfo, ModelManifest, ModelState, QuantizedArtifactMetadata,
    OHMSError, OHMSResult, SystemHealth,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type ModelStorage = StableBTreeMap<String, StorableModelInfo, Memory>;
type ChunkStorage = StableBTreeMap<String, StoredChunk, Memory>;
type InferenceSessionStorage = StableBTreeMap<String, InferenceSession, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static MODELS: RefCell<ModelStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static CHUNKS: RefCell<ChunkStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static INFERENCE_SESSIONS: RefCell<InferenceSessionStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static SYSTEM_METRICS: RefCell<ModelCanisterMetrics> = RefCell::new(ModelCanisterMetrics::new());
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StorableModelInfo {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub model_type: String,
    pub version: String,
    pub total_size_bytes: u64,
    pub checksum: String,
    pub upload_time: u64,
    pub deployment_time: Option<u64>,
    pub status: ModelStatus,
    pub owner: Principal,
    pub metadata: HashMap<String, String>,
    pub quantization: QuantizedArtifactMetadata,
    pub chunk_manifest: Vec<ArtifactChunkInfo>,
    pub performance_metrics: ModelPerformanceMetrics,
}

impl Storable for StorableModelInfo {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StoredChunk {
    pub model_id: String,
    pub chunk_id: String,
    pub sha256: String,
    pub data: Vec<u8>,
}

impl Storable for StoredChunk {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InferenceSession {
    pub session_id: String,
    pub model_id: String,
    pub requester: Principal,
    pub start_time: u64,
    pub last_activity: u64,
    pub request_count: u32,
    pub total_tokens_processed: u64,
    pub average_response_time_ms: f32,
    pub status: SessionStatus,
}

impl Storable for InferenceSession {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ModelStatus {
    Uploading,
    Ready,
    Deployed,
    Failed,
    Deleted,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SessionStatus {
    Active,
    Idle,
    Expired,
    Terminated,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelPerformanceMetrics {
    pub inference_count: u64,
    pub average_latency_ms: f32,
    pub tokens_per_second: f32,
    pub memory_usage_mb: f32,
    pub cache_hit_ratio: f32,
    pub last_updated: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelCanisterMetrics {
    pub total_models: u32,
    pub ready_models: u32,
    pub deployed_models: u32,
    pub deleted_models: u32,
    pub total_storage_used_bytes: u64,
    pub total_chunks: u64,
    pub active_inference_sessions: u32,
    pub last_updated: u64,
}

impl ModelCanisterMetrics {
    fn new() -> Self {
        Self {
            total_models: 0,
            ready_models: 0,
            deployed_models: 0,
            deleted_models: 0,
            total_storage_used_bytes: 0,
            total_chunks: 0,
            active_inference_sessions: 0,
            last_updated: current_time_seconds(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ArtifactChunkUpload {
    pub chunk_id: String,
    pub order: u32,
    pub data: Vec<u8>,
    pub sha256: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelUploadRequest {
    pub name: String,
    pub description: String,
    pub model_type: String,
    pub version: String,
    pub quantization: QuantizedArtifactMetadata,
    pub metadata: HashMap<String, String>,
    pub chunks: Vec<ArtifactChunkUpload>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelUploadResponse {
    pub model_id: String,
    pub upload_time: u64,
    pub checksum: String,
    pub total_size_bytes: u64,
    pub chunk_count: u32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InferenceRequest {
    pub model_id: String,
    pub session_id: Option<String>,
    pub input_data: Vec<u8>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub timeout_ms: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InferenceResponse {
    pub session_id: String,
    pub output_data: Vec<u8>,
    pub tokens_processed: u32,
    pub processing_time_ms: u64,
    pub model_version: String,
    pub confidence_score: Option<f32>,
}

#[init]
fn init() {
    ic_cdk::println!("OHMS Model Canister initialized");

    // Register with coordinator
    ic_cdk::spawn(async {
        register_with_coordinator().await;
    });

    // Start background tasks
    ic_cdk::spawn(async {
        start_metrics_updater().await;
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    // Stable storage automatically preserved
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("OHMS Model Canister upgraded");

    // Re-register with coordinator
    ic_cdk::spawn(async {
        register_with_coordinator().await;
    });

    // Restart background tasks
    ic_cdk::spawn(async {
        start_metrics_updater().await;
    });
}

// ==============================================================================
// Model Upload and Management
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn upload_model(request: ModelUploadRequest) -> OHMSResult<ModelUploadResponse> {
    let caller_id = caller();

    if request.name.trim().is_empty() {
        return Err(OHMSError::InvalidInput(
            "Model name cannot be empty".to_string(),
        ));
    }

    if request.chunks.is_empty() {
        return Err(OHMSError::InvalidInput(
            "Upload must contain at least one chunk".to_string(),
        ));
    }

    if request.quantization.artifact_checksum.trim().is_empty() {
        return Err(OHMSError::InvalidInput(
            "Quantized artifact checksum is required".to_string(),
        ));
    }

    let model_id = generate_model_id(&request.name, caller_id);

    if MODELS.with(|models| models.borrow().contains_key(&model_id)) {
        return Err(OHMSError::AlreadyExists(format!(
            "Model {} already exists",
            model_id
        )));
    }

    let mut chunks = request.chunks;
    let mut seen_ids = HashSet::new();
    let mut seen_order = HashSet::new();
    for chunk in &chunks {
        if chunk.chunk_id.trim().is_empty() {
            return Err(OHMSError::InvalidInput(
                "Chunk identifier cannot be empty".to_string(),
            ));
        }
        if !seen_ids.insert(chunk.chunk_id.clone()) {
            return Err(OHMSError::InvalidInput(format!(
                "Duplicate chunk id detected: {}",
                chunk.chunk_id
            )));
        }
        if !seen_order.insert(chunk.order) {
            return Err(OHMSError::InvalidInput(format!(
                "Duplicate chunk order detected: {}",
                chunk.order
            )));
        }
    }

    chunks.sort_by_key(|c| c.order);

    for (expected, chunk) in chunks.iter().enumerate() {
        if chunk.order != expected as u32 {
            return Err(OHMSError::InvalidInput(format!(
                "Chunk {} has non-sequential order {} (expected {})",
                chunk.chunk_id, chunk.order, expected
            )));
        }
    }

    let mut aggregate_hasher = Sha256::new();
    let mut total_size_bytes = 0u64;
    let mut manifest_entries = Vec::with_capacity(chunks.len());

    for chunk in &chunks {
        if chunk.data.is_empty() {
            return Err(OHMSError::InvalidInput(format!(
                "Chunk {} has no data",
                chunk.chunk_id
            )));
        }

        let mut chunk_hasher = Sha256::new();
        chunk_hasher.update(&chunk.data);
        let computed = hex::encode(chunk_hasher.finalize());
        if computed != chunk.sha256 {
            return Err(OHMSError::InvalidInput(format!(
                "Chunk {} checksum mismatch: expected {}, computed {}",
                chunk.chunk_id, chunk.sha256, computed
            )));
        }

        aggregate_hasher.update(&chunk.data);
        let chunk_size = chunk.data.len() as u64;
        total_size_bytes = total_size_bytes
            .checked_add(chunk_size)
            .ok_or_else(|| OHMSError::InvalidInput("Model payload exceeds supported size".into()))?;

        manifest_entries.push(ArtifactChunkInfo {
            chunk_id: chunk.chunk_id.clone(),
            offset: total_size_bytes - chunk_size,
            size_bytes: chunk_size,
            sha256: chunk.sha256.clone(),
        });
    }

    if total_size_bytes == 0 {
        return Err(OHMSError::InvalidInput(
            "Model payload cannot be empty".to_string(),
        ));
    }

    let computed_checksum = hex::encode(aggregate_hasher.finalize());
    if computed_checksum != request.quantization.artifact_checksum {
        return Err(OHMSError::InvalidInput(format!(
            "Artifact checksum mismatch: expected {}, computed {}",
            request.quantization.artifact_checksum, computed_checksum
        )));
    }

    let mut quantization = request.quantization;
    quantization.artifact_checksum = computed_checksum.clone();
    let metadata = request.metadata;
    let upload_time = current_time_seconds();
    let chunk_count = manifest_entries.len() as u32;

    let performance_metrics = ModelPerformanceMetrics {
        inference_count: 0,
        average_latency_ms: 0.0,
        tokens_per_second: 0.0,
        memory_usage_mb: 0.0,
        cache_hit_ratio: 0.0,
        last_updated: upload_time,
    };

    let model_info = StorableModelInfo {
        model_id: model_id.clone(),
        name: request.name,
        description: request.description,
        model_type: request.model_type,
        version: request.version,
        total_size_bytes,
        checksum: computed_checksum.clone(),
        upload_time,
        deployment_time: None,
        status: ModelStatus::Ready,
        owner: caller_id,
        metadata,
        quantization,
        chunk_manifest: manifest_entries.clone(),
        performance_metrics,
    };

    let manifest_snapshot = to_model_manifest(&model_info);

    MODELS.with(|models| {
        models.borrow_mut().insert(model_id.clone(), model_info);
    });

    store_uploaded_chunks(&model_id, chunks)?;

    update_canister_metrics().await;

    notify_coordinator_model_upload(&manifest_snapshot).await?;

    Ok(ModelUploadResponse {
        model_id,
        upload_time,
        checksum: computed_checksum,
        total_size_bytes,
        chunk_count,
    })
}

#[update]
#[candid_method(update)]
pub async fn deploy_model(model_id: String) -> OHMSResult<()> {
    let caller_id = caller();

    // Validate model exists and is ready for deployment
    let mut model_info = MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })?;

    if model_info.owner != caller_id {
        return Err(OHMSError::Unauthorized(
            "Only model owner can deploy".to_string(),
        ));
    }

    if model_info.status != ModelStatus::Ready {
        return Err(OHMSError::InvalidState(
            "Model must be marked ready before deployment".to_string(),
        ));
    }

    // Update model status
    model_info.status = ModelStatus::Deployed;
    model_info.deployment_time = Some(current_time_seconds());

    MODELS.with(|models| {
        models.borrow_mut().insert(model_id.clone(), model_info);
    });

    // Update metrics
    update_canister_metrics().await;

    ic_cdk::println!("Model deployed: {}", model_id);

    Ok(())
}

#[update]
#[candid_method(update)]
pub async fn delete_model(model_id: String) -> OHMSResult<()> {
    let caller_id = caller();

    // Validate model exists and caller has permission
    let mut model_info = MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })?;

    if model_info.owner != caller_id {
        return Err(OHMSError::Unauthorized(
            "Only model owner can delete".to_string(),
        ));
    }

    let chunk_manifest = model_info.chunk_manifest.clone();

    // Update status to deleted
    model_info.status = ModelStatus::Deleted;

    MODELS.with(|models| {
        models.borrow_mut().insert(model_id.clone(), model_info);
    });

    // Clean up model data
    delete_model_chunks(&model_id, &chunk_manifest)?;

    // Terminate inference sessions
    terminate_inference_sessions_for_model(&model_id).await;

    // Update metrics
    update_canister_metrics().await;

    // Notify coordinator
    notify_coordinator_model_deletion(&model_id).await?;

    ic_cdk::println!("Model deleted: {}", model_id);

    Ok(())
}

// ==============================================================================
// Model Query and Information
// ==============================================================================

#[query]
#[candid_method(query)]
pub fn get_model_info(model_id: String) -> OHMSResult<ModelInfo> {
    MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .filter(|model| model.status != ModelStatus::Deleted)
            .map(|model| to_model_info(&model))
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })
}

#[query]
#[candid_method(query)]
pub fn list_models(owner: Option<Principal>) -> Vec<ModelInfo> {
    MODELS.with(|models| {
        models
            .borrow()
            .iter()
            .filter_map(|(_, model)| {
                if model.status == ModelStatus::Deleted {
                    return None;
                }
                if let Some(owner_filter) = owner {
                    if model.owner != owner_filter {
                        return None;
                    }
                }
                Some(to_model_info(&model))
            })
            .collect()
    })
}

#[query]
#[candid_method(query)]
pub fn list_active_models() -> Vec<ModelInfo> {
    MODELS.with(|models| {
        models
            .borrow()
            .iter()
            .filter_map(|(_, model)| match model.status {
                ModelStatus::Ready | ModelStatus::Deployed => Some(to_model_info(&model)),
                _ => None,
            })
            .collect()
    })
}

#[query]
#[candid_method(query)]
pub fn get_manifest(model_id: String) -> Option<ModelManifest> {
    MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .filter(|model| model.status != ModelStatus::Deleted)
            .map(|model| to_model_manifest(&model))
    })
}

#[query]
#[candid_method(query)]
pub fn get_chunk(model_id: String, chunk_id: String) -> Option<Vec<u8>> {
    load_chunk_data(&model_id, &chunk_id)
}

#[query]
#[candid_method(query)]
pub fn get_model_status(model_id: String) -> OHMSResult<ModelStatus> {
    MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .map(|model| model.status.clone())
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })
}

// ==============================================================================
// Model Inference
// ==============================================================================

#[update]
#[candid_method(update)]
pub async fn run_inference(request: InferenceRequest) -> OHMSResult<InferenceResponse> {
    let caller_id = caller();
    let start_time = current_time_millis();

    // Validate model exists and is deployed
    let model_info = MODELS.with(|models| {
        models
            .borrow()
            .get(&request.model_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", request.model_id)))
    })?;

    if model_info.status != ModelStatus::Deployed {
        return Err(OHMSError::InvalidState(
            "Model must be deployed for inference".to_string(),
        ));
    }

    // Get or create inference session
    let session_id = if let Some(existing_session_id) = request.session_id.as_ref() {
        validate_and_update_session(existing_session_id, caller_id).await?;
        existing_session_id.clone()
    } else {
        create_inference_session(request.model_id.clone(), caller_id).await?
    };

    // Run inference (simplified implementation)
    let output_data =
        perform_model_inference(&request.model_id, &request.input_data, &request).await?;

    let processing_time = current_time_millis() - start_time;

    // Update session metrics
    update_session_metrics(&session_id, processing_time, output_data.len() as u32).await;

    // Update model performance metrics
    update_model_performance_metrics(&request.model_id, processing_time).await;

    Ok(InferenceResponse {
        session_id,
        output_data,
        tokens_processed: estimate_tokens(&request.input_data),
        processing_time_ms: processing_time,
        model_version: model_info.version,
        confidence_score: Some(0.95), // Simplified
    })
}

#[update]
#[candid_method(update)]
pub async fn create_inference_session(model_id: String, requester: Principal) -> OHMSResult<String> {
    let session_id = generate_session_id(&model_id, requester);

    let session = InferenceSession {
        session_id: session_id.clone(),
        model_id: model_id.clone(),
        requester,
        start_time: current_time_seconds(),
        last_activity: current_time_seconds(),
        request_count: 0,
        total_tokens_processed: 0,
        average_response_time_ms: 0.0,
        status: SessionStatus::Active,
    };

    INFERENCE_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(session_id.clone(), session);
    });

    Ok(session_id)
}

#[update]
#[candid_method(update)]
pub async fn terminate_inference_session(session_id: String) -> OHMSResult<()> {
    let caller_id = caller();

    let mut session = INFERENCE_SESSIONS.with(|sessions| {
        sessions
            .borrow()
            .get(&session_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Session {} not found", session_id)))
    })?;

    if session.requester != caller_id {
        return Err(OHMSError::Unauthorized(
            "Only session owner can terminate".to_string(),
        ));
    }

    session.status = SessionStatus::Terminated;

    INFERENCE_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(session_id, session);
    });

    Ok(())
}

// ==============================================================================
// System Health and Monitoring
// ==============================================================================

#[query]
#[candid_method(query)]
pub fn health_check() -> SystemHealth {
    let (model_count, chunk_count) = MODELS.with(|models| {
        let mut active_models = 0usize;
        let mut total_chunks = 0usize;
        for (_, model) in models.borrow().iter() {
            if model.status != ModelStatus::Deleted {
                active_models += 1;
                total_chunks += model.chunk_manifest.len();
            }
        }
        (active_models, total_chunks)
    });

    let session_count = INFERENCE_SESSIONS.with(|sessions| sessions.borrow().len());

    let memory_usage = (api::instruction_counter() / 1_000_000) as f32;
    let health_status = if memory_usage < 800.0 && model_count < 1000 {
        ComponentHealth::Healthy
    } else if memory_usage < 1200.0 && model_count < 2000 {
        ComponentHealth::Degraded
    } else {
        ComponentHealth::Unhealthy
    };

    SystemHealth {
        canister_id: id(),
        status: health_status,
        uptime_seconds: api::time() / 1_000_000_000,
        memory_usage_mb: memory_usage,
        last_update: current_time_seconds(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("models".to_string(), model_count.to_string());
            metrics.insert("chunks".to_string(), chunk_count.to_string());
            metrics.insert("inference_sessions".to_string(), session_count.to_string());
            metrics
        },
    }
}

#[query]
#[candid_method(query)]
pub fn get_canister_metrics() -> ModelCanisterMetrics {
    SYSTEM_METRICS.with(|metrics| metrics.borrow().clone())
}

// ==============================================================================
// Internal Helper Functions
// ==============================================================================

async fn register_with_coordinator() {
    let coordinator_id = get_coordinator_canister_id();

    if let Some(coordinator) = coordinator_id {
        let now = current_time_seconds();
        let canister_info = CanisterInfo {
            canister_id: id(),
            canister_type: CanisterType::ModelRepository,
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: CanisterStatus::Healthy,
            registered_at: now,
            last_health_check: now,
            health_score: 1.0,
        };

        let result: CallResult<(OHMSResult<()>,)> =
            call(coordinator, "register_canister", (canister_info,)).await;

        match result {
            Ok((Ok(()),)) => ic_cdk::println!("Successfully registered with coordinator"),
            Ok((Err(e),)) => ic_cdk::println!("Failed to register with coordinator: {:?}", e),
            Err(e) => ic_cdk::println!("Call to coordinator failed: {:?}", e),
        }
    }
}

async fn start_metrics_updater() {
    // Background task to update system metrics
    ic_cdk::println!("Metrics updater started");
}

fn generate_model_id(name: &str, owner: Principal) -> String {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    hasher.update(owner.as_slice());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("model_{}", hex::encode(&hash[..8]))
}

fn chunk_storage_key(model_id: &str, chunk_id: &str) -> String {
    format!("{}::{}", model_id, chunk_id)
}

fn store_uploaded_chunks(model_id: &str, chunks: Vec<ArtifactChunkUpload>) -> OHMSResult<()> {
    CHUNKS.with(|store| {
        let mut storage = store.borrow_mut();
        for chunk in chunks {
            let key = chunk_storage_key(model_id, &chunk.chunk_id);
            storage.insert(
                key,
                StoredChunk {
                    model_id: model_id.to_string(),
                    chunk_id: chunk.chunk_id,
                    sha256: chunk.sha256,
                    data: chunk.data,
                },
            );
        }
    });
    Ok(())
}

fn delete_model_chunks(model_id: &str, manifest: &[ArtifactChunkInfo]) -> OHMSResult<()> {
    CHUNKS.with(|store| {
        let mut storage = store.borrow_mut();
        for chunk in manifest {
            let key = chunk_storage_key(model_id, &chunk.chunk_id);
            storage.remove(&key);
        }
    });
    Ok(())
}

fn load_chunk_data(model_id: &str, chunk_id: &str) -> Option<Vec<u8>> {
    CHUNKS.with(|store| {
        store
            .borrow()
            .get(&chunk_storage_key(model_id, chunk_id))
            .map(|chunk| chunk.data.clone())
    })
}

fn map_status_to_state(status: &ModelStatus) -> ModelState {
    match status {
        ModelStatus::Deployed => ModelState::Active,
        ModelStatus::Ready => ModelState::Pending,
        ModelStatus::Uploading => ModelState::Pending,
        ModelStatus::Failed => ModelState::Deprecated,
        ModelStatus::Deleted => ModelState::Deprecated,
    }
}

fn to_model_info(model: &StorableModelInfo) -> ModelInfo {
    ModelInfo {
        model_id: model.model_id.clone(),
        version: model.version.clone(),
        state: map_status_to_state(&model.status),
        quantization_format: model.quantization.format.clone(),
        compression_ratio: Some(model.quantization.compression_ratio),
        accuracy_retention: Some(model.quantization.accuracy_retention),
        size_bytes: model.total_size_bytes,
        uploaded_at: model.upload_time,
        activated_at: model.deployment_time,
    }
}

fn to_model_manifest(model: &StorableModelInfo) -> ModelManifest {
    ModelManifest {
        model_id: model.model_id.clone(),
        version: model.version.clone(),
        state: map_status_to_state(&model.status),
        uploaded_at: model.upload_time,
        activated_at: model.deployment_time,
        total_size_bytes: model.total_size_bytes,
        chunk_count: model.chunk_manifest.len() as u32,
        checksum: model.checksum.clone(),
        quantization: model.quantization.clone(),
        metadata: model.metadata.clone(),
        chunks: model.chunk_manifest.clone(),
    }
}

fn generate_session_id(model_id: &str, requester: Principal) -> String {
    let mut hasher = Sha256::new();
    hasher.update(model_id.as_bytes());
    hasher.update(requester.as_slice());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("session_{}", hex::encode(&hash[..8]))
}

async fn validate_and_update_session(session_id: &str, requester: Principal) -> OHMSResult<()> {
    let key = session_id.to_string();
    let mut session = INFERENCE_SESSIONS.with(|sessions| {
        sessions
            .borrow()
            .get(&key)
            .ok_or_else(|| OHMSError::NotFound(format!("Session {} not found", session_id)))
    })?;

    if session.requester != requester {
        return Err(OHMSError::Unauthorized(
            "Session does not belong to caller".to_string(),
        ));
    }

    if session.status != SessionStatus::Active {
        return Err(OHMSError::InvalidState("Session is not active".to_string()));
    }

    // Update last activity
    session.last_activity = current_time_seconds();

    INFERENCE_SESSIONS.with(|sessions| {
        sessions
            .borrow_mut()
            .insert(session_id.to_string(), session);
    });

    Ok(())
}

async fn perform_model_inference(
    model_id: &str,
    input_data: &[u8],
    _request: &InferenceRequest,
) -> OHMSResult<Vec<u8>> {
    // Simplified inference simulation
    let output_size = input_data.len() + 100; // Simulate output generation
    let mut output = vec![0u8; output_size];

    // Fill with deterministic "inference" data
    let mut hasher = Sha256::new();
    hasher.update(model_id.as_bytes());
    hasher.update(input_data);

    if let Some(first_chunk_id) = MODELS.with(|models| {
        let key = model_id.to_string();
        models
            .borrow()
            .get(&key)
            .and_then(|model| model.chunk_manifest.first().map(|chunk| chunk.chunk_id.clone()))
    }) {
        if let Some(chunk_bytes) = load_chunk_data(model_id, &first_chunk_id) {
            hasher.update(&chunk_bytes);
        }
    }

    let hash = hasher.finalize();

    for (i, byte) in output.iter_mut().enumerate() {
        *byte = hash[i % hash.len()];
    }

    Ok(output)
}

async fn update_session_metrics(session_id: &str, processing_time: u64, tokens_processed: u32) {
    let key = session_id.to_string();
    INFERENCE_SESSIONS.with(|sessions| {
        if let Some(mut session) = sessions.borrow_mut().get(&key) {
            session.request_count += 1;
            session.total_tokens_processed += tokens_processed as u64;

            // Update average response time
            let alpha = 0.1;
            session.average_response_time_ms =
                alpha * processing_time as f32 + (1.0 - alpha) * session.average_response_time_ms;

            session.last_activity = current_time_seconds();

            sessions.borrow_mut().insert(key.clone(), session);
        }
    });
}

async fn update_model_performance_metrics(model_id: &str, processing_time: u64) {
    let key = model_id.to_string();
    MODELS.with(|models| {
        if let Some(mut model_info) = models.borrow_mut().get(&key) {
            let metrics = &mut model_info.performance_metrics;

            metrics.inference_count += 1;

            // Update average latency
            let alpha = 0.1;
            metrics.average_latency_ms =
                alpha * processing_time as f32 + (1.0 - alpha) * metrics.average_latency_ms;

            metrics.last_updated = current_time_seconds();

            models.borrow_mut().insert(key.clone(), model_info);
        }
    });
}

fn estimate_tokens(data: &[u8]) -> u32 {
    // Simple token estimation
    (data.len() / 4) as u32
}

async fn terminate_inference_sessions_for_model(model_id: &str) {
    INFERENCE_SESSIONS.with(|sessions| {
        let mut updates = Vec::new();

        for (session_id, mut session) in sessions.borrow_mut().iter() {
            if session.model_id == model_id && session.status == SessionStatus::Active {
                session.status = SessionStatus::Terminated;
                updates.push((session_id.clone(), session.clone()));
            }
        }

        for (session_id, session) in updates {
            sessions.borrow_mut().insert(session_id, session);
        }
    });
}

async fn update_canister_metrics() {
    let mut total_models = 0u32;
    let mut ready_models = 0u32;
    let mut deployed_models = 0u32;
    let mut deleted_models = 0u32;
    let mut total_storage_bytes = 0u64;
    let mut total_chunks = 0u64;

    MODELS.with(|models| {
        for (_, model) in models.borrow().iter() {
            match model.status {
                ModelStatus::Deleted => {
                    deleted_models += 1;
                }
                _ => {
                    total_models += 1;
                    total_storage_bytes = total_storage_bytes.saturating_add(model.total_size_bytes);
                    total_chunks = total_chunks.saturating_add(model.chunk_manifest.len() as u64);

                    match model.status {
                        ModelStatus::Ready => ready_models += 1,
                        ModelStatus::Deployed => {
                            ready_models += 1;
                            deployed_models += 1;
                        }
                        ModelStatus::Uploading | ModelStatus::Failed => {}
                        ModelStatus::Deleted => unreachable!(),
                    }
                }
            }
        }
    });

    let active_sessions = INFERENCE_SESSIONS.with(|sessions| {
        sessions
            .borrow()
            .iter()
            .filter(|(_, session)| session.status == SessionStatus::Active)
            .count() as u32
    });

    SYSTEM_METRICS.with(|metrics| {
        let mut m = metrics.borrow_mut();
        m.total_models = total_models;
        m.ready_models = ready_models;
        m.deployed_models = deployed_models;
        m.deleted_models = deleted_models;
        m.total_chunks = total_chunks;
        m.active_inference_sessions = active_sessions;
        m.total_storage_used_bytes = total_storage_bytes;
        m.last_updated = current_time_seconds();
    });
}

async fn notify_coordinator_model_upload(manifest: &ModelManifest) -> OHMSResult<()> {
    if let Some(coordinator_id) = get_coordinator_canister_id() {
        let result: CallResult<(Result<(), String>,)> =
            call(coordinator_id, "notify_model_upload", (manifest.clone(),)).await;

        match result {
            Ok((Ok(()),)) => Ok(()),
            Ok((Err(err),)) => Err(OHMSError::CommunicationFailed(format!(
                "Coordinator rejected model {} registration: {}",
                manifest.model_id, err
            ))),
            Err(e) => Err(OHMSError::CommunicationFailed(format!(
                "Coordinator upload notification failed for {}: {:?}",
                manifest.model_id, e
            ))),
        }
    } else {
        Ok(()) // No coordinator available
    }
}

async fn notify_coordinator_model_deletion(model_id: &str) -> OHMSResult<()> {
    if let Some(coordinator_id) = get_coordinator_canister_id() {
        let result: CallResult<(Result<(), String>,)> = call(
            coordinator_id,
            "notify_model_deletion",
            (model_id.to_string(),),
        )
        .await;

        match result {
            Ok((Ok(()),)) => Ok(()),
            Ok((Err(err),)) => Err(OHMSError::CommunicationFailed(format!(
                "Coordinator deletion notification failed for {}: {}",
                model_id, err
            ))),
            Err(e) => Err(OHMSError::CommunicationFailed(format!(
                "Coordinator call failed: {:?}",
                e
            ))),
        }
    } else {
        Ok(()) // No coordinator available
    }
}

fn get_coordinator_canister_id() -> Option<Principal> {
    if let Some(configured) = option_env!("OHMS_COORDINATOR_CANISTER_ID") {
        return Principal::from_text(configured).ok();
    }

    None
}

// Candid interface export
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;

    #[test]
    fn generate_candid() {
        let did = export_candid();
        std::fs::write("src/ohms_model.did", did).expect("failed to write Candid interface");
    }
}
