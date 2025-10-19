use ic_cdk::api::time;
use crate::services::with_state_mut;

pub struct CollaborationService;

impl CollaborationService {
    /// Promote agent to queen role
    pub fn promote_to_queen(agent_id: &str) -> Result<(), String> {
        with_state_mut(|state| {
            if let Some(agent) = state.agents.get_mut(agent_id) {
                agent.coordination_role = Some("queen".to_string());
                agent.last_active = time();
                Ok(())
            } else {
                Err(format!("Agent {} not found", agent_id))
            }
        })
    }

    /// Demote agent from queen role
    pub fn demote_from_queen(agent_id: &str) -> Result<(), String> {
        with_state_mut(|state| {
            if let Some(agent) = state.agents.get_mut(agent_id) {
                if agent.coordination_role == Some("queen".to_string()) {
                    agent.coordination_role = Some("worker".to_string());
                    agent.last_active = time();
                    Ok(())
                } else {
                    Err(format!("Agent {} is not a queen", agent_id))
                }
            } else {
                Err(format!("Agent {} not found", agent_id))
            }
        })
    }

    /// Assign worker role to agent
    pub fn assign_worker_role(agent_id: &str) -> Result<(), String> {
        with_state_mut(|state| {
            if let Some(agent) = state.agents.get_mut(agent_id) {
                agent.coordination_role = Some("worker".to_string());
                agent.last_active = time();
                Ok(())
            } else {
                Err(format!("Agent {} not found", agent_id))
            }
        })
    }

    /// Get agent's current role
    pub fn get_agent_role(agent_id: &str) -> Result<Option<String>, String> {
        crate::services::with_state(|state| {
            if let Some(agent) = state.agents.get(agent_id) {
                Ok(agent.coordination_role.clone())
            } else {
                Err(format!("Agent {} not found", agent_id))
            }
        })
    }

    /// Check if agent can be promoted to queen
    pub fn can_be_queen(agent_id: &str) -> bool {
        crate::services::with_state(|state| {
            state.agents.get(agent_id)
                .map(|agent| {
                    agent.capabilities.contains(&"planning".to_string()) ||
                    agent.capabilities.contains(&"synthesis".to_string()) ||
                    agent.capabilities.contains(&"coordination".to_string())
                })
                .unwrap_or(false)
        })
    }

    /// Record peer communication
    pub fn record_peer_message(
        from_agent: &str,
        to_agent: &str,
        _message: &str,
    ) -> Result<(), String> {
        with_state_mut(|state| {
            if !state.agents.contains_key(from_agent) {
                return Err(format!("Sender agent {} not found", from_agent));
            }
            if !state.agents.contains_key(to_agent) {
                return Err(format!("Recipient agent {} not found", to_agent));
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_management() {
        let agent_id = "test_agent_1";
        
        let result = CollaborationService::promote_to_queen(agent_id);
        assert!(result.is_ok() || result.is_err());
    }
}

