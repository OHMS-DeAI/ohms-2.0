use ohms_shared::llm_client::{LlmClient, LlmProvider, LlmRequest, LlmResponse, ProviderConfig};
use std::collections::HashSet;
use std::cell::RefCell;
use crate::services::http_outcall::HttpOutcallService;

const GROQ_API_KEY: Option<&'static str> = option_env!("GROQ_API_KEY");
const OPENROUTER_API_KEY: Option<&'static str> = option_env!("OPENROUTER_API_KEY");

thread_local! {
    static LLM_CLIENT: RefCell<LlmClient> = RefCell::new(LlmClient::new());
}

/// Get default provider configurations
pub fn get_default_providers() -> Vec<ProviderConfig> {
    vec![ProviderConfig::groq(), ProviderConfig::openrouter()]
}

/// Access the LLM client
pub fn with_llm_client<R>(f: impl FnOnce(&LlmClient) -> R) -> R {
    LLM_CLIENT.with(|client| f(&client.borrow()))
}

/// Mutably access the LLM client
pub fn with_llm_client_mut<R>(f: impl FnOnce(&mut LlmClient) -> R) -> R {
    LLM_CLIENT.with(|client| f(&mut client.borrow_mut()))
}

/// Initialize LLM client with providers
pub fn init_llm_client() {
    with_llm_client_mut(|client| {
        *client = LlmClient::new();
    });
}

/// Add a user's custom API key
pub fn add_user_api_key(user_id: &str, provider: String, api_key: String) -> Result<(), String> {
    with_llm_client_mut(|client| {
        // Store user key with user prefix
        let user_provider = format!("{}_{}", user_id, provider);
        client.add_user_key(user_provider, api_key);
        Ok(())
    })
}

/// Get available providers
pub fn get_available_providers() -> Vec<String> {
    with_llm_client(|client| {
        client.providers.iter().map(|p| p.name()).collect()
    })
}

fn provider_api_key(provider: &LlmProvider) -> Result<Option<String>, String> {
    match provider {
        LlmProvider::Groq => GROQ_API_KEY
            .map(|k| Some(k.to_string()))
            .ok_or_else(|| "GROQ_API_KEY environment variable not set".to_string()),
        LlmProvider::OpenRouter => OPENROUTER_API_KEY
            .map(|k| Some(k.to_string()))
            .ok_or_else(|| "OPENROUTER_API_KEY environment variable not set".to_string()),
        LlmProvider::TogetherAi => Err("Together.ai provider is currently disabled".to_string()),
        LlmProvider::UserKey { api_key, .. } => Ok(Some(api_key.clone())),
    }
}

fn estimate_tokens(request: &LlmRequest) -> u32 {
    LlmClient::estimate_tokens(&request.prompt, request.max_tokens)
}

fn select_next_provider(
    estimated_tokens: u32,
    excluded: &HashSet<String>,
) -> Option<LlmProvider> {
    let now = ic_cdk::api::time();
    with_llm_client_mut(|client| {
        client.select_provider(
            estimated_tokens,
            now,
            &excluded.iter().cloned().collect::<Vec<_>>(),
        )
    })
}

fn track_provider_usage(provider: &LlmProvider, tokens_used: u32) {
    let now = ic_cdk::api::time();
    with_llm_client_mut(|client| {
        client.track_usage(&provider.name(), tokens_used, now);
    });
}

pub async fn execute_llm(request: &LlmRequest) -> Result<LlmResponse, String> {
    let estimated = estimate_tokens(request);
    let mut tried = HashSet::new();
    let mut last_error = String::from("no providers available");

    while let Some(provider) = select_next_provider(estimated, &tried) {
        let mut request_clone = request.clone();
        let config = LlmClient::get_provider_config(&provider);
        if request_clone.model == "auto" {
            request_clone.model = config.default_model.clone();
        }

        match provider_api_key(&provider) {
            Ok(api_key) => {
                match HttpOutcallService::make_llm_call(&request_clone, &provider, api_key).await {
                    Ok(mut response) => {
                        track_provider_usage(&provider, response.tokens_used.max(estimated));
                        response.model = "primary".to_string();
                        return Ok(response);
                    }
                    Err(err) => {
                        if err.contains("429") || err.to_lowercase().contains("rate limit") {
                            track_provider_usage(&provider, estimated);
                        }
                        last_error = err;
                        tried.insert(provider.name());
                    }
                }
            }
            Err(err) => {
                last_error = err;
                tried.insert(provider.name());
            }
        }
    }

    Err(format!("All providers exhausted: {}", last_error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_providers() {
        let providers = get_default_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.iter().any(|p| p.name == "groq"));
        assert!(providers.iter().any(|p| p.name == "openrouter"));
    }

    #[test]
    fn test_llm_client_initialization() {
        init_llm_client();
        let providers = get_available_providers();
        assert!(providers.len() >= 2);
    }
}
