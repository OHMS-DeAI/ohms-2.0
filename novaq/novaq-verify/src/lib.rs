//! Verification policies and metric thresholds used by the NOVAQ toolkit.

/// Defines minimum acceptable metrics for a verification run.
#[derive(Debug, Clone)]
pub struct VerifyPolicy {
    pub min_cosine_similarity: f32,
    pub max_mse: f32,
    pub max_kl_divergence: f32,
}

impl VerifyPolicy {
    pub fn strict_defaults() -> Self {
        Self {
            min_cosine_similarity: 0.98,
            max_mse: 1e-2,
            max_kl_divergence: 1e-3,
        }
    }

    pub fn relaxed_defaults() -> Self {
        Self {
            min_cosine_similarity: 0.95,
            max_mse: 5e-2,
            max_kl_divergence: 5e-3,
        }
    }

    pub fn satisfies(&self, cosine: f32, mse: f32, kl: f32) -> bool {
        cosine >= self.min_cosine_similarity && mse <= self.max_mse && kl <= self.max_kl_divergence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_satisfaction_behaves_monotonically() {
        let policy = VerifyPolicy::strict_defaults();
        assert!(policy.satisfies(0.99, 1e-3, 5e-4));
        assert!(!policy.satisfies(0.90, 1e-3, 5e-4));
    }
}
