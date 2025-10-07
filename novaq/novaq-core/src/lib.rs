mod analysis;
mod config;
mod error;
mod metrics;
mod model;
mod normalization;
mod quantization;
mod subspace;

pub use config::QuantizationConfig;
pub use error::{NovaQError, Result};
pub use model::{
    CodebookStage, LayerAnalysis, LayerMetrics, LayerTelemetry, NormalizationRecord, OutlierEntry,
    QuantizationSummary, QuantizedLayer, QuantizedModel, QuantizedSubspace, SubspaceTelemetry,
};
pub use normalization::Normalizer;
pub use quantization::DistillationHints;

use ndarray::Array2;
use rand::{rngs::StdRng, SeedableRng};
use tracing::instrument;

use quantization::ProductQuantizer;

use crate::metrics::compute_layer_metrics;
use crate::subspace::plan_subspaces;

pub struct Quantizer {
    config: QuantizationConfig,
}

impl Quantizer {
    pub fn new(config: QuantizationConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    #[instrument(skip_all, fields(layer = name))]
    pub fn quantize_layer(
        &self,
        name: &str,
        index: usize,
        weights: &Array2<f32>,
    ) -> Result<QuantizedLayer> {
        self.quantize_layer_with_hints(name, index, weights, None)
    }

    pub fn quantize_layer_with_hints(
        &self,
        name: &str,
        index: usize,
        weights: &Array2<f32>,
        hints: Option<&DistillationHints>,
    ) -> Result<QuantizedLayer> {
        let start = std::time::Instant::now();
        if weights.is_empty() {
            return Err(NovaQError::EmptyTensor);
        }

        let (analysis, plan) = plan_subspaces(&self.config, weights)?;

        let mut rng = StdRng::seed_from_u64(self.config.layer_seed(name, index));
        let normalizer = Normalizer::new(self.config.outlier_percentile)?;
        let (normalized, normalization_record) =
            normalizer.normalize_with_context(weights, Some(&analysis))?;

        let pq = ProductQuantizer::new(&self.config)?;
        let quantization = pq.quantize(&normalized, &plan, &mut rng, hints)?;

        let rows = normalized.nrows();
        let cols = normalized.ncols();
        let normalized_reconstruction = pq.reconstruct(rows, cols, &quantization.subspaces);
        let reconstructed =
            normalizer.denormalize(&normalized_reconstruction, &normalization_record);
        let compressed_bits = pq.estimate_compressed_bits(rows, &quantization.subspaces);

        let metrics = compute_layer_metrics(weights, &reconstructed, compressed_bits);
        let elapsed = start.elapsed();

        Ok(QuantizedLayer {
            name: name.to_string(),
            index,
            rows,
            cols,
            seed: self.config.layer_seed(name, index),
            normalization: normalization_record,
            subspaces: quantization.subspaces,
            metrics,
            quantization_time_us: elapsed.as_micros() as u64,
            telemetry: LayerTelemetry {
                analysis,
                subspaces: quantization.telemetry,
            },
        })
    }

    pub fn quantize_model<I>(&self, layers: I) -> Result<QuantizedModel>
    where
        I: IntoIterator<Item = (String, Array2<f32>)>,
    {
        let mut quantized_layers = Vec::new();
        for (index, (name, weights)) in layers.into_iter().enumerate() {
            let layer = self.quantize_layer(&name, index, &weights)?;
            quantized_layers.push(layer);
        }
        Ok(QuantizedModel::from_layers(quantized_layers))
    }

    pub fn config(&self) -> &QuantizationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    use proptest::prelude::*;
    use rand::Rng;
    use rand::SeedableRng;

    fn random_matrix(rows: usize, cols: usize, seed: u64) -> Array2<f32> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        Array2::from_shape_fn((rows, cols), |_| rng.gen_range(-5.0..5.0))
    }

    fn assert_arrays_close(a: &Array2<f32>, b: &Array2<f32>, eps: f32) {
        assert_eq!(a.dim(), b.dim());
        for (left, right) in a.iter().zip(b.iter()) {
            assert!(
                (*left - *right).abs() <= eps,
                "array entries differ: left={} right={} (eps={})",
                left,
                right,
                eps
            );
        }
    }

    #[test]
    fn deterministic_quantization_same_seed() {
        let config = QuantizationConfig {
            max_subspace_dim: 8,
            level1_centroids: 4,
            level2_centroids: 2,
            ..QuantizationConfig::default()
        };
        let quantizer = Quantizer::new(config).unwrap();
        let weights = random_matrix(32, 16, 7);
        let layer_a = quantizer.quantize_layer("linear1", 0, &weights).unwrap();
        let layer_b = quantizer.quantize_layer("linear1", 0, &weights).unwrap();

        assert_eq!(layer_a.subspaces.len(), layer_b.subspaces.len());
        for (sub_a, sub_b) in layer_a.subspaces.iter().zip(layer_b.subspaces.iter()) {
            assert_eq!(sub_a.stage1.assignments, sub_b.stage1.assignments);
            assert_arrays_close(&sub_a.stage1.centroids, &sub_b.stage1.centroids, 1e-6);
            match (&sub_a.stage2, &sub_b.stage2) {
                (Some(a), Some(b)) => {
                    assert_eq!(a.assignments, b.assignments);
                    assert_arrays_close(&a.centroids, &b.centroids, 1e-6);
                }
                (None, None) => {}
                _ => panic!("stage2 mismatch"),
            }
        }

        assert!((layer_a.metrics.mse - layer_b.metrics.mse).abs() <= 1e-8);
        assert_eq!(
            layer_a.telemetry.subspaces.len(),
            layer_b.telemetry.subspaces.len()
        );
        for (tele_a, tele_b) in layer_a
            .telemetry
            .subspaces
            .iter()
            .zip(layer_b.telemetry.subspaces.iter())
        {
            assert_eq!(tele_a.columns, tele_b.columns);
            assert_eq!(tele_a.stage1_iterations, tele_b.stage1_iterations);
            assert_eq!(tele_a.stage2_iterations, tele_b.stage2_iterations);
            assert!((tele_a.residual_energy - tele_b.residual_energy).abs() <= 1e-9);
        }
    }

    proptest! {
        #[test]
        fn reconstruction_error_is_bounded(rows in 4usize..32, cols in 4usize..48, seed in any::<u64>()) {
            let weights = random_matrix(rows, cols.max(4), seed);
            let config = QuantizationConfig {
                max_subspace_dim: 8,
                level1_centroids: 8,
                level2_centroids: 4,
                ..QuantizationConfig::default()
            };
            let quantizer = Quantizer::new(config).unwrap();
            let layer = quantizer.quantize_layer("layer", 0, &weights).unwrap();
            prop_assert!(layer.metrics.mse >= 0.0);
            prop_assert!(layer.metrics.mse < 5.0);
            prop_assert!(layer.metrics.cosine_similarity <= 1.0 + 1e-5);
            prop_assert!(layer.metrics.cosine_similarity >= -1.0 - 1e-5);
            prop_assert!(layer.telemetry.subspaces.len() >= 1);
        }
    }
}
