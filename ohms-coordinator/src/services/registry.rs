use crate::domain::*;
use crate::services::{with_state, with_state_mut};
use base64::{engine::general_purpose, Engine as _};
use ic_cdk::api::time;
use ohms_shared::{current_time_seconds, ComponentHealth, ModelManifest, ModelState, SystemHealth};
use sha2::{Digest, Sha256};
use std::cmp::Reverse;
use std::collections::HashMap;

pub struct RegistryService;

impl RegistryService {
    pub fn register_model(manifest: ModelManifest) -> Result<(), String> {
        let now = time();
        with_state_mut(|state| {
            let entry = state
                .models
                .entry(manifest.model_id.clone())
                .or_insert_with(|| RegisteredModel {
                    manifest: manifest.clone(),
                    registered_at: now,
                    last_updated: now,
                });

            entry.manifest = manifest.clone();
            entry.last_updated = now;

            state.metrics.total_models = state.models.len() as u32;
            state.metrics.ready_models = state
                .models
                .values()
                .filter(|m| is_model_ready(&m.manifest.state))
                .count() as u32;
            state.metrics.last_activity = now;
            Ok(())
        })
    }

    pub fn remove_model(model_id: &str) -> Result<(), String> {
        let now = time();
        with_state_mut(|state| {
            if state.models.remove(model_id).is_none() {
                return Err(format!("Model {} not registered", model_id));
            }

            state.metrics.total_models = state.models.len() as u32;
            state.metrics.ready_models = state
                .models
                .values()
                .filter(|m| is_model_ready(&m.manifest.state))
                .count() as u32;
            state.metrics.last_activity = now;
            Ok(())
        })
    }

    pub fn get_model(model_id: &str) -> Option<RegisteredModel> {
        with_state(|state| state.models.get(model_id).cloned())
    }

    pub fn list_models() -> Vec<RegisteredModel> {
        with_state(|state| state.models.values().cloned().collect())
    }

    pub fn select_model_for_requirements(requirements: &[String]) -> Option<RegisteredModel> {
        with_state(|state| {
            let mut candidates: Vec<&RegisteredModel> = state.models.values().collect();
            candidates.sort_by_key(|model| Reverse(model.last_updated));

            for candidate in candidates {
                if !is_model_ready(&candidate.manifest.state) {
                    continue;
                }

                if model_matches_requirements(&candidate.manifest, requirements) {
                    return Some(candidate.clone());
                }
            }

            None
        })
    }

    pub async fn register_agent(registration: AgentRegistration) -> Result<String, String> {
        let now = time();

        if !with_state(|state| state.models.contains_key(&registration.model_id)) {
            return Err(format!(
                "Model {} is not registered with the coordinator",
                registration.model_id
            ));
        }

        let agent_id =
            Self::generate_agent_id(&registration.agent_principal, &registration.model_id);

        let mut agent_reg = registration;
        agent_reg.agent_id = agent_id.clone();
        agent_reg.registered_at = now;
        agent_reg.last_seen = now;
        agent_reg.health_score = 1.0; // Start with perfect health

        with_state_mut(|state| {
            state.agents.insert(agent_id.clone(), agent_reg.clone());

            // Initialize routing stats for this agent
            let stats = RoutingStats {
                agent_id: agent_id.clone(),
                total_requests: 0,
                success_rate: 1.0,
                average_response_time_ms: 0.0,
                capability_scores: agent_reg
                    .capabilities
                    .iter()
                    .map(|cap| (cap.clone(), 1.0))
                    .collect(),
            };
            state.routing_stats.insert(agent_id.clone(), stats);

            state.metrics.total_agents += 1;
            state.metrics.last_activity = now;
        });

        Ok(agent_id)
    }

    pub fn get_agent(agent_id: &str) -> Result<AgentRegistration, String> {
        with_state(|state| {
            state
                .agents
                .get(agent_id)
                .cloned()
                .ok_or_else(|| format!("Agent not found: {}", agent_id))
        })
    }

    pub fn list_agents() -> Vec<AgentRegistration> {
        with_state(|state| state.agents.values().cloned().collect())
    }

    pub fn update_agent_health(agent_id: String, health_score: f32) -> Result<(), String> {
        let now = time();
        let clamped_score = health_score.max(0.0).min(1.0);

        with_state_mut(|state| {
            if let Some(agent) = state.agents.get_mut(&agent_id) {
                agent.health_score = clamped_score;
                agent.last_seen = now;
                Ok(())
            } else {
                Err(format!("Agent not found: {}", agent_id))
            }
        })
    }

    pub fn get_agents_by_capability(capability: &str) -> Vec<AgentRegistration> {
        with_state(|state| {
            state
                .agents
                .values()
                .filter(|agent| agent.capabilities.contains(&capability.to_string()))
                .cloned()
                .collect()
        })
    }

