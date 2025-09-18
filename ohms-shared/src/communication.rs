// OHMS 2.0 Inter-Canister Communication Module
// This module handles communication between all OHMS canisters

use candid::{CandidType, Principal};
use ic_cdk::api::call::{call, CallResult};
use crate::{
    AgentInfo, ComponentHealth, IntercanisterMessage, IntercanisterResponse, JobCost, ModelInfo,
    OHMSError, OHMSResult, SystemHealth,
};
use serde::{Deserialize, Serialize};

// ==============================================================================
// Canister ID Management
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CanisterIds {
    pub model: Principal,
    pub agent: Principal,
    pub coordinator: Principal,
    pub econ: Principal,
}

impl CanisterIds {
    pub fn from_env() -> OHMSResult<Self> {
        // In production, these would be read from environment or canister registry
        // For now, we'll use placeholder values that will be replaced during deployment
        Ok(CanisterIds {
            model: Principal::anonymous(), // Will be replaced with actual Principal
            agent: Principal::anonymous(),
            coordinator: Principal::anonymous(),
            econ: Principal::anonymous(),
        })
    }
}

// ==============================================================================
// Model Repository Communication
// ==============================================================================

pub struct ModelRepoClient {
    canister_id: Principal,
}

impl ModelRepoClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn health_check(&self) -> OHMSResult<ComponentHealth> {
        let result: CallResult<(ComponentHealth,)> = call(self.canister_id, "health", ()).await;

        match result {
            Ok((health,)) => Ok(health),
            Err(_) => Ok(ComponentHealth::Unhealthy),
        }
    }

    pub async fn get_model_info(&self, model_id: &str) -> OHMSResult<ModelInfo> {
        let result: CallResult<(OHMSResult<ModelInfo>,)> =
            call(self.canister_id, "get_model_info", (model_id,)).await;

        match result {
            Ok((model_result,)) => model_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to call model repository".to_string(),
            )),
        }
    }

    pub async fn list_active_models(&self) -> OHMSResult<Vec<ModelInfo>> {
        let result: CallResult<(Vec<ModelInfo>,)> =
            call(self.canister_id, "list_active_models", ()).await;

        match result {
            Ok((models,)) => Ok(models),
            Err(_) => Err(OHMSError::NetworkError("Failed to list models".to_string())),
        }
    }

    pub async fn notify_model_access(&self, model_id: &str, agent_id: &str) -> OHMSResult<()> {
        let result: CallResult<(OHMSResult<()>,)> = call(
            self.canister_id,
            "notify_model_access",
            (model_id, agent_id),
        )
        .await;

        match result {
            Ok((access_result,)) => access_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to notify model access".to_string(),
            )),
        }
    }
}

// ==============================================================================
// Agent Communication
// ==============================================================================

pub struct AgentClient {
    canister_id: Principal,
}

impl AgentClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn health_check(&self) -> OHMSResult<ComponentHealth> {
        let result: CallResult<(ComponentHealth,)> = call(self.canister_id, "health", ()).await;

        match result {
            Ok((health,)) => Ok(health),
            Err(_) => Ok(ComponentHealth::Unhealthy),
        }
    }

    pub async fn create_agent(&self, spec: &AgentSpec) -> OHMSResult<AgentInfo> {
        let result: CallResult<(OHMSResult<AgentInfo>,)> =
            call(self.canister_id, "create_agent", (spec,)).await;

        match result {
            Ok((agent_result,)) => agent_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to create agent".to_string(),
            )),
        }
    }

    pub async fn get_agent_info(&self, agent_id: &str) -> OHMSResult<AgentInfo> {
        let result: CallResult<(OHMSResult<AgentInfo>,)> =
            call(self.canister_id, "get_agent_info", (agent_id,)).await;

        match result {
            Ok((agent_result,)) => agent_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to get agent info".to_string(),
            )),
        }
    }

    pub async fn send_inference_request(
        &self,
        request: &InferenceRequest,
    ) -> OHMSResult<InferenceResponse> {
        let result: CallResult<(OHMSResult<InferenceResponse>,)> =
            call(self.canister_id, "inference", (request,)).await;

        match result {
            Ok((inference_result,)) => inference_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to send inference request".to_string(),
            )),
        }
    }
}

