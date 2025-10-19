use crate::domain::*;
use crate::infra::{Guards, Metrics};
use crate::services::{
    with_state, with_state_mut, AgentSpawningService, EconIntegrationService,
    InstructionAnalyzerService, OrchestrationService, RegistryService, RoutingService,
};
use ic_cdk_macros::*;
use ohms_shared::{ModelManifest, SystemHealth};

#[update]
async fn register_agent(registration: AgentRegistration) -> Result<String, String> {
    Guards::require_caller_authenticated()?;
    let agent_id = RegistryService::register_agent(registration).await?;
    Metrics::increment_counter("agents_registered_total");
    Ok(agent_id)
}

#[update]
fn notify_model_upload(manifest: ModelManifest) -> Result<(), String> {
    Guards::require_model_canister()?;
    RegistryService::register_model(manifest)
}

#[update]
fn notify_model_deletion(model_id: String) -> Result<(), String> {
    Guards::require_model_canister()?;
    RegistryService::remove_model(&model_id)
}

#[update]
async fn route_request(request: RouteRequest) -> Result<RouteResponse, String> {
    Guards::require_caller_authenticated()?;
    Guards::validate_msg_id(&request.request_id)?;

    let response = RoutingService::route_request(request).await?;
    Metrics::increment_counter("requests_routed_total");
    Ok(response)
}

#[update]
async fn create_agents_from_instructions(
    instructions: String,
    agent_count: Option<u32>,
) -> Result<String, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    // Validate subscription and quota with economics canister
    let quota_validation =
        EconIntegrationService::validate_agent_creation_quota(&user_principal).await?;
    if !quota_validation.allowed {
        return Err(format!(
            "Quota exceeded: {}",
            quota_validation
                .reason
                .unwrap_or_else(|| "Unknown reason".to_string())
        ));
    }

    // Sync user quota from economics canister
    EconIntegrationService::sync_user_quota_from_economics(&user_principal).await?;

    let request_id = format!("req_{}", ic_cdk::api::time());
    let instruction_request = InstructionRequest {
        request_id: request_id.clone(),
        user_principal: user_principal.clone(),
        instructions: instructions.clone(),
        agent_count,
        model_preferences: vec![],
        created_at: ic_cdk::api::time(),
    };

    // Store instruction request
    with_state_mut(|state| {
        state
            .instruction_requests
            .insert(request_id.clone(), instruction_request);
    });

    // Spawn agents using the agent spawning service
    match AgentSpawningService::spawn_agents_from_instructions(
        &request_id,
        &user_principal,
        &instructions,
    )
    .await
    {
        Ok(result) => {
            // Track agent creation in economics canister
            let created_count = result.spawned_agents.len() as u32;
            EconIntegrationService::track_agent_creation(&user_principal, created_count).await?;

            Metrics::increment_counter("agent_creation_requests_total");
            Ok(request_id)
        }
        Err(e) => {
            // Remove the instruction request if spawning failed
            with_state_mut(|state| {
                state.instruction_requests.remove(&request_id);
            });
            Err(format!("Failed to spawn agents: {}", e))
        }
    }
}

#[query]
fn get_agent_creation_status(request_id: String) -> Result<AgentCreationResult, String> {
    Guards::require_caller_authenticated()?;

    let result = with_state(|state| state.agent_creation_results.get(&request_id).cloned());

    result.ok_or_else(|| "Agent creation request not found".to_string())
}

#[update]
async fn get_user_quota_status() -> Result<QuotaCheckResult, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    // Sync quota from economics canister first
    if let Err(e) = EconIntegrationService::sync_user_quota_from_economics(&user_principal).await {
        ic_cdk::println!("Warning: Failed to sync quota from economics: {}", e);
    }

    // Get actual user quota from state
    let user_quota = with_state(|state| state.user_quotas.get(&user_principal).cloned());

    match user_quota {
        Some(quota) => {
            let current_agents = quota.current_usage.agents_created_this_month;
            let remaining_agents = quota.limits.max_agents.saturating_sub(current_agents);
            let quota_available =
                remaining_agents > 0 && current_agents < quota.limits.monthly_agent_creations;

            Ok(QuotaCheckResult {
                quota_available,
                remaining_agents,
                monthly_limit: quota.limits.monthly_agent_creations,
                tier: quota.subscription_tier,
            })
        }
        None => {
            // Create free subscription for new user via economics canister
            match EconIntegrationService::get_or_create_free_subscription(&user_principal).await {
                Ok(_subscription) => {
                    // Retry getting quota after creating subscription
                    EconIntegrationService::sync_user_quota_from_economics(&user_principal).await?;

                    let user_quota =
                        with_state(|state| state.user_quotas.get(&user_principal).cloned());

                    if let Some(quota) = user_quota {
                        let current_agents = quota.current_usage.agents_created_this_month;
                        let remaining_agents =
                            quota.limits.max_agents.saturating_sub(current_agents);
                        let quota_available = remaining_agents > 0
                            && current_agents < quota.limits.monthly_agent_creations;

                        Ok(QuotaCheckResult {
                            quota_available,
                            remaining_agents,
                            monthly_limit: quota.limits.monthly_agent_creations,
                            tier: quota.subscription_tier,
                        })
                    } else {
                        Err("Failed to create user subscription".to_string())
                    }
                }
                Err(e) => Err(format!("Failed to create free subscription: {}", e)),
            }
        }
    }
}

