use std::fmt::{self, Display};

use ic_cdk::api::time;

use super::{generate_id, with_state, with_state_mut, MemoryEntry, MemoryStats};

pub struct MemoryService;

impl MemoryService {
    pub fn reconfigure(state: &mut super::AgentCanisterState) {
        let max_tokens = state.config.max_tokens.max(128);
        let st = (max_tokens / 128).clamp(8, 64) as usize;
        let lt = (max_tokens / 32).clamp(32, 256) as usize;
        state.memory.short_term_capacity = st;
        state.memory.long_term_capacity = lt;
    }

    pub fn record_interaction(prompt: &str, response: &str, importance: f32) {
        let prompt_importance = importance.max(0.1).min(1.0);
        let response_importance = (importance + 0.2).min(1.0);
        with_state_mut(|state| {
            let now = time();
            let prompt_entry = MemoryEntry {
                memory_id: generate_id(state, "prompt"),
                content: format!("PROMPT:{}", prompt),
                created_at: now,
                last_accessed: now,
                importance: prompt_importance,
                ttl_seconds: state.cache_ttl_seconds(),
            };
            let response_entry = MemoryEntry {
                memory_id: generate_id(state, "response"),
                content: format!("RESPONSE:{}", response),
                created_at: now,
                last_accessed: now,
                importance: response_importance,
                ttl_seconds: state.cache_ttl_seconds() * 2,
            };

            state.memory.insert(prompt_entry);
            state.memory.insert(response_entry);
        });
    }

    pub fn get_stats() -> MemoryStatsWrapper {
        with_state(|state| {
            let now = time();
            state.memory.stats(now)
        })
        .into()
    }

    pub fn clear_expired() {
        with_state_mut(|state| {
            let now = time();
            state.memory.prune_expired(now);
        });
    }
}

pub struct MemoryStatsWrapper(MemoryStats);

impl From<MemoryStats> for MemoryStatsWrapper {
    fn from(stats: MemoryStats) -> Self {
        MemoryStatsWrapper(stats)
    }
}

impl Display for MemoryStatsWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "entries={}, avg_importance={:.3}, max_idle_ms={}",
            self.0.entries,
            self.0.average_importance,
            self.0.max_idle_ns / 1_000_000
        )
    }
}