// ==============================================================================
// Coordinator Communication
// ==============================================================================

pub struct CoordinatorClient {
    canister_id: Principal,
}

impl CoordinatorClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn health_check(&self) -> OHMSResult<ComponentHealth> {
        let result: CallResult<(ComponentHealth,)> = call(self.canister_id, "health", ()).await;

        match result {
            Ok((health,)) => Ok(health),
            Err(_) => Ok(ComponentHealth::Unhealthy),
        }
    }

    pub async fn register_agent(&self, agent_info: &AgentInfo) -> OHMSResult<()> {
        let result: CallResult<(OHMSResult<()>,)> =
            call(self.canister_id, "register_agent", (agent_info,)).await;

        match result {
            Ok((register_result,)) => register_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to register agent".to_string(),
            )),
        }
    }

    pub async fn request_agent_creation(
        &self,
        request: &CoordinationRequest,
    ) -> OHMSResult<String> {
        let result: CallResult<(OHMSResult<String>,)> = call(
            self.canister_id,
            "analyze_instruction_and_spawn_agents",
            (request,),
        )
        .await;

        match result {
            Ok((request_result,)) => request_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to request agent creation".to_string(),
            )),
        }
    }

    pub async fn get_coordination_status(
        &self,
        request_id: &str,
    ) -> OHMSResult<CoordinationStatus> {
        let result: CallResult<(OHMSResult<CoordinationStatus>,)> =
            call(self.canister_id, "get_coordination_status", (request_id,)).await;

        match result {
            Ok((status_result,)) => status_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to get coordination status".to_string(),
            )),
        }
    }
}

// ==============================================================================
// Economics Communication
// ==============================================================================

pub struct EconClient {
    canister_id: Principal,
}

impl EconClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn health_check(&self) -> OHMSResult<ComponentHealth> {
        let result: CallResult<(ComponentHealth,)> = call(self.canister_id, "health", ()).await;

        match result {
            Ok((health,)) => Ok(health),
            Err(_) => Ok(ComponentHealth::Unhealthy),
        }
    }

    pub async fn get_cost_quote(&self, job_spec: &JobSpec) -> OHMSResult<JobCost> {
        let result: CallResult<(OHMSResult<JobCost>,)> =
            call(self.canister_id, "get_cost_quote", (job_spec,)).await;

        match result {
            Ok((quote_result,)) => quote_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to get cost quote".to_string(),
            )),
        }
    }

    pub async fn create_escrow(
        &self,
        job_cost: &JobCost,
        principal: Principal,
    ) -> OHMSResult<String> {
        let result: CallResult<(OHMSResult<String>,)> =
            call(self.canister_id, "create_escrow", (job_cost, principal)).await;

        match result {
            Ok((escrow_result,)) => escrow_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to create escrow".to_string(),
            )),
        }
    }

    pub async fn release_escrow(&self, escrow_id: &str, agent_id: &str) -> OHMSResult<()> {
        let result: CallResult<(OHMSResult<()>,)> =
            call(self.canister_id, "release_escrow", (escrow_id, agent_id)).await;

        match result {
            Ok((release_result,)) => release_result,
            Err(_) => Err(OHMSError::NetworkError(
                "Failed to release escrow".to_string(),
            )),
        }
    }
}

// ==============================================================================
// Unified OHMS Client
// ==============================================================================

pub struct OHMSClient {
    pub model: ModelRepoClient,
    pub agent: AgentClient,
    pub coordinator: CoordinatorClient,
    pub econ: EconClient,
}

