use candid::{CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use ohms_adaptq::novaq::NOVAQModel;
use ohms_adaptq::{PublicNOVAQ, ValidationReport};

pub struct ModelRepoClient {
    canister_id: Principal,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct ModelInfo {
    pub model_id: String,
    pub version: String,
    pub state: ModelState,
    pub compression_type: CompressionType,
    pub compression_ratio: Option<f32>,
    pub accuracy_retention: Option<f32>,
    pub size_bytes: u64,
    pub uploaded_at: u64,
    pub activated_at: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum ModelState {
    Pending,
    Active,
    Deprecated,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum CompressionType {
    NOVAQ,
    Uncompressed,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct NOVAQValidationResult {
    pub compression_ratio: f32,
    pub bit_accuracy: f32,
    pub quality_score: f32,
    pub passed: bool,
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct NOVAQModelMeta {
    pub target_bits: f32,
    pub num_subspaces: usize,
    pub compression_ratio: f32,
    pub bit_accuracy: f32,
    pub quality_score: f32,
    pub codebook_size_l1: usize,
    pub codebook_size_l2: usize,
    pub parameter_count: usize,
    pub checksum: String,
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

    pub async fn validate_novaq_model(
        model_id: &str,
        model_data: &[u8],
    ) -> Result<NOVAQValidationResult, String> {
        let model = deserialize_model(model_data)?;
        let engine = PublicNOVAQ::new(model.config.clone());
        let report = engine
            .validate_model(&model)
            .map_err(|e| format!("failed to validate NOVAQ model {model_id}: {e}"))?;
        Ok(report.into())
    }

    pub async fn extract_novaq_metadata(model_data: &[u8]) -> Result<NOVAQModelMeta, String> {
        let model = deserialize_model(model_data)?;
        Ok(build_meta(&model))
    }

    pub fn is_novaq_model(model_data: &[u8]) -> bool {
        deserialize_model(model_data).is_ok()
    }

    pub fn get_novaq_quality_score(model_data: &[u8]) -> Result<f64, String> {
        let model = deserialize_model(model_data)?;
        Ok(build_meta(&model).quality_score as f64)
    }
}

fn deserialize_model(model_data: &[u8]) -> Result<NOVAQModel, String> {
    bincode::deserialize::<NOVAQModel>(model_data)
        .map_err(|e| format!("invalid NOVAQ model payload: {e}"))
}

fn build_meta(model: &NOVAQModel) -> NOVAQModelMeta {
    let parameter_count: usize = model
        .weight_shapes
        .values()
        .map(|shape| shape.iter().product::<usize>())
        .sum();
    let checksum = {
        let mut hasher = Sha256::new();
        hasher.update(
            bincode::serialize(model)
                .expect("serializing NOVAQ model should not fail during metadata extraction"),
        );
        hex::encode(hasher.finalize())
    };

    NOVAQModelMeta {
        target_bits: model.config.target_bits,
        num_subspaces: model.config.num_subspaces,
        compression_ratio: model.compression_ratio,
        bit_accuracy: model.bit_accuracy,
        quality_score: (model.compression_ratio / 100.0 + model.bit_accuracy) / 2.0,
        codebook_size_l1: model.config.codebook_size_l1,
        codebook_size_l2: model.config.codebook_size_l2,
        parameter_count,
        checksum,
    }
}

impl From<ValidationReport> for NOVAQValidationResult {
    fn from(report: ValidationReport) -> Self {
        Self {
            compression_ratio: report.compression_ratio,
            bit_accuracy: report.bit_accuracy,
            quality_score: report.quality_score,
            passed: report.passed_validation,
            issues: report.issues,
        }
    }
}
