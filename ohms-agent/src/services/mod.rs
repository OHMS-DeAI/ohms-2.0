use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use ic_cdk::api::time;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use serde::{Deserialize, Serialize};

use crate::domain::{
    AgentConfig, AgentPerformanceMetrics, AgentRecord, AgentTask, InferenceResponse, SystemMetrics,
};
use ohms_shared::ModelManifest;

pub mod agent_factory;
pub mod binding_service;
pub mod cache_service;
pub mod inference_service;
pub mod instruction_analyzer;
pub mod memory_service;
pub mod model_repo_client;

pub use agent_factory::AgentFactory;
pub use binding_service::BindingService;
pub use cache_service::CacheService;
pub use inference_service::InferenceService;
pub use instruction_analyzer::InstructionAnalyzer;
pub use memory_service::MemoryService;
pub use model_repo_client::ModelRepoClient;

thread_local! {
    static STATE: RefCell<AgentCanisterState> = RefCell::new(AgentCanisterState::default());
}

pub fn with_state<R>(f: impl FnOnce(&AgentCanisterState) -> R) -> R {
    STATE.with(|state| f(&state.borrow()))
}

pub fn with_state_mut<R>(f: impl FnOnce(&mut AgentCanisterState) -> R) -> R {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

#[derive(Debug)]
pub struct AgentCanisterState {
    pub config: AgentConfig,
    pub binding: Option<ModelBindingState>,
    pub cache_entries: HashMap<String, CachedInference>,
    pub cache_order: VecDeque<String>,
    pub cache_capacity: usize,
    pub memory: MemoryStore,
    pub agents: HashMap<String, AgentRecord>,
    pub performance: HashMap<String, AgentPerformanceMetrics>,
    pub metrics: SystemMetrics,
    pub task_queue: VecDeque<QueuedTask>,
    pub rng: ChaCha20Rng,
    pub active_inference: u32,
    pub last_inference: u64,
}

impl Default for AgentCanisterState {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentCanisterState {
    pub fn new() -> Self {
        let seed = time();
        let mut seed_bytes = [0u8; 32];
        seed_bytes[..8].copy_from_slice(&seed.to_be_bytes());
        Self {
            config: AgentConfig::default(),
            binding: None,
            cache_entries: HashMap::new(),
            cache_order: VecDeque::new(),
            cache_capacity: 64,
            memory: MemoryStore::default(),
            agents: HashMap::new(),
            performance: HashMap::new(),
            metrics: SystemMetrics::default(),
            task_queue: VecDeque::new(),
            rng: ChaCha20Rng::from_seed(seed_bytes),
            active_inference: 0,
            last_inference: 0,
        }
    }

    pub fn next_random_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn cache_ttl_seconds(&self) -> u64 {
        self.config.ttl_seconds.max(5)
    }

    pub fn purge_expired_cache(&mut self, now: u64) {
        let ttl_ns = self.cache_ttl_seconds() * 1_000_000_000;
        let mut expired = Vec::new();
        for (key, entry) in self.cache_entries.iter() {
            if now.saturating_sub(entry.created_at) > ttl_ns {
                expired.push(key.clone());
            }
        }
        for key in expired {
            self.cache_entries.remove(&key);
            if let Some(pos) = self.cache_order.iter().position(|k| k == &key) {
                self.cache_order.remove(pos);
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelBindingState {
    pub model_id: String,
    pub manifest: ModelManifest,
    pub bound_at: u64,
    pub last_prefetch_at: u64,
    pub chunks_loaded: u32,
    pub total_chunks: u32,
    pub ready: bool,
}

impl ModelBindingState {
    pub fn new(manifest: ModelManifest) -> Self {
        let now = time();
        Self {
            total_chunks: manifest.chunk_count.max(1),
            model_id: manifest.model_id.clone(),
            manifest,
            bound_at: now,
            last_prefetch_at: now,
            chunks_loaded: 0,
            ready: false,
        }
    }

    pub fn mark_ready(&mut self) {
        self.ready = true;
    }

    pub fn register_prefetch(&mut self, count: u32) {
        self.last_prefetch_at = time();
        self.chunks_loaded = self
            .chunks_loaded
            .saturating_add(count)
            .min(self.total_chunks);
        if self.chunks_loaded >= self.total_chunks {
            self.ready = true;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedInference {
    pub response: InferenceResponse,
    pub created_at: u64,
    pub hits: u32,
    pub misses: u32,
}

impl CachedInference {
    pub fn new(response: InferenceResponse, now: u64) -> Self {
        Self {
            response,
            created_at: now,
            hits: 0,
            misses: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub memory_id: String,
    pub content: String,
    pub created_at: u64,
    pub last_accessed: u64,
    pub importance: f32,
    pub ttl_seconds: u64,
}

#[derive(Default, Debug)]
pub struct MemoryStore {
    pub entries: HashMap<String, MemoryEntry>,
    pub short_term_capacity: usize,
    pub long_term_capacity: usize,
    pub eviction_queue: VecDeque<String>,
}

impl MemoryStore {
    pub fn insert(&mut self, entry: MemoryEntry) {
        if self.entries.len() >= (self.short_term_capacity + self.long_term_capacity).max(32) {
            if let Some(oldest) = self.eviction_queue.pop_front() {
                self.entries.remove(&oldest);
            }
        }
        self.eviction_queue.push_back(entry.memory_id.clone());
        self.entries.insert(entry.memory_id.clone(), entry);
    }

    pub fn prune_expired(&mut self, now: u64) {
        self.eviction_queue.retain(|key| {
            if let Some(entry) = self.entries.get(key) {
                now.saturating_sub(entry.created_at) <= entry.ttl_seconds * 1_000_000_000
            } else {
                false
            }
        });
        let valid_keys: Vec<String> = self.eviction_queue.iter().cloned().collect();
        let valid: std::collections::HashSet<_> = valid_keys.iter().collect();
        self.entries.retain(|k, entry| {
            let alive = now.saturating_sub(entry.created_at) <= entry.ttl_seconds * 1_000_000_000;
            alive && valid.contains(k)
        });
    }

    pub fn stats(&self, now: u64) -> MemoryStats {
        let mut total_importance = 0.0;
        let mut freshest = 0u64;
        for entry in self.entries.values() {
            total_importance += entry.importance as f64;
            freshest = freshest.max(now.saturating_sub(entry.last_accessed));
        }
        MemoryStats {
            entries: self.entries.len() as u32,
            average_importance: if self.entries.is_empty() {
                0.0
            } else {
                total_importance / self.entries.len() as f64
            },
            max_idle_ns: freshest,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub entries: u32,
    pub average_importance: f64,
    pub max_idle_ns: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedTask {
    pub agent_id: String,
    pub task: AgentTask,
    pub enqueued_at: u64,
}

impl QueuedTask {
    pub fn new(agent_id: String, task: AgentTask) -> Self {
        Self {
            agent_id,
            task,
            enqueued_at: time(),
        }
    }
}

pub fn generate_id(state: &mut AgentCanisterState, prefix: &str) -> String {
    let counter = state.next_random_u64();
    format!("{}-{:x}-{:x}", prefix, time(), counter)
}
