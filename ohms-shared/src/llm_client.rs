use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request structure for LLM inference
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct LlmRequest {
    pub prompt: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub system_prompt: Option<String>,
    pub user_id: String,
}

/// Response from LLM inference
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct LlmResponse {
    pub content: String,
    pub tokens_used: u32,
    pub provider: String,
    pub model: String,
    pub finish_reason: String,
    pub cached: bool,
}

/// Supported LLM providers
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum LlmProvider {
    Groq,
    TogetherAi,
    OpenRouter,
    UserKey { provider: String, api_key: String },
}

impl LlmProvider {
    pub fn name(&self) -> String {
        match self {
            LlmProvider::Groq => "groq".to_string(),
            LlmProvider::TogetherAi => "together_ai".to_string(),
            LlmProvider::OpenRouter => "openrouter".to_string(),
            LlmProvider::UserKey { provider, .. } => provider.clone(),
        }
    }
}

/// Rate limit tracking for a provider
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub provider: String,
    pub requests_this_minute: u32,
    pub tokens_this_minute: u32,
    pub last_reset: u64,
    pub max_requests_per_minute: u32,
    pub max_tokens_per_minute: u32,
}

impl RateLimit {
    pub fn new(provider: String, rpm: u32, tpm: u32) -> Self {
        Self {
            provider,
            requests_this_minute: 0,
            tokens_this_minute: 0,
            last_reset: 0,
            max_requests_per_minute: rpm,
            max_tokens_per_minute: tpm,
        }
    }

    pub fn can_make_request(&self, estimated_tokens: u32, now: u64) -> bool {
        if now - self.last_reset > 60_000_000_000 {
            return true;
        }

        self.requests_this_minute < self.max_requests_per_minute
            && self.tokens_this_minute + estimated_tokens < self.max_tokens_per_minute
    }

    pub fn record_request(&mut self, tokens_used: u32, now: u64) {
        if now - self.last_reset > 60_000_000_000 {
            self.requests_this_minute = 0;
            self.tokens_this_minute = 0;
            self.last_reset = now;
        }

        self.requests_this_minute += 1;
        self.tokens_this_minute += tokens_used;
    }
}

/// Cached LLM response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedResponse {
    pub response: LlmResponse,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CachedResponse {
    pub fn is_valid(&self, now: u64) -> bool {
        now - self.cached_at < self.ttl_seconds * 1_000_000_000
    }
}

/// Provider configuration
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub free_tier_rpm: u32,
    pub free_tier_tpm: u32,
    pub default_model: String,
    pub supported_models: Vec<String>,
}

impl ProviderConfig {
    pub fn groq() -> Self {
        Self {
            name: "groq".to_string(),
            base_url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
            free_tier_rpm: 30,
            free_tier_tpm: 14_400,
            default_model: "llama-3.1-8b-instant".to_string(),
            supported_models: vec![
                "llama-3.1-8b-instant".to_string(),
                "mixtral-8x7b-32768".to_string(),
                "gemma2-9b-it".to_string(),
            ],
        }
    }

    pub fn together_ai() -> Self {
        Self {
            name: "together_ai".to_string(),
            base_url: "https://api.together.xyz/v1/chat/completions".to_string(),
            free_tier_rpm: 60,
            free_tier_tpm: 60_000,
            default_model: "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo".to_string(),
            supported_models: vec![
                "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo".to_string(),
                "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(),
                "google/gemma-2-9b-it".to_string(),
            ],
        }
    }

    pub fn openrouter() -> Self {
        Self {
            name: "openrouter".to_string(),
            base_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
            free_tier_rpm: 20,
            free_tier_tpm: 10_000,
            default_model: "google/gemma-2-9b-it:free".to_string(),
            supported_models: vec![
                // Mistral Models (free)
                "mistralai/mistral-small-3".to_string(),
                "mistralai/mistral-7b-instruct:free".to_string(),
                // Google Gemma Models (free)
                "google/gemma-2-9b-it:free".to_string(),
                "google/gemma-3-12b:free".to_string(),
                "google/gemma-3-4b:free".to_string(),
                "google/gemma-3n-4b:free".to_string(),
                "google/gemma-3n-2b:free".to_string(),
                // Meta Llama (free)
                "meta-llama/llama-3.1-8b-instruct:free".to_string(),
                // Tencent Hunyuan (free)
                "tencent/hunyuan-a13b-instruct:free".to_string(),
                // Qwen Models (free)
                "arliai/qwq-32b-rpr-v1:free".to_string(),
                // Code Generation (free)
                "agentica-org/deepcoder-14b-preview:free".to_string(),
            ],
        }
    }
}

