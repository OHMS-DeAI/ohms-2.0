use candid::Principal;
use ic_cdk::api::{caller, time};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Guards;

impl Guards {
    pub fn require_caller_authenticated() -> Result<(), String> {
        let caller = caller();
        if caller == Principal::anonymous() {
            return Err("Authentication required".to_string());
        }
        Ok(())
    }

    pub fn validate_msg_id(msg_id: &str) -> Result<(), String> {
        if msg_id.is_empty() || msg_id.len() > 64 {
            return Err("Invalid msg_id format".to_string());
        }

        if !msg_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err("msg_id contains invalid characters".to_string());
        }

        Ok(())
    }

    pub fn require_model_canister() -> Result<(), String> {
        let caller = caller();
        if caller == Principal::anonymous() {
            return Err("Model canister authentication required".to_string());
        }

        if let Some(expected) = option_env!("OHMS_MODEL_CANISTER_ID") {
            if caller.to_text() != expected {
                return Err("Caller is not recognized as the model canister".to_string());
            }
        }

        Ok(())
    }
}
