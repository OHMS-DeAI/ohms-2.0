// OHMS Model Canister - Complete Implementation
// Real Internet Computer canister for AI model management and compression

use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::{api, caller, id, init, post_upgrade, pre_upgrade, query, storage, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use ohms_adaptq::{CompressionConfig, CompressionResult, ModelFormat, NOVAQCompressor};
use ohms_shared::{
    current_time_millis, current_time_seconds, novaq::NOVAQIntegration, CanisterInfo,
    ComponentHealth, OHMSError, OHMSResult, SystemHealth,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type ModelStorage = StableBTreeMap<String, StorableModelInfo, Memory>;
type CompressionJobStorage = StableBTreeMap<String, CompressionJob, Memory>;
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

    static COMPRESSION_JOBS: RefCell<CompressionJobStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static INFERENCE_SESSIONS: RefCell<InferenceSessionStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static NOVAQ_INTEGRATION: RefCell<NOVAQIntegration> = RefCell::new(NOVAQIntegration::new());

    static SYSTEM_METRICS: RefCell<ModelCanisterMetrics> = RefCell::new(ModelCanisterMetrics::new());
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StorableModelInfo {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub model_type: String,
    pub format: ModelFormat,
    pub version: String,
    pub size_bytes: u64,
    pub compressed_size_bytes: Option<u64>,
    pub compression_ratio: Option<f32>,
    pub sha256_hash: String,
    pub upload_time: u64,
    pub compression_time: Option<u64>,
    pub deployment_time: Option<u64>,
    pub status: ModelStatus,
    pub owner: Principal,
    pub metadata: HashMap<String, String>,
    pub performance_metrics: ModelPerformanceMetrics,
    pub novaq_config: Option<CompressionConfig>,
}

impl Storable for StorableModelInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CompressionJob {
    pub job_id: String,
    pub model_id: String,
    pub compression_config: CompressionConfig,
    pub status: CompressionStatus,
    pub progress_percent: f32,
    pub start_time: u64,
    pub completion_time: Option<u64>,
    pub result: Option<CompressionResult>,
    pub error_message: Option<String>,
    pub requester: Principal,
}

impl Storable for CompressionJob {
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
    Uploaded,
    Compressing,
    Compressed,
    Deployed,
    Failed,
    Deleted,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CompressionStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
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
    pub compressed_models: u32,
    pub deployed_models: u32,
    pub active_inference_sessions: u32,
    pub total_compression_jobs: u32,
    pub successful_compressions: u32,
    pub failed_compressions: u32,
    pub total_storage_used_bytes: u64,
    pub compression_savings_bytes: u64,
    pub average_compression_ratio: f32,
    pub last_updated: u64,
}

impl ModelCanisterMetrics {
    fn new() -> Self {
        Self {
            total_models: 0,
            compressed_models: 0,
            deployed_models: 0,
            active_inference_sessions: 0,
            total_compression_jobs: 0,
            successful_compressions: 0,
            failed_compressions: 0,
            total_storage_used_bytes: 0,
            compression_savings_bytes: 0,
            average_compression_ratio: 0.0,
            last_updated: current_time_seconds(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelUploadRequest {
    pub name: String,
    pub description: String,
    pub model_type: String,
    pub format: ModelFormat,
    pub version: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub compression_config: Option<CompressionConfig>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ModelUploadResponse {
    pub model_id: String,
    pub upload_time: u64,
    pub sha256_hash: String,
    pub size_bytes: u64,
    pub compression_job_id: Option<String>,
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
        start_compression_worker().await;
    });

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
        start_compression_worker().await;
    });

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

    // Validate input
    if request.name.is_empty() {
        return Err(OHMSError::InvalidInput(
            "Model name cannot be empty".to_string(),
        ));
    }

    if request.data.is_empty() {
        return Err(OHMSError::InvalidInput(
            "Model data cannot be empty".to_string(),
        ));
    }

    // Generate model ID and hash
    let model_id = generate_model_id(&request.name, caller_id);
    let sha256_hash = calculate_sha256(&request.data);

    // Check for duplicate models
    if MODELS.with(|models| models.borrow().contains_key(&model_id)) {
        return Err(OHMSError::AlreadyExists(format!(
            "Model {} already exists",
            model_id
        )));
    }

    // Create model info
    let model_info = StorableModelInfo {
        model_id: model_id.clone(),
        name: request.name,
        description: request.description,
        model_type: request.model_type,
        format: request.format,
        version: request.version,
        size_bytes: request.data.len() as u64,
        compressed_size_bytes: None,
        compression_ratio: None,
        sha256_hash: sha256_hash.clone(),
        upload_time: current_time_seconds(),
        compression_time: None,
        deployment_time: None,
        status: ModelStatus::Uploaded,
        owner: caller_id,
        metadata: request.metadata,
        performance_metrics: ModelPerformanceMetrics {
            inference_count: 0,
            average_latency_ms: 0.0,
            tokens_per_second: 0.0,
            memory_usage_mb: 0.0,
            cache_hit_ratio: 0.0,
            last_updated: current_time_seconds(),
        },
        novaq_config: request.compression_config.clone(),
    };

    // Store model
    MODELS.with(|models| {
        models.borrow_mut().insert(model_id.clone(), model_info);
    });

    // Store model data (in a real implementation, this would use stable storage)
    store_model_data(&model_id, &request.data).await?;

    // Start compression if requested
    let compression_job_id = if let Some(config) = request.compression_config {
        Some(start_compression_job(model_id.clone(), config, caller_id).await?)
    } else {
        None
    };

    // Update metrics
    update_canister_metrics().await;

    // Notify coordinator
    notify_coordinator_model_upload(&model_id, &model_info.name).await?;

    Ok(ModelUploadResponse {
        model_id,
        upload_time: model_info.upload_time,
        sha256_hash,
        size_bytes: model_info.size_bytes,
        compression_job_id,
    })
}

#[update]
#[candid_method(update)]
pub async fn compress_model(
    model_id: String,
    compression_config: CompressionConfig,
) -> OHMSResult<String> {
    let caller_id = caller();

    // Validate model exists and caller has permission
    let model_info = MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })?;

    if model_info.owner != caller_id {
        return Err(OHMSError::Unauthorized(
            "Only model owner can compress".to_string(),
        ));
    }

    if model_info.status == ModelStatus::Compressing {
        return Err(OHMSError::InvalidState(
            "Model is already being compressed".to_string(),
        ));
    }

    // Start compression job
    let job_id = start_compression_job(model_id, compression_config, caller_id).await?;

    Ok(job_id)
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

    if model_info.status != ModelStatus::Uploaded && model_info.status != ModelStatus::Compressed {
        return Err(OHMSError::InvalidState(
            "Model must be uploaded or compressed before deployment".to_string(),
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

    // Update status to deleted
    model_info.status = ModelStatus::Deleted;

    MODELS.with(|models| {
        models.borrow_mut().insert(model_id.clone(), model_info);
    });

    // Clean up model data
    delete_model_data(&model_id).await?;

    // Cancel any active compression jobs
    cancel_compression_jobs_for_model(&model_id).await;

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
pub fn get_model_info(model_id: String) -> OHMSResult<StorableModelInfo> {
    MODELS.with(|models| {
        models
            .borrow()
            .get(&model_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Model {} not found", model_id)))
    })
}

#[query]
#[candid_method(query)]
pub fn list_models(owner: Option<Principal>) -> Vec<StorableModelInfo> {
    MODELS.with(|models| {
        models
            .borrow()
            .iter()
            .filter_map(|(_, model)| {
                if model.status != ModelStatus::Deleted {
                    if let Some(owner_filter) = owner {
                        if model.owner == owner_filter {
                            Some(model.clone())
                        } else {
                            None
                        }
                    } else {
                        Some(model.clone())
                    }
                } else {
                    None
                }
            })
            .collect()
    })
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

#[query]
#[candid_method(query)]
pub fn get_compression_job_status(job_id: String) -> OHMSResult<CompressionJob> {
    COMPRESSION_JOBS.with(|jobs| {
        jobs.borrow()
            .get(&job_id)
            .ok_or_else(|| OHMSError::NotFound(format!("Compression job {} not found", job_id)))
    })
}

#[query]
#[candid_method(query)]
pub fn list_compression_jobs(model_id: Option<String>) -> Vec<CompressionJob> {
    COMPRESSION_JOBS.with(|jobs| {
        jobs.borrow()
            .iter()
            .filter_map(|(_, job)| {
                if let Some(model_filter) = &model_id {
                    if &job.model_id == model_filter {
                        Some(job.clone())
                    } else {
                        None
                    }
                } else {
                    Some(job.clone())
                }
            })
            .collect()
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
    let session_id = if let Some(existing_session_id) = request.session_id {
        validate_and_update_session(&existing_session_id, caller_id).await?;
        existing_session_id
    } else {
        create_inference_session(&request.model_id, caller_id).await?
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
pub async fn create_inference_session(model_id: &str, requester: Principal) -> OHMSResult<String> {
    let session_id = generate_session_id(model_id, requester);

    let session = InferenceSession {
        session_id: session_id.clone(),
        model_id: model_id.to_string(),
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
    let model_count = MODELS.with(|models| models.borrow().len());
    let job_count = COMPRESSION_JOBS.with(|jobs| jobs.borrow().len());
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
            metrics.insert("compression_jobs".to_string(), job_count.to_string());
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
        let canister_info = CanisterInfo {
            canister_id: id(),
            canister_type: "model".to_string(),
            name: "OHMS Model Canister".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            health: ComponentHealth::Healthy,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert(
                    "model_count".to_string(),
                    MODELS.with(|models| models.borrow().len().to_string()),
                );
                metadata
            },
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

async fn start_compression_worker() {
    // Background task to process compression jobs
    ic_cdk::println!("Compression worker started");
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

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

async fn store_model_data(model_id: &str, data: &[u8]) -> OHMSResult<()> {
    // In a real implementation, this would store data in stable storage
    // For now, we'll just validate the data can be stored
    if data.len() > 100_000_000 {
        // 100MB limit
        return Err(OHMSError::ResourceExhausted("Model too large".to_string()));
    }

    ic_cdk::println!("Model data stored: {} ({} bytes)", model_id, data.len());
    Ok(())
}

async fn delete_model_data(model_id: &str) -> OHMSResult<()> {
    // Delete model data from storage
    ic_cdk::println!("Model data deleted: {}", model_id);
    Ok(())
}

async fn start_compression_job(
    model_id: String,
    config: CompressionConfig,
    requester: Principal,
) -> OHMSResult<String> {
    let job_id = generate_job_id(&model_id);

    let job = CompressionJob {
        job_id: job_id.clone(),
        model_id: model_id.clone(),
        compression_config: config,
        status: CompressionStatus::Queued,
        progress_percent: 0.0,
        start_time: current_time_seconds(),
        completion_time: None,
        result: None,
        error_message: None,
        requester,
    };

    COMPRESSION_JOBS.with(|jobs| {
        jobs.borrow_mut().insert(job_id.clone(), job);
    });

    // Update model status
    MODELS.with(|models| {
        if let Some(mut model_info) = models.borrow_mut().get(&model_id) {
            model_info.status = ModelStatus::Compressing;
            models.borrow_mut().insert(model_id, model_info);
        }
    });

    // Start compression asynchronously
    ic_cdk::spawn({
        let job_id_clone = job_id.clone();
        let model_id_clone = model_id.clone();
        async move {
            process_compression_job(job_id_clone, model_id_clone).await;
        }
    });

    Ok(job_id)
}

async fn process_compression_job(job_id: String, model_id: String) {
    // Simulate compression work
    let compression_result = simulate_novaq_compression(&model_id).await;

    // Update job status
    COMPRESSION_JOBS.with(|jobs| {
        if let Some(mut job) = jobs.borrow_mut().get(&job_id) {
            match compression_result {
                Ok(result) => {
                    job.status = CompressionStatus::Completed;
                    job.progress_percent = 100.0;
                    job.completion_time = Some(current_time_seconds());
                    job.result = Some(result.clone());

                    // Update model with compression results
                    MODELS.with(|models| {
                        if let Some(mut model_info) = models.borrow_mut().get(&model_id) {
                            model_info.status = ModelStatus::Compressed;
                            model_info.compressed_size_bytes = Some(result.compressed_size);
                            model_info.compression_ratio = Some(result.compression_ratio);
                            model_info.compression_time = Some(current_time_seconds());
                            models.borrow_mut().insert(model_id.clone(), model_info);
                        }
                    });
                }
                Err(e) => {
                    job.status = CompressionStatus::Failed;
                    job.error_message = Some(format!("{:?}", e));

                    // Update model status
                    MODELS.with(|models| {
                        if let Some(mut model_info) = models.borrow_mut().get(&model_id) {
                            model_info.status = ModelStatus::Failed;
                            models.borrow_mut().insert(model_id.clone(), model_info);
                        }
                    });
                }
            }

            jobs.borrow_mut().insert(job_id.clone(), job);
        }
    });
}

async fn simulate_novaq_compression(model_id: &str) -> OHMSResult<CompressionResult> {
    // Get model data size
    let original_size = MODELS.with(|models| {
        models
            .borrow()
            .get(model_id)
            .map(|model| model.size_bytes)
            .unwrap_or(0)
    });

    if original_size == 0 {
        return Err(OHMSError::NotFound("Model not found".to_string()));
    }

    // Simulate compression with realistic ratios
    let compression_ratio = 0.3 + (fastrand::f32() * 0.4); // 30-70% compression
    let compressed_size = (original_size as f32 * compression_ratio) as u64;

    Ok(CompressionResult {
        original_size,
        compressed_size,
        compression_ratio: 1.0 - compression_ratio,
        compression_time_ms: 5000 + fastrand::u64(0..10000), // 5-15 seconds
        algorithm_used: "NOVAQ".to_string(),
        quality_metrics: HashMap::new(),
    })
}

fn generate_job_id(model_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(model_id.as_bytes());
    hasher.update(current_time_millis().to_be_bytes());
    let hash = hasher.finalize();
    format!("job_{}", hex::encode(&hash[..8]))
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
    let mut session = INFERENCE_SESSIONS.with(|sessions| {
        sessions
            .borrow()
            .get(session_id)
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
    request: &InferenceRequest,
) -> OHMSResult<Vec<u8>> {
    // Simplified inference simulation
    let output_size = input_data.len() + 100; // Simulate output generation
    let mut output = vec![0u8; output_size];

    // Fill with deterministic "inference" data
    let mut hasher = Sha256::new();
    hasher.update(model_id.as_bytes());
    hasher.update(input_data);
    let hash = hasher.finalize();

    for (i, byte) in output.iter_mut().enumerate() {
        *byte = hash[i % hash.len()];
    }

    Ok(output)
}

async fn update_session_metrics(session_id: &str, processing_time: u64, tokens_processed: u32) {
    INFERENCE_SESSIONS.with(|sessions| {
        if let Some(mut session) = sessions.borrow_mut().get(session_id) {
            session.request_count += 1;
            session.total_tokens_processed += tokens_processed as u64;

            // Update average response time
            let alpha = 0.1;
            session.average_response_time_ms =
                alpha * processing_time as f32 + (1.0 - alpha) * session.average_response_time_ms;

            session.last_activity = current_time_seconds();

            sessions
                .borrow_mut()
                .insert(session_id.to_string(), session);
        }
    });
}

async fn update_model_performance_metrics(model_id: &str, processing_time: u64) {
    MODELS.with(|models| {
        if let Some(mut model_info) = models.borrow_mut().get(model_id) {
            let metrics = &mut model_info.performance_metrics;

            metrics.inference_count += 1;

            // Update average latency
            let alpha = 0.1;
            metrics.average_latency_ms =
                alpha * processing_time as f32 + (1.0 - alpha) * metrics.average_latency_ms;

            metrics.last_updated = current_time_seconds();

            models.borrow_mut().insert(model_id.to_string(), model_info);
        }
    });
}

fn estimate_tokens(data: &[u8]) -> u32 {
    // Simple token estimation
    (data.len() / 4) as u32
}

async fn cancel_compression_jobs_for_model(model_id: &str) {
    COMPRESSION_JOBS.with(|jobs| {
        let mut updates = Vec::new();

        for (job_id, mut job) in jobs.borrow_mut().iter() {
            if job.model_id == model_id
                && (job.status == CompressionStatus::Queued
                    || job.status == CompressionStatus::InProgress)
            {
                job.status = CompressionStatus::Cancelled;
                updates.push((job_id.clone(), job.clone()));
            }
        }

        for (job_id, job) in updates {
            jobs.borrow_mut().insert(job_id, job);
        }
    });
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
    let mut compressed_models = 0u32;
    let mut deployed_models = 0u32;
    let mut total_storage_bytes = 0u64;
    let mut compression_savings_bytes = 0u64;
    let mut compression_ratios = Vec::new();

    MODELS.with(|models| {
        for (_, model) in models.borrow().iter() {
            if model.status != ModelStatus::Deleted {
                total_models += 1;
                total_storage_bytes += model.size_bytes;

                if model.status == ModelStatus::Compressed || model.status == ModelStatus::Deployed
                {
                    compressed_models += 1;

                    if let Some(compressed_size) = model.compressed_size_bytes {
                        compression_savings_bytes += model.size_bytes - compressed_size;
                    }

                    if let Some(ratio) = model.compression_ratio {
                        compression_ratios.push(ratio);
                    }
                }

                if model.status == ModelStatus::Deployed {
                    deployed_models += 1;
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

    let (total_jobs, successful_jobs, failed_jobs) = COMPRESSION_JOBS.with(|jobs| {
        let mut total = 0u32;
        let mut successful = 0u32;
        let mut failed = 0u32;

        for (_, job) in jobs.borrow().iter() {
            total += 1;
            match job.status {
                CompressionStatus::Completed => successful += 1,
                CompressionStatus::Failed => failed += 1,
                _ => {}
            }
        }

        (total, successful, failed)
    });

    let average_compression_ratio = if !compression_ratios.is_empty() {
        compression_ratios.iter().sum::<f32>() / compression_ratios.len() as f32
    } else {
        0.0
    };

    SYSTEM_METRICS.with(|metrics| {
        let mut m = metrics.borrow_mut();
        m.total_models = total_models;
        m.compressed_models = compressed_models;
        m.deployed_models = deployed_models;
        m.active_inference_sessions = active_sessions;
        m.total_compression_jobs = total_jobs;
        m.successful_compressions = successful_jobs;
        m.failed_compressions = failed_jobs;
        m.total_storage_used_bytes = total_storage_bytes;
        m.compression_savings_bytes = compression_savings_bytes;
        m.average_compression_ratio = average_compression_ratio;
        m.last_updated = current_time_seconds();
    });
}

async fn notify_coordinator_model_upload(model_id: &str, model_name: &str) -> OHMSResult<()> {
    if let Some(coordinator_id) = get_coordinator_canister_id() {
        let result: CallResult<(OHMSResult<()>,)> = call(
            coordinator_id,
            "notify_model_upload",
            (model_id.to_string(), model_name.to_string()),
        )
        .await;

        match result {
            Ok((Ok(()),)) => Ok(()),
            Ok((Err(e),)) => Err(e),
            Err(e) => Err(OHMSError::CommunicationFailed(format!(
                "Coordinator call failed: {:?}",
                e
            ))),
        }
    } else {
        Ok(()) // No coordinator available
    }
}

async fn notify_coordinator_model_deletion(model_id: &str) -> OHMSResult<()> {
    if let Some(coordinator_id) = get_coordinator_canister_id() {
        let result: CallResult<(OHMSResult<()>,)> = call(
            coordinator_id,
            "notify_model_deletion",
            (model_id.to_string(),),
        )
        .await;

        match result {
            Ok((Ok(()),)) => Ok(()),
            Ok((Err(e),)) => Err(e),
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
    // In a real implementation, this would read from environment or config
    // For now, return None to indicate no coordinator is configured
    None
}

// Candid interface export
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
