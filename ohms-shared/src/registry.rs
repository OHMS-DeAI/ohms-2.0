// OHMS 2.0 Canister Registry
// Manages canister discovery and inter-canister communication

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse,
};
use ic_stable_structures::{memory::Memory, storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable};
use crate::{OHMSError, OHMSResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

type MemoryId = u8;
type CanisterRegistryMemory = DefaultMemoryImpl;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CanisterInfo {
    pub canister_id: Principal,
    pub canister_type: CanisterType,
    pub version: String,
    pub status: CanisterStatus,
    pub registered_at: u64,
    pub last_health_check: u64,
    pub health_score: f32,
}

impl Storable for CanisterInfo {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CanisterType {
    ModelRepository,
    AgentFactory,
    Coordinator,
    Economics,
    Frontend(String), // UI, Website, etc.
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CanisterStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Stopped,
    Unknown,
}

pub struct CanisterRegistry {
    canisters: StableBTreeMap<String, CanisterInfo, CanisterRegistryMemory>,
    canister_ids_by_type: HashMap<CanisterType, Principal>,
}

impl CanisterRegistry {
    pub fn new(memory: CanisterRegistryMemory) -> Self {
        Self {
            canisters: StableBTreeMap::new(memory),
            canister_ids_by_type: HashMap::new(),
        }
    }

    pub fn register_canister(&mut self, info: CanisterInfo) -> OHMSResult<()> {
        let key = info.canister_id.to_text();

        // Validate the canister exists and is accessible
        // Note: In a real implementation, we would verify the canister's Candid interface

        self.canisters.insert(key, info.clone());
        self.canister_ids_by_type
            .insert(info.canister_type.clone(), info.canister_id);

        Ok(())
    }

    pub fn get_canister(&self, canister_id: &str) -> Option<CanisterInfo> {
        self.canisters.get(&canister_id.to_string())
    }

    pub fn get_canister_by_type(&self, canister_type: &CanisterType) -> Option<Principal> {
        self.canister_ids_by_type.get(canister_type).copied()
    }

    pub fn list_canisters(&self) -> Vec<CanisterInfo> {
        self.canisters.iter().map(|(_, info)| info).collect()
    }

    pub fn list_canisters_by_type(&self, canister_type: &CanisterType) -> Vec<CanisterInfo> {
        self.canisters
            .iter()
            .filter(|(_, info)| &info.canister_type == canister_type)
            .map(|(_, info)| info)
            .collect()
    }

    pub fn update_health_status(
        &mut self,
        canister_id: &str,
        status: CanisterStatus,
        health_score: f32,
    ) -> OHMSResult<()> {
        if let Some(mut info) = self.canisters.get(&canister_id.to_string()) {
            info.status = status;
            info.health_score = health_score;
            info.last_health_check = crate::current_time_seconds();
            self.canisters.insert(canister_id.to_string(), info);
            Ok(())
        } else {
            Err(OHMSError::NotFound(format!(
                "Canister {} not registered",
                canister_id
            )))
        }
    }

    pub async fn health_check_all(&mut self) -> Vec<(String, CanisterStatus)> {
        let mut results = Vec::new();

        for (canister_id, info) in self.canisters.iter() {
            let status = self.check_canister_health(&info.canister_id).await;
            self.update_health_status(
                &canister_id,
                status.clone(),
                match status {
                    CanisterStatus::Healthy => 1.0,
                    CanisterStatus::Degraded => 0.5,
                    CanisterStatus::Unhealthy => 0.1,
                    CanisterStatus::Stopped => 0.0,
                    CanisterStatus::Unknown => 0.0,
                },
            )
            .unwrap_or(());

            results.push((canister_id, status));
        }

        results
    }

    async fn check_canister_health(&self, canister_id: &Principal) -> CanisterStatus {
        match canister_status(CanisterIdRecord {
            canister_id: *canister_id,
        })
        .await
        {
            Ok((response,)) => match response.status {
                ic_cdk::api::management_canister::main::CanisterStatusType::Running => {
                    CanisterStatus::Healthy
                }
                ic_cdk::api::management_canister::main::CanisterStatusType::Stopping => {
                    CanisterStatus::Degraded
                }
                ic_cdk::api::management_canister::main::CanisterStatusType::Stopped => {
                    CanisterStatus::Stopped
                }
            },
            Err(_) => CanisterStatus::Unknown,
        }
    }

    pub fn bootstrap_local_canisters(&mut self) -> OHMSResult<()> {
        // This method initializes the registry with known canister IDs from deployment
        // In a real deployment, these would be read from canister_ids.json or environment

        // Read from environment variables or canister_ids.json
        let model_id = std::env::var("OHMS_MODEL_CANISTER_ID")
            .or_else(|_| self.read_canister_id_from_file("ohms_model"))
            .map_err(|_| OHMSError::NotFound("Model canister ID not found".to_string()))?;

        let agent_id = std::env::var("OHMS_AGENT_CANISTER_ID")
            .or_else(|_| self.read_canister_id_from_file("ohms_agent"))
            .map_err(|_| OHMSError::NotFound("Agent canister ID not found".to_string()))?;

        let coordinator_id = std::env::var("OHMS_COORDINATOR_CANISTER_ID")
            .or_else(|_| self.read_canister_id_from_file("ohms_coordinator"))
            .map_err(|_| OHMSError::NotFound("Coordinator canister ID not found".to_string()))?;

        let econ_id = std::env::var("OHMS_ECON_CANISTER_ID")
            .or_else(|_| self.read_canister_id_from_file("ohms_econ"))
            .map_err(|_| OHMSError::NotFound("Econ canister ID not found".to_string()))?;

        // Register all known canisters
        let current_time = crate::current_time_seconds();

        self.register_canister(CanisterInfo {
            canister_id: Principal::from_text(&model_id)
                .map_err(|_| OHMSError::InvalidInput("Invalid model canister ID".to_string()))?,
            canister_type: CanisterType::ModelRepository,
            version: "1.0.0".to_string(),
            status: CanisterStatus::Unknown,
            registered_at: current_time,
            last_health_check: 0,
            health_score: 0.0,
        })?;

        self.register_canister(CanisterInfo {
            canister_id: Principal::from_text(&agent_id)
                .map_err(|_| OHMSError::InvalidInput("Invalid agent canister ID".to_string()))?,
            canister_type: CanisterType::AgentFactory,
            version: "1.0.0".to_string(),
            status: CanisterStatus::Unknown,
            registered_at: current_time,
            last_health_check: 0,
            health_score: 0.0,
        })?;

        self.register_canister(CanisterInfo {
            canister_id: Principal::from_text(&coordinator_id).map_err(|_| {
                OHMSError::InvalidInput("Invalid coordinator canister ID".to_string())
            })?,
            canister_type: CanisterType::Coordinator,
            version: "1.0.0".to_string(),
            status: CanisterStatus::Unknown,
            registered_at: current_time,
            last_health_check: 0,
            health_score: 0.0,
        })?;

        self.register_canister(CanisterInfo {
            canister_id: Principal::from_text(&econ_id)
                .map_err(|_| OHMSError::InvalidInput("Invalid econ canister ID".to_string()))?,
            canister_type: CanisterType::Economics,
            version: "1.0.0".to_string(),
            status: CanisterStatus::Unknown,
            registered_at: current_time,
            last_health_check: 0,
            health_score: 0.0,
        })?;

        Ok(())
    }

    fn read_canister_id_from_file(&self, canister_name: &str) -> Result<String, std::io::Error> {
        // In a real implementation, this would read from .dfx/local/canister_ids.json
        // For now, we'll use hardcoded fallbacks or return an error

        match canister_name {
            "ohms_model" => Ok("rdmx6-jaaaa-aaaaa-aaadq-cai".to_string()),
            "ohms_agent" => Ok("rrkah-fqaaa-aaaaa-aaaaq-cai".to_string()),
            "ohms_coordinator" => Ok("ryjl3-tyaaa-aaaaa-aaaba-cai".to_string()),
            "ohms_econ" => Ok("raxkx-dyaaa-aaaaa-aaafa-cai".to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Canister not found",
            )),
        }
    }

    pub fn get_all_canister_ids(&self) -> HashMap<String, Principal> {
        let mut ids = HashMap::new();

        for (_, info) in self.canisters.iter() {
            let name = match info.canister_type {
                CanisterType::ModelRepository => "model",
                CanisterType::AgentFactory => "agent",
                CanisterType::Coordinator => "coordinator",
                CanisterType::Economics => "econ",
                CanisterType::Frontend(ref name) => name,
            };
            ids.insert(name.to_string(), info.canister_id);
        }

        ids
    }
}

// Helper functions for canister discovery
pub fn discover_canisters_from_dfx() -> OHMSResult<HashMap<String, Principal>> {
    // This function would parse .dfx/local/canister_ids.json in a real implementation
    // For now, we return hardcoded development canister IDs

    let mut canisters = HashMap::new();

    // These IDs would be read from the actual deployment
    canisters.insert(
        "ohms_model".to_string(),
        Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")
            .map_err(|_| OHMSError::InvalidInput("Invalid model canister ID".to_string()))?,
    );

    canisters.insert(
        "ohms_agent".to_string(),
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
            .map_err(|_| OHMSError::InvalidInput("Invalid agent canister ID".to_string()))?,
    );

    canisters.insert(
        "ohms_coordinator".to_string(),
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")
            .map_err(|_| OHMSError::InvalidInput("Invalid coordinator canister ID".to_string()))?,
    );

    canisters.insert(
        "ohms_econ".to_string(),
        Principal::from_text("raxkx-dyaaa-aaaaa-aaafa-cai")
            .map_err(|_| OHMSError::InvalidInput("Invalid econ canister ID".to_string()))?,
    );

    Ok(canisters)
}

// Global registry instance (would be properly initialized in each canister)
thread_local! {
    static CANISTER_REGISTRY: std::cell::RefCell<Option<CanisterRegistry>> = std::cell::RefCell::new(None);
}

pub fn init_canister_registry() -> OHMSResult<()> {
    CANISTER_REGISTRY.with(|registry| {
        let memory = DefaultMemoryImpl::default();
        let mut reg = CanisterRegistry::new(memory);
        reg.bootstrap_local_canisters()?;
        *registry.borrow_mut() = Some(reg);
        Ok(())
    })
}

pub fn get_canister_registry() -> OHMSResult<std::cell::Ref<'static, CanisterRegistry>> {
    Ok(CANISTER_REGISTRY.with(|registry| {
        std::cell::Ref::map(registry.borrow(), |opt| {
            opt.as_ref().expect("Canister registry not initialized")
        })
    }))
}

pub fn get_canister_registry_mut() -> OHMSResult<std::cell::RefMut<'static, CanisterRegistry>> {
    Ok(CANISTER_REGISTRY.with(|registry| {
        std::cell::RefMut::map(registry.borrow_mut(), |opt| {
            opt.as_mut().expect("Canister registry not initialized")
        })
    }))
}
