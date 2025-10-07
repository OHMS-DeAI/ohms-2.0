use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::error::{NovaQError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    pub target_bits: f32,
    pub max_subspace_dim: usize,
    pub min_subspace_dim: usize,
    pub level1_centroids: usize,
    pub level2_centroids: usize,
    pub outlier_percentile: f32,
    pub max_iterations: usize,
    pub tolerance: f32,
    pub seed: u64,
    pub use_parallel: bool,
    pub min_cluster_size: usize,
    pub residual_variance_floor: f32,
    pub max_refinement_steps: usize,
    pub refinement_learning_rate: f32,
    pub distillation_kl_weight: f32,
    pub distillation_cosine_weight: f32,
}

impl Default for QuantizationConfig {
    fn default() -> Self {
        Self {
            target_bits: 1.5,
            max_subspace_dim: 16,
            min_subspace_dim: 4,
            level1_centroids: 16,
            level2_centroids: 8,
            outlier_percentile: 0.01,
            max_iterations: 100,
            tolerance: 1e-4,
            seed: 42,
            use_parallel: true,
            min_cluster_size: 4,
            residual_variance_floor: 1e-6,
            max_refinement_steps: 25,
            refinement_learning_rate: 1e-2,
            distillation_kl_weight: 1.0,
            distillation_cosine_weight: 0.5,
        }
    }
}

impl QuantizationConfig {
    pub fn validate(&self) -> Result<()> {
        if !(0.5..=8.0).contains(&self.target_bits) {
            return Err(NovaQError::InvalidConfig(format!(
                "target_bits must be between 0.5 and 8.0, found {}",
                self.target_bits
            )));
        }

        if self.max_subspace_dim == 0 {
            return Err(NovaQError::InvalidConfig(
                "max_subspace_dim must be greater than zero".to_string(),
            ));
        }

        if self.min_subspace_dim == 0 || self.min_subspace_dim > self.max_subspace_dim {
            return Err(NovaQError::InvalidConfig(
                "min_subspace_dim must be > 0 and <= max_subspace_dim".to_string(),
            ));
        }

        if self.level1_centroids < 2 {
            return Err(NovaQError::InvalidConfig(
                "level1_centroids must be at least 2".to_string(),
            ));
        }

        if self.outlier_percentile <= 0.0 || self.outlier_percentile >= 1.0 {
            return Err(NovaQError::InvalidConfig(
                "outlier_percentile must be in (0, 1)".to_string(),
            ));
        }

        if self.max_iterations == 0 {
            return Err(NovaQError::InvalidConfig(
                "max_iterations must be positive".to_string(),
            ));
        }

        if self.tolerance <= 0.0 {
            return Err(NovaQError::InvalidConfig(
                "tolerance must be positive".to_string(),
            ));
        }

        if self.min_cluster_size == 0 {
            return Err(NovaQError::InvalidConfig(
                "min_cluster_size must be positive".to_string(),
            ));
        }

        if self.residual_variance_floor <= 0.0 {
            return Err(NovaQError::InvalidConfig(
                "residual_variance_floor must be positive".to_string(),
            ));
        }

        if self.max_refinement_steps == 0 {
            return Err(NovaQError::InvalidConfig(
                "max_refinement_steps must be positive".to_string(),
            ));
        }

        if self.refinement_learning_rate <= 0.0 {
            return Err(NovaQError::InvalidConfig(
                "refinement_learning_rate must be positive".to_string(),
            ));
        }

        if self.distillation_kl_weight < 0.0 || self.distillation_cosine_weight < 0.0 {
            return Err(NovaQError::InvalidConfig(
                "distillation weights must be non-negative".to_string(),
            ));
        }

        Ok(())
    }

    pub fn layer_seed(&self, layer_name: &str, layer_index: usize) -> u64 {
        let mut hasher = Hasher::new();
        hasher.update(&self.seed.to_le_bytes());
        hasher.update(layer_name.as_bytes());
        hasher.update(&layer_index.to_le_bytes());
        let hash = hasher.finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash.as_bytes()[..8]);
        u64::from_le_bytes(bytes)
    }
}
