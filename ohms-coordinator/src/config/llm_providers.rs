use ohms_shared::llm_client::{LlmClient, LlmProvider, ProviderConfig};
use std::cell::RefCell;

thread_local! {
    static LLM_CLIENT: RefCell<LlmClient> = RefCell::new(LlmClient::new());
}

/// Get default provider configurations
pub fn get_default_providers() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig::groq(),
        ProviderConfig::together_ai(),
        ProviderConfig::openrouter(),
    ]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_providers() {
        let providers = get_default_providers();
        assert_eq!(providers.len(), 3);
        assert!(providers.iter().any(|p| p.name == "groq"));
        assert!(providers.iter().any(|p| p.name == "together_ai"));
        assert!(providers.iter().any(|p| p.name == "openrouter"));
    }

    #[test]
    fn test_llm_client_initialization() {
        init_llm_client();
        let providers = get_available_providers();
        assert!(providers.len() >= 3);
    }
}

