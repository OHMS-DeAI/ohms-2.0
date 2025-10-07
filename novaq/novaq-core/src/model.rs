//! Data structures persisted by the NOVAQ quantization pipeline.

use std::ops::Range;

use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierEntry {
    pub row: usize,
    pub col: usize,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationRecord {
    pub column_means: Vec<f32>,
    pub column_stds: Vec<f32>,
    pub outliers: Vec<OutlierEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Statistical snapshot captured before quantization.
pub struct LayerAnalysis {
    /// Number of weight rows (input channels).
    pub rows: usize,
    /// Number of weight columns (output channels).
    pub cols: usize,
    /// Mean of the tensor values.
    pub mean: f32,
    pub variance: f32,
    pub std: f32,
    /// Excess kurtosis estimate used to gauge heavy tails.
    pub kurtosis: f32,
    pub skewness: f32,
    /// Ratio of zeros to total elements.
    pub sparsity: f32,
    pub max_abs: f32,
    pub l2_norm: f32,
    /// Ratio between the largest and smallest column variances.
    pub anisotropy: f32,
    pub column_variances: Vec<f32>,
    pub row_variances: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebookStage {
    pub stage_id: u8,
    pub centroids: Array2<f32>,
    pub assignments: Vec<u16>,
    pub iterations: usize,
    pub inertia: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedSubspace {
    pub columns: Range<usize>,
    pub stage1: CodebookStage,
    pub stage2: Option<CodebookStage>,
    pub residual_energy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMetrics {
    pub mse: f32,
    pub cosine_similarity: f32,
    pub kl_divergence: f32,
    pub original_bits: u64,
    pub compressed_bits: u64,
    pub bits_per_weight: f32,
}

impl LayerMetrics {
    pub fn compression_ratio(&self) -> f32 {
        if self.compressed_bits == 0 {
            return 0.0;
        }
        self.original_bits as f32 / self.compressed_bits as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Telemetry gathered for each quantized subspace.
pub struct SubspaceTelemetry {
    /// Column range covered by this subspace.
    pub columns: Range<usize>,
    /// Lloyd iterations performed for stage 1.
    pub stage1_iterations: usize,
    /// Lloyd iterations performed for stage 2 (if enabled).
    pub stage2_iterations: Option<usize>,
    /// Final inertia reported by stage 1 training.
    pub stage1_inertia: f32,
    /// Final inertia reported by stage 2 training (if enabled).
    pub stage2_inertia: Option<f32>,
    /// Average residual energy remaining after reconstruction.
    pub residual_energy: f32,
    /// Whether stage 2 residual quantization was applied.
    pub enabled_stage2: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Layer-level telemetry combining the original analysis and per-subspace metrics.
pub struct LayerTelemetry {
    pub analysis: LayerAnalysis,
    pub subspaces: Vec<SubspaceTelemetry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedLayer {
    pub name: String,
    pub index: usize,
    pub rows: usize,
    pub cols: usize,
    pub seed: u64,
    pub normalization: NormalizationRecord,
    pub subspaces: Vec<QuantizedSubspace>,
    pub metrics: LayerMetrics,
    pub quantization_time_us: u64,
    pub telemetry: LayerTelemetry,
}

impl QuantizedLayer {
    pub fn parameter_count(&self) -> usize {
        self.rows * self.cols
    }

    pub fn compressed_bits(&self) -> u64 {
        self.metrics.compressed_bits
    }

    pub fn original_bits(&self) -> u64 {
        self.metrics.original_bits
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aggregated metrics covering all layers in a quantized model.
pub struct QuantizationSummary {
    pub total_layers: usize,
    pub total_parameters: usize,
    pub total_original_bits: u64,
    pub total_compressed_bits: u64,
    pub global_mse: f32,
    pub global_cosine_similarity: f32,
    pub global_kl_divergence: f32,
    pub average_residual_energy: f32,
    pub max_residual_energy: f32,
}

impl QuantizationSummary {
    pub fn compression_ratio(&self) -> f32 {
        if self.total_compressed_bits == 0 {
            return 0.0;
        }
        self.total_original_bits as f32 / self.total_compressed_bits as f32
    }

    pub fn bits_per_weight(&self) -> f32 {
        if self.total_parameters == 0 {
            return 0.0;
        }
        self.total_compressed_bits as f32 / self.total_parameters as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedModel {
    pub layers: Vec<QuantizedLayer>,
    pub summary: QuantizationSummary,
}

impl QuantizedModel {
    pub fn from_layers(layers: Vec<QuantizedLayer>) -> Self {
        let total_layers = layers.len();
        let total_parameters = layers.iter().map(|l| l.parameter_count()).sum();
        let total_original_bits = layers.iter().map(|l| l.original_bits()).sum();
        let total_compressed_bits = layers.iter().map(|l| l.compressed_bits()).sum();

        let (weighted_mse, weighted_cosine, weighted_kl, weight_sum) = layers.iter().fold(
            (0.0f64, 0.0f64, 0.0f64, 0.0f64),
            |(acc_mse, acc_cos, acc_kl, acc_weight), layer| {
                let weight = layer.parameter_count() as f64;
                (
                    acc_mse + (layer.metrics.mse as f64) * weight,
                    acc_cos + (layer.metrics.cosine_similarity as f64) * weight,
                    acc_kl + (layer.metrics.kl_divergence as f64) * weight,
                    acc_weight + weight,
                )
            },
        );

        let weight_sum = weight_sum.max(1.0);

        let (residual_sum, residual_max, residual_count) =
            layers
                .iter()
                .fold((0.0f64, 0.0f32, 0usize), |(sum, max_val, count), layer| {
                    let mut layer_sum = 0.0f64;
                    let mut layer_max = max_val;
                    for subspace in &layer.telemetry.subspaces {
                        layer_sum += subspace.residual_energy as f64;
                        if subspace.residual_energy > layer_max {
                            layer_max = subspace.residual_energy;
                        }
                    }
                    (
                        sum + layer_sum,
                        layer_max,
                        count + layer.telemetry.subspaces.len(),
                    )
                });

        let residual_count = residual_count.max(1);

        let summary = QuantizationSummary {
            total_layers,
            total_parameters,
            total_original_bits,
            total_compressed_bits,
            global_mse: (weighted_mse / weight_sum) as f32,
            global_cosine_similarity: (weighted_cosine / weight_sum) as f32,
            global_kl_divergence: (weighted_kl / weight_sum) as f32,
            average_residual_energy: (residual_sum / residual_count as f64) as f32,
            max_residual_energy: residual_max,
        };

        Self { layers, summary }
    }
}
