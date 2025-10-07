use candid::Principal;
use ic_cdk::api::call::{call, CallResult};
use ohms_shared::{ArtifactChunkInfo, ModelInfo, ModelManifest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkData {
    pub info: ArtifactChunkInfo,
    pub data: Vec<u8>,
}

pub struct ModelRepoClient {
    canister_id: Principal,
}

impl ModelRepoClient {
    pub fn new(principal: Principal) -> Self {
        Self {
            canister_id: principal,
        }
    }

    pub async fn get_model_info(&self, model_id: &str) -> Result<ModelInfo, String> {
        let result: CallResult<(Result<ModelInfo, String>,)> =
            call(self.canister_id, "get_model_info", (model_id.to_string(),)).await;

        match result {
            Ok((Ok(info),)) => Ok(info),
            Ok((Err(err),)) => Err(err),
            Err((code, msg)) => Err(format!("model repo call failed ({code:?}): {msg}")),
        }
    }

    pub async fn list_models(
        &self,
        state: Option<ohms_shared::ModelState>,
    ) -> Result<Vec<ModelInfo>, String> {
        let result: CallResult<(Vec<ModelInfo>,)> =
            call(self.canister_id, "list_models", (state,)).await;

        match result {
            Ok((models,)) => Ok(models),
            Err((code, msg)) => Err(format!("list_models failed ({code:?}): {msg}")),
        }
    }

    pub async fn get_manifest(&self, model_id: &str) -> Result<ModelManifest, String> {
        let result: CallResult<(Option<ModelManifest>,)> =
            call(self.canister_id, "get_manifest", (model_id.to_string(),)).await;

        match result {
            Ok((Some(manifest),)) => Ok(manifest),
            Ok((None,)) => Err(format!("manifest for {model_id} not found")),
            Err((code, msg)) => Err(format!("get_manifest failed ({code:?}): {msg}")),
        }
    }

    pub async fn fetch_chunk(&self, model_id: &str, chunk_id: &str) -> Result<Vec<u8>, String> {
        let result: CallResult<(Option<Vec<u8>>,)> = call(
            self.canister_id,
            "get_chunk",
            (model_id.to_string(), chunk_id.to_string()),
        )
        .await;

        match result {
            Ok((Some(data),)) => Ok(data),
            Ok((None,)) => Err(format!("chunk {chunk_id} not found for model {model_id}")),
            Err((code, msg)) => Err(format!("get_chunk failed ({code:?}): {msg}")),
        }
    }

    pub async fn notify_model_access(&self, model_id: &str, agent_id: &str) -> Result<(), String> {
        let result: CallResult<(Result<(), String>,)> = call(
            self.canister_id,
            "notify_model_access",
            (model_id.to_string(), agent_id.to_string()),
        )
        .await;

        match result {
            Ok((Ok(()),)) => Ok(()),
            Ok((Err(err),)) => Err(err),
            Err((code, msg)) => Err(format!("notify_model_access failed ({code:?}): {msg}")),
        }
    }

    pub fn verify_chunk_data(chunk: &ArtifactChunkInfo, data: &[u8]) -> Result<(), String> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let digest = hex::encode(hasher.finalize());
        if digest == chunk.sha256 {
            Ok(())
        } else {
            Err(format!(
                "chunk {} hash mismatch: expected {}, computed {}",
                chunk.chunk_id, chunk.sha256, digest
            ))
        }
    }

    pub fn build_chunk_data(chunk: ArtifactChunkInfo, data: Vec<u8>) -> Result<ChunkData, String> {
        Self::verify_chunk_data(&chunk, &data)?;
        Ok(ChunkData { info: chunk, data })
    }
}