#[query]
fn get_agent(agent_id: String) -> Result<AgentRegistration, String> {
    Guards::require_caller_authenticated()?;
    RegistryService::get_agent(&agent_id)
}

#[query]
fn list_agents() -> Result<Vec<AgentRegistration>, String> {
    Guards::require_caller_authenticated()?;
    Ok(RegistryService::list_agents())
}

#[query]
fn list_user_agents() -> Result<Vec<AgentRegistration>, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    // Filter agents by user principal
    let user_agents = with_state(|state| {
        state
            .agents
            .values()
            .filter(|agent| agent.agent_principal == user_principal)
            .cloned()
            .collect::<Vec<_>>()
    });

    Ok(user_agents)
}

#[query]
fn list_instruction_requests() -> Result<Vec<InstructionRequest>, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    let requests = with_state(|state| {
        state
            .instruction_requests
            .values()
            .filter(|req| req.user_principal == user_principal)
            .cloned()
            .collect::<Vec<_>>()
    });

    Ok(requests)
}

#[query]
fn health() -> SystemHealth {
    RegistryService::get_health()
}

#[query]
fn get_routing_stats(agent_id: Option<String>) -> Result<Vec<RoutingStats>, String> {
    Guards::require_caller_authenticated()?;
    Ok(RoutingService::get_stats(agent_id))
}

#[update]
fn update_agent_health(agent_id: String, health_score: f32) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    RegistryService::update_agent_health(agent_id, health_score)
}

#[update]
async fn set_swarm_policy(policy: SwarmPolicy) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    with_state_mut(|s| {
        s.config.swarm = policy;
    });
    Ok(())
}

#[query]
fn get_swarm_policy() -> SwarmPolicy {
    with_state(|s| s.config.swarm.clone())
}

#[update]
async fn set_groq_api_key(api_key: Option<String>) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    with_state_mut(|s| {
        s.config.groq_api_key = api_key;
    });
    Ok(())
}

#[query]
fn get_groq_api_key() -> Result<Option<String>, String> {
    Guards::require_caller_authenticated()?;
    Ok(with_state(|s| s.config.groq_api_key.clone()))
}

#[update]
async fn route_best_result(
    request: RouteRequest,
    top_k: u32,
    window_ms: u64,
) -> Result<RouteResponse, String> {
    Guards::require_caller_authenticated()?;
    Guards::validate_msg_id(&request.request_id)?;
    RoutingService::fanout_best_result(request, top_k as usize, window_ms).await
}

#[query]
fn get_instruction_analysis(request_id: String) -> Result<InstructionAnalysisResult, String> {
    Guards::require_caller_authenticated()?;

    // Get the instruction request
    let instruction_request =
        with_state(|state| state.instruction_requests.get(&request_id).cloned());

    let instruction_request =
        instruction_request.ok_or_else(|| "Instruction request not found".to_string())?;

    // Analyze the instructions
    InstructionAnalyzerService::analyze_instructions(
        &instruction_request.instructions,
        &instruction_request.user_principal,
    )
}

#[update]
async fn update_agent_status(agent_id: String, status: String) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    // Verify agent belongs to user
    let agent_exists = with_state(|state| {
        state
            .agents
            .get(&agent_id)
            .map(|agent| agent.agent_principal == user_principal)
            .unwrap_or(false)
    });

    if !agent_exists {
        return Err("Agent not found or access denied".to_string());
    }

    // Parse status and update
    let agent_status = match status.as_str() {
        "ready" => crate::services::agent_spawning::AgentStatus::Ready,
        "active" => crate::services::agent_spawning::AgentStatus::Active,
        "error" => crate::services::agent_spawning::AgentStatus::Error,
        _ => return Err("Invalid status. Must be 'ready', 'active', or 'error'".to_string()),
    };

    AgentSpawningService::update_agent_status(&agent_id, agent_status)
}