/// Main LLM client with multi-provider support
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LlmClient {
    pub providers: Vec<LlmProvider>,
    pub rate_limits: HashMap<String, RateLimit>,
    pub cache: HashMap<String, CachedResponse>,
    pub cache_capacity: usize,
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmClient {
    pub fn new() -> Self {
        let mut rate_limits = HashMap::new();
        
        let groq_config = ProviderConfig::groq();
        rate_limits.insert(
            "groq".to_string(),
            RateLimit::new("groq".to_string(), groq_config.free_tier_rpm, groq_config.free_tier_tpm),
        );

        let together_config = ProviderConfig::together_ai();
        rate_limits.insert(
            "together_ai".to_string(),
            RateLimit::new("together_ai".to_string(), together_config.free_tier_rpm, together_config.free_tier_tpm),
        );

        let openrouter_config = ProviderConfig::openrouter();
        rate_limits.insert(
            "openrouter".to_string(),
            RateLimit::new("openrouter".to_string(), openrouter_config.free_tier_rpm, openrouter_config.free_tier_tpm),
        );

        Self {
            providers: vec![
                LlmProvider::Groq,
                LlmProvider::TogetherAi,
                LlmProvider::OpenRouter,
            ],
            rate_limits,
            cache: HashMap::new(),
            cache_capacity: 1000,
        }
    }

    /// Generate cache key from request
    pub fn cache_key(request: &LlmRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.prompt.hash(&mut hasher);
        request.model.hash(&mut hasher);
        request.max_tokens.hash(&mut hasher);
        request.temperature.to_bits().hash(&mut hasher);
        if let Some(ref sys) = request.system_prompt {
            sys.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }

    /// Check cache for response
    pub fn get_cached(&self, request: &LlmRequest, now: u64) -> Option<LlmResponse> {
        let key = Self::cache_key(request);
        if let Some(cached) = self.cache.get(&key) {
            if cached.is_valid(now) {
                let mut response = cached.response.clone();
                response.cached = true;
                return Some(response);
            }
        }
        None
    }

    /// Cache a response
    pub fn cache_response(&mut self, request: &LlmRequest, response: &LlmResponse, now: u64) {
        let key = Self::cache_key(request);
        
        if self.cache.len() >= self.cache_capacity {
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }

        self.cache.insert(
            key,
            CachedResponse {
                response: response.clone(),
                cached_at: now,
                ttl_seconds: 3600,
            },
        );
    }

    /// Select best available provider based on rate limits
    pub fn select_provider(&mut self, estimated_tokens: u32, now: u64) -> Option<LlmProvider> {
        for provider in &self.providers {
            let name = provider.name();
            if let Some(rate_limit) = self.rate_limits.get(&name) {
                if rate_limit.can_make_request(estimated_tokens, now) {
                    return Some(provider.clone());
                }
            }
        }
        None
    }

    /// Track rate limit usage
    pub fn track_usage(&mut self, provider: &str, tokens_used: u32, now: u64) {
        if let Some(rate_limit) = self.rate_limits.get_mut(provider) {
            rate_limit.record_request(tokens_used, now);
        }
    }

    /// Add user's custom API key
    pub fn add_user_key(&mut self, provider: String, api_key: String) {
        let user_provider = LlmProvider::UserKey {
            provider: provider.clone(),
            api_key,
        };
        
        self.providers.insert(0, user_provider);
        
        self.rate_limits.insert(
            provider.clone(),
            RateLimit::new(provider, 1000, 1_000_000),
        );
    }

    /// Get provider configuration
    pub fn get_provider_config(provider: &LlmProvider) -> ProviderConfig {
        match provider {
            LlmProvider::Groq => ProviderConfig::groq(),
            LlmProvider::TogetherAi => ProviderConfig::together_ai(),
            LlmProvider::OpenRouter => ProviderConfig::openrouter(),
            LlmProvider::UserKey { provider, .. } => {
                if provider.contains("groq") {
                    ProviderConfig::groq()
                } else if provider.contains("together") {
                    ProviderConfig::together_ai()
                } else {
                    ProviderConfig::openrouter()
                }
            }
        }
    }
}

/// HTTP outcall request/response structures for OpenAI-compatible APIs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

impl LlmRequest {
    /// Convert to OpenAI-compatible chat completion request
    pub fn to_chat_request(&self) -> ChatCompletionRequest {
        let mut messages = Vec::new();

        if let Some(ref sys) = self.system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: sys.clone(),
            });
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: self.prompt.clone(),
        });

        ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}
