use ic_cdk::api::time;

use crate::domain::{InferenceRequest, InferenceResponse};

use super::{with_state, with_state_mut, AgentCanisterState, CachedInference};

pub struct CacheService;

impl CacheService {
    pub fn try_get(request: &InferenceRequest) -> Option<InferenceResponse> {
        let key = request.cache_key();
        with_state_mut(|state| {
            let now = time();
            state.purge_expired_cache(now);
            if let Some(entry) = state.cache_entries.get_mut(&key) {
                entry.hits = entry.hits.saturating_add(1);
                if let Some(pos) = state.cache_order.iter().position(|k| k == &key) {
                    let key = state.cache_order.remove(pos).unwrap();
                    state.cache_order.push_back(key);
                }
                state.metrics.total_cache_hits += 1;
                return Some(entry.response.clone());
            }
            state.metrics.total_cache_misses += 1;
            None
        })
    }

    pub fn insert(request: &InferenceRequest, response: InferenceResponse) {
        let key = request.cache_key();
        with_state_mut(|state| {
            insert_response(state, key, response);
        });
    }

    pub fn reserve_slots(state: &mut AgentCanisterState, count: usize) {
        let now = time();
        state.purge_expired_cache(now);
        let needed = state
            .cache_entries
            .len()
            .saturating_add(count)
            .saturating_sub(state.cache_capacity);
        for _ in 0..needed {
            if let Some(key) = state.cache_order.pop_front() {
                state.cache_entries.remove(&key);
            } else {
                break;
            }
        }
    }

    pub fn clear_all(state: &mut AgentCanisterState) {
        state.cache_entries.clear();
        state.cache_order.clear();
    }

    pub fn get_utilization() -> f64 {
        with_state(|state| {
            if state.cache_capacity == 0 {
                return 0.0;
            }
            (state.cache_entries.len() as f64 / state.cache_capacity as f64).min(1.0)
        })
    }
}

fn insert_response(state: &mut AgentCanisterState, key: String, response: InferenceResponse) {
    if state.cache_entries.contains_key(&key) {
        if let Some(pos) = state.cache_order.iter().position(|k| k == &key) {
            state.cache_order.remove(pos);
        }
    }
    state.cache_order.push_back(key.clone());

    let now = time();
    state
        .cache_entries
        .insert(key, CachedInference::new(response, now));
    CacheService::reserve_slots(state, 0);
}
