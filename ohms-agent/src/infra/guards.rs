use candid::Principal;
use ic_cdk::api::{caller, time};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::services::with_state;

const RATE_LIMIT_WINDOW_NS: u64 = 60 * 1_000_000_000; // 60 seconds
const RATE_LIMIT_REQUESTS: u32 = 60; // 60 requests per minute

thread_local! {
    static RATE_LIMITS: RefCell<HashMap<Principal, RateLimitBucket>> = RefCell::new(HashMap::new());
}

pub struct Guards;

impl Guards {
    pub fn require_caller_authenticated() -> Result<(), String> {
        let caller = caller();
        if caller == Principal::anonymous() {
            Err("anonymous callers are not permitted".to_string())
        } else {
            Ok(())
        }
    }

    pub fn rate_limit_check() -> Result<(), String> {
        let principal = caller();
        RATE_LIMITS.with(|map| {
            let mut buckets = map.borrow_mut();
            let now = time();
            let bucket = buckets.entry(principal).or_insert_with(|| RateLimitBucket {
                last_reset: now,
                count: 0,
            });

            if now.saturating_sub(bucket.last_reset) > RATE_LIMIT_WINDOW_NS {
                bucket.count = 0;
                bucket.last_reset = now;
            }

            if bucket.count >= RATE_LIMIT_REQUESTS {
                Err("rate limit exceeded: 60 requests per minute".to_string())
            } else {
                bucket.count += 1;
                Ok(())
            }
        })
    }

    pub fn validate_prompt_length(prompt: &str) -> Result<(), String> {
        let estimated_tokens = prompt.split_whitespace().count();
        let max_tokens = with_state(|state| state.config.max_tokens);
        if estimated_tokens as u32 > max_tokens {
            Err(format!(
                "prompt too long: estimated {} tokens exceeds limit {}",
                estimated_tokens, max_tokens
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_msg_id(msg_id: &str) -> Result<(), String> {
        if msg_id.is_empty() {
            return Err("message id cannot be empty".to_string());
        }
        if msg_id.len() > 128 {
            return Err("message id too long".to_string());
        }
        if !msg_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':'))
        {
            return Err(
                "message id may only contain alphanumeric characters, '-', '_' or ':'".to_string(),
            );
        }
        Ok(())
    }
}

struct RateLimitBucket {
    last_reset: u64,
    count: u32,
}
