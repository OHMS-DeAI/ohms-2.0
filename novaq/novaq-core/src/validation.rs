//! Input validation and numerical stability checks for NOVAQ quantization.

use ndarray::Array2;

use crate::error::{NovaQError, Result};

/// Validates that all elements in a tensor are finite (not NaN or Inf).
/// 
/// This is critical for ensuring numerical stability throughout the quantization pipeline.
/// NaN or Inf values will cause catastrophic failures in statistical calculations,
/// centroid computations, and metric evaluations.
///
/// # Arguments
/// * `tensor` - Input tensor to validate
/// * `context` - Human-readable context for error messages (e.g., "layer3.weight")
///
/// # Errors
/// Returns `NovaQError::InvalidInput` if any NaN or Inf values are detected.
pub fn validate_finite(tensor: &Array2<f32>, context: &str) -> Result<()> {
    for (row, col_data) in tensor.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col, &value) in col_data.iter().enumerate() {
            if value.is_nan() {
                return Err(NovaQError::InvalidInput {
                    reason: format!(
                        "NaN detected in {} at position ({}, {})",
                        context, row, col
                    ),
                });
            }
            if value.is_infinite() {
                return Err(NovaQError::InvalidInput {
                    reason: format!(
                        "Infinite value ({}) detected in {} at position ({}, {})",
                        value, context, row, col
                    ),
                });
            }
        }
    }
    Ok(())
}

/// Validates that a set of centroids are sufficiently distinct from each other.
///
/// Duplicate or near-duplicate centroids indicate degenerate k-means clustering
/// and will produce suboptimal compression quality.
///
/// # Arguments
/// * `centroids` - Matrix where each row is a centroid
/// * `min_distance` - Minimum L2 distance required between any pair of centroids
///
/// # Errors
/// Returns `NovaQError::InvariantViolation` if any two centroids are too close.
pub fn validate_centroid_distinctness(
    centroids: &Array2<f32>,
    min_distance: f32,
) -> Result<()> {
    let k = centroids.nrows();
    if k < 2 {
        return Ok(());
    }

    for i in 0..k {
        for j in (i + 1)..k {
            let mut dist_sq = 0.0f32;
            for (a, b) in centroids.row(i).iter().zip(centroids.row(j).iter()) {
                let diff = a - b;
                dist_sq += diff * diff;
            }
            let dist = dist_sq.sqrt();
            if dist < min_distance {
                return Err(NovaQError::InvariantViolation(format!(
                    "Centroids {} and {} are too close (distance={}, minimum={}). This indicates degenerate clustering.",
                    i, j, dist, min_distance
                )));
            }
        }
    }
    Ok(())
}

/// Validates that all values in a probability distribution sum to approximately 1.0.
///
/// Used to verify that KL divergence calculations are operating on valid probability distributions.
pub fn validate_probability_distribution(probs: &[f64], tolerance: f64) -> Result<()> {
    let sum: f64 = probs.iter().sum();
    if (sum - 1.0).abs() > tolerance {
        return Err(NovaQError::InvariantViolation(format!(
            "Probability distribution sums to {}, expected 1.0 (tolerance={})",
            sum, tolerance
        )));
    }
    for (idx, &p) in probs.iter().enumerate() {
        if p < 0.0 {
            return Err(NovaQError::InvariantViolation(format!(
                "Negative probability {} at index {}",
                p, idx
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn validate_finite_accepts_normal_values() {
        let tensor = array![[1.0f32, 2.0, 3.0], [-1.0, 0.0, 5.5]];
        assert!(validate_finite(&tensor, "test").is_ok());
    }

    #[test]
    fn validate_finite_rejects_nan() {
        let tensor = array![[1.0f32, f32::NAN, 3.0]];
        let result = validate_finite(&tensor, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("NaN"));
    }

    #[test]
    fn validate_finite_rejects_infinity() {
        let tensor = array![[1.0f32, f32::INFINITY, 3.0]];
        let result = validate_finite(&tensor, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Infinite"));
    }

    #[test]
    fn validate_centroid_distinctness_accepts_distinct_centroids() {
        let centroids = array![[0.0f32, 0.0], [10.0, 10.0], [20.0, 0.0]];
        assert!(validate_centroid_distinctness(&centroids, 1.0).is_ok());
    }

    #[test]
    fn validate_centroid_distinctness_rejects_duplicates() {
        let centroids = array![[1.0f32, 2.0], [1.0, 2.0]];
        let result = validate_centroid_distinctness(&centroids, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn validate_probability_distribution_accepts_valid() {
        let probs = vec![0.3, 0.3, 0.4];
        assert!(validate_probability_distribution(&probs, 1e-6).is_ok());
    }

    #[test]
    fn validate_probability_distribution_rejects_invalid_sum() {
        let probs = vec![0.3, 0.3, 0.3];
        let result = validate_probability_distribution(&probs, 1e-6);
        assert!(result.is_err());
    }

    #[test]
    fn validate_probability_distribution_rejects_negative() {
        let probs = vec![0.5, -0.1, 0.6];
        let result = validate_probability_distribution(&probs, 1e-6);
        assert!(result.is_err());
    }
}

