use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ohms_shared::llm_client::{
    ChatCompletionResponse, LlmProvider, LlmRequest, LlmResponse,
    ProviderConfig,
};
use serde::Serialize;

const MAX_RESPONSE_BYTES: u64 = 2_000_000;
const CYCLES_PER_CALL: u128 = 50_000_000_000;
const MAX_RETRIES: u8 = 3;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct HttpOutcallError {
    pub message: String,
    pub retry_after: Option<u64>,
}

pub struct HttpOutcallService;

impl HttpOutcallService {
    /// Make LLM inference call with retry logic and circuit breaker
    pub async fn make_llm_call(
        request: &LlmRequest,
        provider: &LlmProvider,
        api_key: Option<String>,
    ) -> Result<LlmResponse, String> {
        let config = ohms_shared::llm_client::LlmClient::get_provider_config(provider);
        
        let mut last_error = String::new();
        
        for attempt in 0..MAX_RETRIES {
            match Self::try_llm_call(request, &config, api_key.clone()).await {
                Ok(response) => {
                    return Ok(LlmResponse {
                        content: response.choices.first()
                            .map(|c| c.message.content.clone())
                            .unwrap_or_default(),
                        tokens_used: response.usage.total_tokens,
                        provider: provider.name(),
                        model: request.model.clone(),
                        finish_reason: response.choices.first()
                            .map(|c| c.finish_reason.clone())
                            .unwrap_or_else(|| "unknown".to_string()),
                        cached: false,
                    });
                }
                Err(e) => {
                    last_error = format!("Attempt {}/{}: {}", attempt + 1, MAX_RETRIES, e);
                    
                    if attempt < MAX_RETRIES - 1 {
                        ic_cdk::api::call::call_raw(
                            Principal::management_canister(),
                            "raw_rand",
                            &[],
                            0,
                        )
                        .await
                        .ok();
                    }
                }
            }
        }
        
        Err(format!("Failed after {} retries: {}", MAX_RETRIES, last_error))
    }

    /// Single attempt at LLM call
    async fn try_llm_call(
        request: &LlmRequest,
        config: &ProviderConfig,
        api_key: Option<String>,
    ) -> Result<ChatCompletionResponse, String> {
        let chat_request = request.to_chat_request();
        let body = serde_json::to_string(&chat_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let mut headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ];

        if let Some(key) = api_key {
            headers.push(HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", key),
            });
        }

        let http_request_arg = CanisterHttpRequestArgument {
            url: config.base_url.clone(),
            method: HttpMethod::POST,
            body: Some(body.into_bytes()),
            max_response_bytes: Some(MAX_RESPONSE_BYTES),
            transform: Some(TransformContext::from_name(
                "transform_http_response".to_string(),
                vec![],
            )),
            headers,
        };

        let http_response = http_request(http_request_arg, CYCLES_PER_CALL)
            .await
            .map_err(|(code, msg)| format!("HTTP request failed: {:?} - {}", code, msg))?;

        let response = http_response.0;

        let status_num: u32 = response.status.0.clone().try_into()
            .map_err(|_| "Invalid status code".to_string())?;

        if status_num >= 200 && status_num < 300 {
            let body_str = String::from_utf8(response.body)
                .map_err(|e| format!("Failed to decode response: {}", e))?;
            
            serde_json::from_str::<ChatCompletionResponse>(&body_str)
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("HTTP error {}: {:?}", response.status, String::from_utf8_lossy(&response.body)))
        }
    }

}

/// Transform HTTP response to remove headers for consensus
#[ic_cdk_macros::query]
fn transform_http_response(args: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: args.response.status,
        body: args.response.body,
        headers: vec![],
    }
}

