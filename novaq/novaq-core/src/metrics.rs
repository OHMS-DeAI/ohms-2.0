use ndarray::Array2;

use crate::model::LayerMetrics;

const EPS: f64 = 1e-12;

pub fn compute_layer_metrics(
    original: &Array2<f32>,
    reconstructed: &Array2<f32>,
    compressed_bits: u64,
) -> LayerMetrics {
    assert_eq!(original.shape(), reconstructed.shape());

    let mut mse_acc = 0.0f64;
    let mut dot = 0.0f64;
    let mut norm_orig = 0.0f64;
    let mut norm_rec = 0.0f64;

    for (o, r) in original.iter().zip(reconstructed.iter()) {
        let diff = (*o as f64) - (*r as f64);
        mse_acc += diff * diff;
        dot += (*o as f64) * (*r as f64);
        norm_orig += (*o as f64) * (*o as f64);
        norm_rec += (*r as f64) * (*r as f64);
    }

    let len = original.len().max(1) as f64;
    let mse = (mse_acc / len) as f32;
    let cosine = (dot / ((norm_orig.sqrt() + EPS) * (norm_rec.sqrt() + EPS))) as f32;

    let kl = kl_divergence(original, reconstructed) as f32;

    let original_bits = (original.len() as u64) * 32;
    let bits_per_weight = if original.len() == 0 {
        0.0
    } else {
        compressed_bits as f32 / original.len() as f32
    };

    LayerMetrics {
        mse,
        cosine_similarity: cosine,
        kl_divergence: kl,
        original_bits,
        compressed_bits,
        bits_per_weight,
    }
}

fn kl_divergence(original: &Array2<f32>, reconstructed: &Array2<f32>) -> f64 {
    let p = to_distribution(original);
    let q = to_distribution(reconstructed);
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| {
            let q = (*q).max(EPS);
            (*p) * ((*p) / q).ln()
        })
        .sum::<f64>()
}

fn to_distribution(tensor: &Array2<f32>) -> Vec<f64> {
    let mut values: Vec<f64> = tensor.iter().map(|v| *v as f64).collect();
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut sum = 0.0f64;
    for value in values.iter_mut() {
        *value = (*value - max).exp();
        sum += *value;
    }
    if sum <= 0.0 {
        let uniform = 1.0 / values.len().max(1) as f64;
        values.fill(uniform);
        values
    } else {
        for value in values.iter_mut() {
            *value /= sum;
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn metrics_are_reasonable() {
        let orig = array![[1.0f32, 2.0, 3.0]];
        let rec = array![[1.0f32, 1.9, 2.9]];
        let metrics = compute_layer_metrics(&orig, &rec, 12);
        assert!(metrics.mse < 0.02);
        assert!(metrics.cosine_similarity > 0.99);
    }
}
