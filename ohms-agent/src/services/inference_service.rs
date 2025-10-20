use candid::{CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::api::time;
use ic_llm::{ChatMessage, Model, Response, Tool};
use serde::{Deserialize, Serialize};

use crate::domain::{
    AgentError, AgentTask, AgentTaskResult, DecodeParams, InferenceRequest, InferenceResponse,
};

use super::{with_state, with_state_mut, CacheService, MemoryService};

pub struct InferenceService;

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct LlmChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    tools: Option<Vec<Tool>>,
}

impl InferenceService {
    pub async fn process_inference(request: InferenceRequest) -> Result<InferenceResponse, String> {
        if with_state(|state| state.binding.is_none()) {
            return Err(AgentError::ModelNotBound.to_string());
        }

        if let Some(cached) = CacheService::try_get(&request) {
            with_state_mut(|state| {
                state.last_inference = time();
            });
            return Ok(cached);
        }

        let start = time();
        with_state_mut(|state| {
            state.active_inference += 1;
            state.metrics.total_inference_requests += 1;
        });

        let generation_result = Self::run_generation(&request.prompt, &request.decode_params).await;

        let elapsed_ns = time().saturating_sub(start);
        let inference_time_ms = elapsed_ns / 1_000_000;
        let mut maybe_response: Option<InferenceResponse> = None;
        let mut failure: Option<String> = None;

        match generation_result {
            Ok(generated_text) => {
                let tokens = Self::tokenize_text(&generated_text);
                let response = InferenceResponse {
                    generated_text: generated_text.clone(),
                    tokens,
                    inference_time_ms,
                    cache_hits: 0,
                    cache_misses: 1,
                };
                CacheService::insert(&request, response.clone());
                MemoryService::record_interaction(&request.prompt, &generated_text, 0.35);
                maybe_response = Some(response);
            }
            Err(err) => {
                failure = Some(err);
            }
        }

        with_state_mut(|state| {
            state.active_inference = state.active_inference.saturating_sub(1);
            state.last_inference = time();
            if maybe_response.is_some() {
                let total = state.metrics.total_inference_requests;
                state.metrics.average_latency_ms = if total == 0 {
                    0.0
                } else {
                    ((state.metrics.average_latency_ms * (total.saturating_sub(1) as f64))
                        + inference_time_ms as f64)
                        / total as f64
                };
            }
        });

        match maybe_response {
            Some(response) => Ok(response),
            None => Err(failure.unwrap_or_else(|| "IC LLM generation failed".to_string())),
        }
    }

    pub async fn execute_task(agent_id: &str, task: &AgentTask) -> Result<AgentTaskResult, String> {
        if with_state(|state| !state.agents.contains_key(agent_id)) {
            return Err(AgentError::InvalidInstruction("agent not found".to_string()).to_string());
        }

        let decode = with_state(|state| DecodeParams {
            max_tokens: Some(state.config.max_tokens.min(4096)),
            temperature: Some(0.65),
            top_p: Some(0.9),
            top_k: Some(40),
            repetition_penalty: Some(1.05),
        });

        let request = InferenceRequest {
            seed: with_state_mut(|state| state.next_random_u64()),
            prompt: format!(
                "{}\nContext:{}",
                task.description,
                Self::format_context(&task.context)
            ),
            decode_params: decode,
            msg_id: format!("task:{}", task.task_id),
        };

        let start = time();
        let inference = Self::process_inference(request).await?;
        let elapsed = time().saturating_sub(start) / 1_000_000;

        Ok(AgentTaskResult {
            task_id: task.task_id.clone(),
            success: true,
            result: inference.generated_text.clone(),
            tokens_used: inference.tokens.len() as u64,
            execution_time_ms: elapsed,
            error_message: None,
        })
    }

    async fn run_generation(prompt: &str, params: &DecodeParams) -> Result<String, String> {
        let messages = vec![
            ChatMessage::System {
                content: Self::build_system_prompt(params),
            },
            ChatMessage::User {
                content: prompt.to_string(),
            },
        ];

        let response = Self::invoke_ic_llm(Model::Llama3_1_8B, messages).await?;
        response
            .message
            .content
            .ok_or_else(|| "IC LLM returned empty content".to_string())
    }

    fn tokenize_text(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        for word in text.split(|c: char| !c.is_alphanumeric()) {
            if word.len() <= 2 {
                continue;
            }
            let normalized = word.to_lowercase();
            if !normalized.is_empty() {
                tokens.push(normalized);
            }
        }
        if tokens.is_empty() {
            tokens.push("general".to_string());
        }
        tokens
    }

    async fn invoke_ic_llm(model: Model, messages: Vec<ChatMessage>) -> Result<Response, String> {
        let llm_canister = Principal::from_text("w36hm-eqaaa-aaaal-qr76a-cai")
            .map_err(|err| format!("invalid IC LLM canister id: {err}"))?;

        let request = LlmChatRequest {
            model: model.to_string(),
            messages,
            tools: None,
        };

        let result: CallResult<(Response,)> = call(llm_canister, "v1_chat", (request,)).await;
        match result {
            Ok((response,)) => Ok(response),
            Err((code, msg)) => Err(format!("IC LLM call failed ({code:?}): {msg}")),
        }
    }

    fn build_system_prompt(params: &DecodeParams) -> String {
        let temperature = params.temperature.unwrap_or(0.7);
        let top_p = params.top_p.unwrap_or(0.9);
        let top_k = params.top_k.unwrap_or(40);
        let max_tokens = params.max_tokens.unwrap_or(1024);
        let repetition_penalty = params.repetition_penalty.unwrap_or(1.05);

        format!(
            "You are the OHMS autonomous reasoning core. Produce detailed, executable guidance with concrete actions. \
Respect temperature {temperature:.2}, top-p {top_p:.2}, top-k {top_k}, repetition penalty {repetition_penalty:.2}, \
and keep the response within {max_tokens} tokens. Highlight assumptions, risks, and next steps when relevant."
        )
    }

    fn format_context(context: &[(String, String)]) -> String {
        if context.is_empty() {
            return "{}".to_string();
        }
        let parts: Vec<String> = context
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        parts.join(", ")
    }
}