    pub fn get_healthy_agents(min_health: f32) -> Vec<AgentRegistration> {
        with_state(|state| {
            state
                .agents
                .values()
                .filter(|agent| agent.health_score >= min_health)
                .cloned()
                .collect()
        })
    }

    pub fn get_health() -> SystemHealth {
        with_state(|state| {
            let total_agents = state.agents.len() as u32;
            let active_agents = state
                .agents
                .values()
                .filter(|agent| agent.health_score > 0.6)
                .count() as u32;

            let total_models = state.models.len() as u32;
            let ready_models = state
                .models
                .values()
                .filter(|m| is_model_ready(&m.manifest.state))
                .count() as u32;

            let total_agent_creations = state.metrics.total_agent_creations;
            let active_instructions = state.instruction_requests.len() as u32;
            let coordination_sessions = state
                .coordination_sessions
                .as_ref()
                .map(|sessions| sessions.len() as u32)
                .unwrap_or(0);

            let memory_usage_mb = (ic_cdk::api::instruction_counter() / 1_000_000) as f32;

            let status = derive_coordinator_status(
                total_agents,
                active_agents,
                ready_models,
                active_instructions,
                memory_usage_mb,
            );

            let mut metrics = HashMap::new();
            metrics.insert("agents.total".to_string(), total_agents.to_string());
            metrics.insert("agents.active".to_string(), active_agents.to_string());
            metrics.insert("models.total".to_string(), total_models.to_string());
            metrics.insert("models.ready".to_string(), ready_models.to_string());
            metrics.insert(
                "agent_creations.total".to_string(),
                total_agent_creations.to_string(),
            );
            metrics.insert(
                "instructions.active".to_string(),
                active_instructions.to_string(),
            );
            metrics.insert(
                "routing.total_requests".to_string(),
                state.metrics.total_routes.to_string(),
            );
            metrics.insert(
                "routing.avg_time_ms".to_string(),
                format!("{:.2}", state.metrics.average_routing_time_ms),
            );
            metrics.insert(
                "dedup.cache_size".to_string(),
                state.dedup_cache.len().to_string(),
            );
            metrics.insert(
                "coordination.sessions".to_string(),
                coordination_sessions.to_string(),
            );
            metrics.insert(
                "metrics.last_activity".to_string(),
                state.metrics.last_activity.to_string(),
            );
            metrics.insert(
                "swarm.topology".to_string(),
                format!("{:?}", state.config.swarm.topology),
            );

            SystemHealth {
                canister_id: ic_cdk::id(),
                status,
                uptime_seconds: ic_cdk::api::time() / 1_000_000_000,
                memory_usage_mb,
                last_update: current_time_seconds(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                metrics,
            }
        })
    }

    fn generate_agent_id(principal: &str, model_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(principal.as_bytes());
        hasher.update(model_id.as_bytes());
        hasher.update(time().to_be_bytes());
        let hash = hasher.finalize();
        format!("agent_{}", general_purpose::STANDARD.encode(&hash[..8]))
    }
}

fn is_model_ready(state: &ModelState) -> bool {
    matches!(state, ModelState::Active | ModelState::Pending)
}

fn derive_coordinator_status(
    total_agents: u32,
    active_agents: u32,
    ready_models: u32,
    active_instructions: u32,
    memory_usage_mb: f32,
) -> ComponentHealth {
    if total_agents == 0 || ready_models == 0 {
        return ComponentHealth::Degraded;
    }

    if active_agents == 0 && active_instructions > 0 {
        return ComponentHealth::Unhealthy;
    }

    if memory_usage_mb > 1500.0 {
        return ComponentHealth::Degraded;
    }

    let active_ratio = if total_agents == 0 {
        0.0
    } else {
        active_agents as f32 / (total_agents as f32)
    };

    if active_ratio < 0.5 {
        ComponentHealth::Degraded
    } else {
        ComponentHealth::Healthy
    }
}

fn model_matches_requirements(manifest: &ModelManifest, requirements: &[String]) -> bool {
    if requirements.is_empty() {
        return true;
    }

    let quantization_label = match &manifest.quantization.format {
        ohms_shared::QuantizationFormat::NOVAQ => "novaq".to_string(),
        ohms_shared::QuantizationFormat::GGUF => "gguf".to_string(),
        ohms_shared::QuantizationFormat::Custom(value) => value.to_lowercase(),
    };

    requirements.iter().all(|requirement| {
        let req_lower = requirement.to_lowercase();

        if quantization_label.contains(&req_lower) {
            return true;
        }

        if manifest.metadata.iter().any(|(key, value)| {
            key.to_lowercase().contains(&req_lower) || value.to_lowercase().contains(&req_lower)
        }) {
            return true;
        }

        manifest
            .quantization
            .notes
            .as_ref()
            .map(|notes| notes.to_lowercase().contains(&req_lower))
            .unwrap_or(false)
    })
}
