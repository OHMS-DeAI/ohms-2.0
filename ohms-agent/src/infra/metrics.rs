use ic_cdk::api::time;

use crate::services::with_state_mut;

pub struct Metrics;

impl Metrics {
    pub fn increment_inference_count() {
        with_state_mut(|state| {
            state.metrics.total_inference_requests =
                state.metrics.total_inference_requests.saturating_add(1);
            state.last_inference = time();
        });
    }
}
