use ic_cdk::api::time;

use crate::domain::{
    AgentError, AgentTask, AgentTaskResult, DecodeParams, InferenceRequest, InferenceResponse,
};

use super::{with_state, with_state_mut, CacheService, MemoryService};

pub struct InferenceService;

impl InferenceService {
    pub async fn process_inference(request: InferenceRequest) -> Result<InferenceResponse, String> {
        if with_state(|state| state.binding.is_none()) {
            return Err(AgentError::ModelNotBound.to_string());
        }

        if let Some(mut cached) = CacheService::try_get(&request) {
            with_state_mut(|state| {
                state.last_inference = time();
            });
            cached.cache_hits = cached.cache_hits.saturating_add(1);
            return Ok(cached);
        }

        let start = time();
        with_state_mut(|state| {
            state.active_inference += 1;
            state.metrics.total_inference_requests += 1;
        });

        let response = Self::run_generation(&request.prompt, &request.decode_params);

        let elapsed_ns = time().saturating_sub(start);
        let inference_time_ms = elapsed_ns / 1_000_000;
        let final_response = InferenceResponse {
            generated_text: response.generated_text,
            tokens: response.tokens,
            inference_time_ms,
            cache_hits: 0,
            cache_misses: 1,
        };

        CacheService::insert(&request, final_response.clone());
        MemoryService::record_interaction(&request.prompt, &final_response.generated_text, 0.35);

        with_state_mut(|state| {
            state.active_inference = state.active_inference.saturating_sub(1);
            state.last_inference = time();
            let total = state.metrics.total_inference_requests;
            state.metrics.average_latency_ms = if total == 0 {
                0.0
            } else {
                ((state.metrics.average_latency_ms * (total.saturating_sub(1) as f64))
                    + inference_time_ms as f64)
                    / total as f64
            };
        });

        Ok(final_response)
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

    fn run_generation(prompt: &str, params: &DecodeParams) -> InferenceResponse {
        let tokens = Self::tokenize(prompt);
        let summary = Self::summarize(prompt);
        let insights = Self::derive_insights(&tokens);
        let plan = Self::build_plan(&tokens, params);

        let generated_text = format!(
            "Summary:\n{}\n\nKey Insights:\n{}\n\nRecommended Plan:\n{}",
            summary,
            insights.join("\n"),
            plan.join("\n"),
        );

        InferenceResponse {
            generated_text,
            tokens,
            inference_time_ms: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn tokenize(prompt: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        for word in prompt.split(|c: char| !c.is_alphanumeric()) {
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

    fn summarize(prompt: &str) -> String {
        let sentences: Vec<&str> = prompt
            .split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .collect();
        match sentences.len() {
            0 => "No context provided".to_string(),
            1 => sentences[0].trim().to_string(),
            _ => {
                let first = sentences[0].trim();
                let last = sentences.last().unwrap().trim();
                format!("{} ... {}", first, last)
            }
        }
    }

    fn derive_insights(tokens: &[String]) -> Vec<String> {
        let mut counts = std::collections::HashMap::new();
        for token in tokens {
            *counts.entry(token).or_insert(0usize) += 1;
        }
        let mut top: Vec<_> = counts.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1));
        top.into_iter()
            .take(5)
            .map(|(token, count)| format!("- `{}` appears {} times", token, count))
            .collect()
    }

    fn build_plan(tokens: &[String], params: &DecodeParams) -> Vec<String> {
        let mut plan = Vec::new();
        if tokens.is_empty() {
            return vec!["Analyze requirements".to_string()];
        }

        let unique: std::collections::HashSet<_> = tokens.iter().collect();
        let mut idx = 1;
        for token in unique.iter().take(4) {
            plan.push(format!("{}. Investigate `{}` impact", idx, token));
            idx += 1;
        }
        let temperature = params.temperature.unwrap_or(0.7);
        let max_tokens = params.max_tokens.unwrap_or(2048);
        plan.push(format!(
            "{}. Allocate up to {} tokens with temperature {:.2}",
            idx, max_tokens, temperature
        ));
        plan
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