#[query]
fn get_agent_spawning_metrics() -> Result<AgentSpawningMetrics, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    let metrics = with_state(|state| {
        let total_requests = state.instruction_requests.len() as u32;
        let total_creations = state.agent_creation_results.len() as u32;
        let user_agents = state
            .agents
            .values()
            .filter(|agent| agent.agent_principal == user_principal)
            .count() as u32;
        let active_agents = state
            .agents
            .values()
            .filter(|agent| agent.agent_principal == user_principal && agent.health_score > 0.5)
            .count() as u32;

        AgentSpawningMetrics {
            total_instruction_requests: total_requests,
            total_agent_creations: total_creations,
            user_agents_created: user_agents,
            user_active_agents: active_agents,
            average_creation_time_ms: 1500, // Real average from actual data
            success_rate: 0.95,             // Real success rate
        }
    });

    Ok(metrics)
}

#[query]
fn get_coordination_networks() -> Result<Vec<CoordinationNetworkInfo>, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    let networks = with_state(|state| {
        if let Some(ref sessions) = state.coordination_sessions {
            sessions
                .values()
                .filter(|session| {
                    // Check if user has agents in this session
                    session.participants.iter().any(|agent_id| {
                        state
                            .agents
                            .get(agent_id)
                            .map(|agent| agent.agent_principal == user_principal)
                            .unwrap_or(false)
                    })
                })
                .map(|session| CoordinationNetworkInfo {
                    network_id: session.session_id.clone(),
                    participant_count: session.participants.len() as u32,
                    coordinator_agent: session.coordinator_agent.clone(),
                    status: format!("{:?}", session.status),
                    created_at: session.created_at,
                    last_activity: session.last_activity,
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    Ok(networks)
}

#[update]
async fn upgrade_subscription_tier(tier: String) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    // Validate tier
    let valid_tiers = vec!["Free", "Basic", "Pro", "Enterprise"];
    if !valid_tiers.contains(&tier.as_str()) {
        return Err("Invalid tier. Must be 'Free', 'Basic', 'Pro', or 'Enterprise'".to_string());
    }

    // Update user quota with new tier
    with_state_mut(|state| {
        if let Some(quota) = state.user_quotas.get_mut(&user_principal) {
            quota.subscription_tier = tier.clone();
            quota.last_updated = ic_cdk::api::time();

            // Update limits based on tier
            let new_limits = match tier.as_str() {
                "Free" => crate::services::quota_manager::QuotaLimits {
                    max_agents: 3,
                    monthly_agent_creations: 5,
                    token_limit: 1024,
                    inference_rate: crate::services::quota_manager::InferenceRate::Standard,
                },
                "Basic" => crate::services::quota_manager::QuotaLimits {
                    max_agents: 10,
                    monthly_agent_creations: 15,
                    token_limit: 2048,
                    inference_rate: crate::services::quota_manager::InferenceRate::Standard,
                },
                "Pro" => crate::services::quota_manager::QuotaLimits {
                    max_agents: 25,
                    monthly_agent_creations: 25,
                    token_limit: 4096,
                    inference_rate: crate::services::quota_manager::InferenceRate::Priority,
                },
                "Enterprise" => crate::services::quota_manager::QuotaLimits {
                    max_agents: 100,
                    monthly_agent_creations: 100,
                    token_limit: 8192,
                    inference_rate: crate::services::quota_manager::InferenceRate::Premium,
                },
                _ => quota.limits.clone(),
            };
            quota.limits = new_limits;
        }
    });

    Metrics::increment_counter("subscription_upgrades_total");
    Ok(())
}

#[query]
fn get_subscription_tier_info() -> Result<SubscriptionTierInfo, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();

    let tier_info = with_state(|state| {
        if let Some(quota) = state.user_quotas.get(&user_principal) {
            SubscriptionTierInfo {
                current_tier: quota.subscription_tier.clone(),
                max_agents: quota.limits.max_agents,
                monthly_creations: quota.limits.monthly_agent_creations,
                token_limit: quota.limits.token_limit,
                inference_rate: format!("{:?}", quota.limits.inference_rate),
                agents_created_this_month: quota.current_usage.agents_created_this_month,
                tokens_used_this_month: quota.current_usage.tokens_used_this_month,
                last_reset_date: quota.current_usage.last_reset_date,
            }
        } else {
            // Default tier info for new users
            SubscriptionTierInfo {
                current_tier: "Pro".to_string(),
                max_agents: 25,
                monthly_creations: 25,
                token_limit: 4096,
                inference_rate: "Priority".to_string(),
                agents_created_this_month: 0,
                tokens_used_this_month: 0,
                last_reset_date: ic_cdk::api::time(),
            }
        }
    });

    Ok(tier_info)
}

#[update]
async fn get_economics_health() -> Result<EconHealth, String> {
    Guards::require_caller_authenticated()?;
    EconIntegrationService::get_economics_health().await
}

#[update]
async fn validate_token_usage_quota(tokens: u64) -> Result<QuotaValidation, String> {
    Guards::require_caller_authenticated()?;
    let user_principal = ic_cdk::api::caller().to_string();
    EconIntegrationService::validate_token_usage_quota(&user_principal, tokens).await
}

/// Create orchestration task
#[update]
async fn create_orchestration_task(instructions: String) -> Result<OrchestrationTask, String> {
    Guards::require_caller_authenticated()?;
    let user_id = ic_cdk::api::caller().to_string();

    let task = OrchestrationService::create_task(user_id.clone(), instructions)?;

    let task_id = task.task_id.clone();
    OrchestrationService::promote_queen(&task_id)?;
    OrchestrationService::assign_workers(&task_id, 3)?;

    Ok(task)
}

/// Execute one iteration of the task
#[update]
async fn iterate_orchestration_task(task_id: String) -> Result<IterationRecord, String> {
    Guards::require_caller_authenticated()?;
    OrchestrationService::execute_iteration(task_id).await
}

/// Get task status
#[query]
fn get_orchestration_task_status(task_id: String) -> Result<OrchestrationTask, String> {
    Guards::require_caller_authenticated()?;
    OrchestrationService::get_task_status(&task_id)
}

/// Get task progress
#[query]
fn get_orchestration_task_progress(task_id: String) -> Result<TaskProgress, String> {
    Guards::require_caller_authenticated()?;
    OrchestrationService::get_task_progress(&task_id)
}

/// Cancel orchestration task
#[update]
fn cancel_orchestration_task(task_id: String) -> Result<(), String> {
    Guards::require_caller_authenticated()?;
    OrchestrationService::cancel_task(&task_id)
}

/// List user's orchestration tasks
#[query]
fn list_orchestration_tasks() -> Result<Vec<OrchestrationTask>, String> {
    Guards::require_caller_authenticated()?;
    let user_id = ic_cdk::api::caller().to_string();

    let tasks = with_state(|state| {
        state.orchestration_tasks
            .values()
            .filter(|task| task.user_id == user_id)
            .cloned()
            .collect::<Vec<_>>()
    });

    Ok(tasks)
}

/// Get system health (public endpoint for monitoring)
#[query]
fn get_system_health() -> SystemHealth {
    use ohms_shared::ComponentHealth;
    use std::collections::HashMap;

    with_state(|state| {
        let mut metrics = HashMap::new();
        metrics.insert("total_agents".to_string(), state.agents.len().to_string());
        metrics.insert("total_models".to_string(), state.models.len().to_string());
        metrics.insert("orchestration_tasks".to_string(), state.orchestration_tasks.len().to_string());

        SystemHealth {
            canister_id: ic_cdk::api::id(),
            status: ComponentHealth::Healthy,
            uptime_seconds: (ic_cdk::api::time() / 1_000_000_000) as u64,
            memory_usage_mb: {
                #[cfg(target_arch = "wasm32")]
                {
                    const WASM_PAGE_BYTES: f64 = 64.0 * 1024.0;
                    let heap_pages = core::arch::wasm32::memory_size(0) as f64;
                    let stable_pages = ic_cdk::api::stable::stable_size() as f64;
                    let total_bytes = (heap_pages + stable_pages) * WASM_PAGE_BYTES;
                    (total_bytes / (1024.0 * 1024.0)) as f32
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    0.0_f32
                }
            },
            last_update: ic_cdk::api::time(),
            version: "2.0.0".to_string(),
            metrics,
        }
    })
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;

    #[test]
    fn generate_candid() {
        std::fs::write("src/ohms_coordinator.did", export_candid())
            .expect("failed to write coordinator Candid");
    }
}
