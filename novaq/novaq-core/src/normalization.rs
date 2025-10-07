use std::collections::HashSet;

use itertools::iproduct;
use ndarray::{Array2, Axis};
use tracing::{debug, instrument, trace, warn};

use crate::error::{NovaQError, Result};
use crate::model::{LayerAnalysis, NormalizationRecord, OutlierEntry};
use crate::validation::validate_finite;

/// Epsilon for numerical stability in division operations.
/// Uses f32 precision to match tensor data type.
const EPSILON: f32 = 1e-6;

#[derive(Debug, Clone)]
pub struct Normalizer {
    percentile: f32,
}

impl Normalizer {
    pub fn new(percentile: f32) -> Result<Self> {
        if percentile <= 0.0 || percentile >= 1.0 {
            return Err(NovaQError::InvalidConfig(
                "outlier percentile must be in (0, 1)".to_string(),
            ));
        }
        Ok(Self { percentile })
    }

    #[allow(dead_code)]
    pub fn normalize(&self, weights: &Array2<f32>) -> Result<(Array2<f32>, NormalizationRecord)> {
        self.normalize_with_context(weights, None)
    }

    #[instrument(skip(self, weights, analysis))]
    pub fn normalize_with_context(
        &self,
        weights: &Array2<f32>,
        analysis: Option<&LayerAnalysis>,
    ) -> Result<(Array2<f32>, NormalizationRecord)> {
        if weights.is_empty() {
            return Err(NovaQError::EmptyTensor);
        }

        // CRITICAL: Validate input for NaN/Inf before any processing
        validate_finite(weights, "normalization input")?;

        let rows = weights.nrows();
        let cols = weights.ncols();

        let mut magnitudes: Vec<f32> = weights.iter().map(|v| v.abs()).collect();
        let mut percentile = self.percentile;
        if let Some(analysis) = analysis {
            if analysis.kurtosis > 3.0 {
                let scaling = (analysis.kurtosis / 3.0).min(5.0);
                percentile = (percentile * scaling).min(0.1);
            }
            if analysis.sparsity > 0.9 {
                percentile = (percentile * 0.5).max(1e-4);
            }
        }
        let outlier_count = ((magnitudes.len() as f32) * percentile).ceil() as usize;
        trace!(percentile, outlier_count, "computed outlier threshold");

        // FIXED: Use deterministic sorting instead of select_nth_unstable_by
        // This ensures reproducibility and correct handling of all values
        let mut outlier_threshold = f32::INFINITY;
        if outlier_count > 0 && outlier_count < magnitudes.len() {
            // Sort in ascending order deterministically
            magnitudes.sort_by(|a, b| {
                // All values are finite (validated above), so this is safe
                a.partial_cmp(b).unwrap()
            });
            let split_index = magnitudes.len() - outlier_count;
            outlier_threshold = magnitudes[split_index];
            debug!(
                outlier_threshold,
                outlier_count,
                "selected outlier threshold deterministically"
            );
        }
        let mut normalized = weights.clone();
        let mut means = vec![0.0f32; cols];
        let mut stds = vec![0.0f32; cols];
        let mut outliers = Vec::new();
        let mut outlier_set = HashSet::new();

        for (row, col) in iproduct!(0..rows, 0..cols) {
            let val = normalized[[row, col]];
            if val.abs() >= outlier_threshold {
                outliers.push(OutlierEntry {
                    row,
                    col,
                    value: val,
                });
                outlier_set.insert((row, col));
                normalized[[row, col]] = 0.0;
            }
        }

        for col in 0..cols {
            let column = normalized.index_axis(Axis(1), col);
            let mut sum = 0.0f64;
            let mut weight = 0usize;
            for (row, &val) in column.iter().enumerate() {
                if outlier_set.contains(&(row, col)) {
                    continue;
                }
                sum += val as f64;
                weight += 1;
            }

            let weight = weight.max(1);
            let mean = sum / weight as f64;
            let mut variance = 0.0f64;
            for (row, &val) in column.iter().enumerate() {
                if outlier_set.contains(&(row, col)) {
                    continue;
                }
                let diff = val as f64 - mean;
                variance += diff * diff;
            }
            let variance = variance / weight as f64;
            let std = variance.sqrt().max(EPSILON as f64);

            means[col] = mean as f32;
            stds[col] = std as f32;

            for row in 0..rows {
                if outlier_set.contains(&(row, col)) {
                    continue;
                }
                let orig = weights[[row, col]];
                normalized[[row, col]] = ((orig as f64 - mean) / std) as f32;
            }
        }

        debug!(
            total_outliers = outliers.len(),
            rows, cols, "masked normalization outliers"
        );

        debug!(total_outliers = outliers.len(), "normalization complete");

        Ok((
            normalized,
            NormalizationRecord {
                column_means: means,
                column_stds: stds,
                outliers,
            },
        ))
    }

    pub fn denormalize(
        &self,
        normalized: &Array2<f32>,
        record: &NormalizationRecord,
    ) -> Array2<f32> {
        let mut reconstructed = normalized.clone();
        let rows = normalized.nrows();
        let cols = normalized.ncols();

        for col in 0..cols {
            let mean = record.column_means[col];
            let std = record.column_stds[col].max(EPSILON);
            for row in 0..rows {
                let val = normalized[[row, col]];
                reconstructed[[row, col]] = val * std + mean;
            }
        }

        for outlier in &record.outliers {
            if outlier.row < rows && outlier.col < cols {
                reconstructed[[outlier.row, outlier.col]] = outlier.value;
            }
        }

        reconstructed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn normalization_round_trip() {
        let normalizer = Normalizer::new(0.1).unwrap();
        let weights = array![[1.0, 2.0, 10.0], [2.0, 3.0, 9.5], [3.0, 4.0, 11.0]];
        let (normalized, record) = normalizer.normalize(&weights).unwrap();
        let reconstructed = normalizer.denormalize(&normalized, &record);
        for (orig, rec) in weights.iter().zip(reconstructed.iter()) {
            assert!((orig - rec).abs() < 1e-4);
        }
    }
}
