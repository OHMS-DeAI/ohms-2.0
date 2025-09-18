use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, Default)]
pub struct DecodeParams {
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub repetition_penalty: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct InferenceRequest {
    pub seed: u64,
    pub prompt: String,
    pub decode_params: DecodeParams,
    pub msg_id: String,
}

impl InferenceRequest {
    pub fn cache_key(&self) -> String {
        format!("{}:{}:{}", self.seed, self.msg_id, self.prompt.hash_token())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct InferenceResponse {
    pub tokens: Vec<String>,
    pub generated_text: String,
    pub inference_time_ms: u64,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

impl InferenceResponse {
    pub fn cache_hit() -> Self {
        Self {
            tokens: Vec::new(),
            generated_text: String::new(),
            inference_time_ms: 0,
            cache_hits: 1,
            cache_misses: 0,
        }
    }
}

trait PromptHash {
    fn hash_token(&self) -> String;
}

impl PromptHash for String {
    fn hash_token(&self) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self.as_bytes());
        let digest = hasher.finalize();
        hex::encode(&digest[..8])
    }
}

impl PromptHash for &str {
    fn hash_token(&self) -> String {
        self.to_string().hash_token()
    }
}