impl OHMSClient {
    pub fn new(canister_ids: CanisterIds) -> Self {
        Self {
            model: ModelRepoClient::new(canister_ids.model),
            agent: AgentClient::new(canister_ids.agent),
            coordinator: CoordinatorClient::new(canister_ids.coordinator),
            econ: EconClient::new(canister_ids.econ),
        }
    }

    pub async fn system_health_check(&self) -> SystemHealth {
        let (model_health, agent_health, coordinator_health, econ_health) = futures::join!(
            self.model.health_check(),
            self.agent.health_check(),
            self.coordinator.health_check(),
            self.econ.health_check()
        );

        SystemHealth {
            model: model_health.unwrap_or(ComponentHealth::Unknown),
            agent: agent_health.unwrap_or(ComponentHealth::Unknown),
            coordinator: coordinator_health.unwrap_or(ComponentHealth::Unknown),
            econ: econ_health.unwrap_or(ComponentHealth::Unknown),
            timestamp: crate::current_time_seconds(),
        }
    }

    // High-level orchestration methods
    pub async fn create_agent_with_payment(
        &self,
        spec: &AgentSpec,
        user_principal: Principal,
    ) -> OHMSResult<AgentInfo> {
        // 1. Get cost quote
        let job_spec = JobSpec {
            job_id: format!("agent-creation-{}", spec.agent_id),
            model_id: spec.model_id.clone(),
            estimated_tokens: spec.estimated_tokens.unwrap_or(1000),
            estimated_compute_cycles: spec.estimated_compute_cycles.unwrap_or(1_000_000_000),
            priority: spec.priority.clone(),
        };

        let cost_quote = self.econ.get_cost_quote(&job_spec).await?;

        // 2. Create escrow
        let escrow_id = self.econ.create_escrow(&cost_quote, user_principal).await?;

        // 3. Create agent
        let agent_info = match self.agent.create_agent(spec).await {
            Ok(info) => info,
            Err(e) => {
                // Refund escrow on failure
                // Note: This would require a refund_escrow method in the econ canister
                return Err(e);
            }
        };

        // 4. Register agent with coordinator
        if let Err(e) = self.coordinator.register_agent(&agent_info).await {
            // Log error but don't fail the creation
            ic_cdk::println!("Failed to register agent with coordinator: {:?}", e);
        }

        // 5. Release escrow payment
        if let Err(e) = self
            .econ
            .release_escrow(&escrow_id, &agent_info.agent_id)
            .await
        {
            ic_cdk::println!("Failed to release escrow: {:?}", e);
        }

        Ok(agent_info)
    }
}

// ==============================================================================
// Supporting Types (should be moved to ohms-shared eventually)
// ==============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AgentSpec {
    pub agent_id: String,
    pub agent_type: crate::AgentType,
    pub model_id: String,
    pub capabilities: Vec<String>,
    pub complexity: crate::ComplexityLevel,
    pub urgency: crate::UrgencyLevel,
    pub estimated_tokens: Option<u32>,
    pub estimated_compute_cycles: Option<u64>,
    pub priority: crate::JobPriority,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InferenceRequest {
    pub msg_id: String,
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub seed: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InferenceResponse {
    pub generated_text: String,
    pub tokens: Vec<String>,
    pub inference_time_ms: u64,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct JobSpec {
    pub job_id: String,
    pub model_id: String,
    pub estimated_tokens: u32,
    pub estimated_compute_cycles: u64,
    pub priority: crate::JobPriority,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CoordinationRequest {
    pub request_id: String,
    pub user_principal: String,
    pub instructions: String,
    pub coordination_type: crate::CoordinationType,
    pub agent_requirements: Vec<crate::AgentRequirement>,
    pub created_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CoordinationStatus {
    pub request_id: String,
    pub status: String,
    pub created_agents: Vec<String>,
    pub completed_tasks: Vec<String>,
    pub pending_tasks: Vec<String>,
    pub last_updated: u64,
}
