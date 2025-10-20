use std::collections::HashMap;

use candid::Principal;
use ic_cdk::api::time;
use ohms_shared::{ModelManifest, ModelState, QuantizationFormat, QuantizedArtifactMetadata};

use crate::domain::{AgentConfig, AgentHealth};

use super::{
    with_state, with_state_mut, CacheService, MemoryService, ModelBindingState, ModelRepoClient,
};

const IC_LLAMA_MODEL_IDS: &[&str] = &[
    "llama3.1-8b",
    "llama3.1:8b",
    "ic-llm/llama3.1-8b",
    "IC_LLAMa3.1-8B",
    "ic-llm/llama-3.1-8b-instruct",
];

pub struct BindingService;

impl BindingService {
    pub fn get_config() -> Result<AgentConfig, String> {
        Ok(with_state(|state| state.config.clone()))
    }

    pub fn set_config(config: AgentConfig) -> Result<(), String> {
        if config.model_repo_canister_id.is_empty() {
            return Err("model_repo_canister_id must be configured".to_string());
        }

        with_state_mut(|state| {
            state.config = config;
            MemoryService::reconfigure(state);
        });
        Ok(())
    }

    pub async fn bind_model(model_id: String) -> Result<(), String> {
        if Self::is_ic_llama_model(&model_id) {
            Self::bind_ic_llama_model(&model_id);
            return Ok(());
        }

        let canister_id = with_state(|state| state.config.model_repo_canister_id.clone());
        if canister_id.is_empty() {
            return Err("model repository canister id is not configured".to_string());
        }

        let principal = Principal::from_text(canister_id.clone())
            .map_err(|e| format!("invalid model repo canister id: {e}"))?;

        let client = ModelRepoClient::new(principal);
        let model_info = client
            .get_model_info(&model_id)
            .await
            .map_err(|e| format!("failed to fetch model metadata: {e}"))?;

        let manifest = client
            .get_manifest(&model_id)
            .await
            .map_err(|e| format!("failed to fetch model manifest: {e}"))?;

        if model_info.model_id != model_id {
            return Err(format!(
                "model repository returned mismatched metadata: requested {}, got {}",
                model_id, model_info.model_id
            ));
        }

        if manifest.quantization.format != model_info.quantization_format {
            return Err(format!(
                "quantization format mismatch for {}: manifest {:?}, info {:?}",
                model_id, manifest.quantization.format, model_info.quantization_format
            ));
        }

        with_state_mut(|state| {
            state.binding = Some(ModelBindingState::new(manifest.clone()));
            CacheService::clear_all(state);
        });

        // Notify the model repository that this agent intends to use the model. We log the
        // error but don't fail the binding to keep the system responsive if the repo is busy.
        if let Err(err) = client
            .notify_model_access(&model_id, &ic_cdk::id().to_string())
            .await
        {
            ic_cdk::println!("Model access notification failed: {err}");
        }

        Ok(())
    }

    fn is_ic_llama_model(model_id: &str) -> bool {
        let normalized = model_id.trim().to_lowercase();
        IC_LLAMA_MODEL_IDS
            .iter()
            .any(|candidate| normalized == candidate.trim().to_lowercase())
    }

    fn bind_ic_llama_model(model_id: &str) {
        let now = time();
        let manifest = ModelManifest {
            model_id: model_id.to_string(),
            version: "ic-llm-1.0.0".to_string(),
            state: ModelState::Active,
            uploaded_at: now,
            activated_at: Some(now),
            total_size_bytes: 0,
            chunk_count: 1,
            checksum: "ic-llm-remote-model".to_string(),
            quantization: QuantizedArtifactMetadata {
                format: QuantizationFormat::Custom("ic-llm".to_string()),
                artifact_checksum: "ic-llm".to_string(),
                compression_ratio: 1.0,
                accuracy_retention: 1.0,
                bits_per_weight: None,
                notes: Some(
                    "Served directly from the Internet Computer LLM canister; no local chunks."
                        .to_string(),
                ),
            },
            metadata: {
                let mut map = HashMap::new();
                map.insert("provider".to_string(), "ic-llm".to_string());
                map.insert("model".to_string(), "llama3.1-8b".to_string());
                map
            },
            chunks: Vec::new(),
        };

        with_state_mut(|state| {
            let mut binding = ModelBindingState::new(manifest);
            binding.chunks_loaded = binding.total_chunks;
            binding.ready = true;
            state.binding = Some(binding);
            CacheService::clear_all(state);
            MemoryService::reconfigure(state);
        });
    }

    pub async fn prefetch_next(requested: u32) -> Result<u32, String> {
        if requested == 0 {
            return Ok(0);
        }

        let mut result = Ok(0);
        with_state_mut(|state| {
            let binding = match state.binding.as_mut() {
                Some(binding) => binding,
                None => {
                    result = Err("no model is currently bound".to_string());
                    return;
                }
            };

            let warm_target = state.config.warm_set_target.clamp(0.0, 1.0);
            let desired_chunks = ((binding.total_chunks as f32) * warm_target).ceil() as u32;
            let remaining = desired_chunks.saturating_sub(binding.chunks_loaded);
            let to_prefetch = remaining.min(requested).max(0);

            if to_prefetch == 0 {
                result = Ok(0);
                return;
            }

            binding.register_prefetch(to_prefetch);
            CacheService::reserve_slots(state, to_prefetch as usize);
            result = Ok(to_prefetch);
        });
        result
    }

    pub fn get_health() -> AgentHealth {
        with_state(|state| {
            let cache_total = state.metrics.total_cache_hits + state.metrics.total_cache_misses;
            let hit_rate = if cache_total == 0 {
                0.0
            } else {
                state.metrics.total_cache_hits as f32 / cache_total as f32
            };

            let warm_utilization = state
                .binding
                .as_ref()
                .map(|binding| {
                    if binding.total_chunks == 0 {
                        0.0
                    } else {
                        binding.chunks_loaded as f32 / binding.total_chunks as f32
                    }
                })
                .unwrap_or(0.0);

            AgentHealth {
                model_bound: state
                    .binding
                    .as_ref()
                    .map(|b| b.ready || b.chunks_loaded > 0)
                    .unwrap_or(false),
                cache_hit_rate: hit_rate,
                warm_set_utilization: warm_utilization,
                queue_depth: state.task_queue.len() as u32,
                last_inference_timestamp: state.last_inference,
            }
        })
    }
}
