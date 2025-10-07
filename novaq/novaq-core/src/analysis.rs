use ndarray::{Array2, Axis};

use crate::error::{NovaQError, Result};
use crate::model::LayerAnalysis;

const EPS: f64 = 1e-12;

pub fn analyze_layer(weights: &Array2<f32>) -> Result<LayerAnalysis> {
    if weights.is_empty() {
        return Err(NovaQError::EmptyTensor);
    }

    let rows = weights.nrows();
    let cols = weights.ncols();
    let len = (rows * cols) as f64;

    let mut sum = 0.0f64;
    let mut sumsq = 0.0f64;
    let mut zero_count = 0usize;
    let mut max_abs = 0.0f32;

    for &value in weights.iter() {
        let value64 = value as f64;
        sum += value64;
        sumsq += value64 * value64;
        if value.abs() <= 1e-8 {
            zero_count += 1;
        }
        if value.abs() > max_abs {
            max_abs = value.abs();
        }
    }

    let mean = sum / len;
    let second_moment = sumsq / len;
    let variance = (second_moment - mean * mean).max(0.0);
    let std = variance.sqrt();

    let mut third_central_moment = 0.0f64;
    let mut fourth_central_moment = 0.0f64;
    for &value in weights.iter() {
        let diff = value as f64 - mean;
        third_central_moment += diff.powi(3);
        fourth_central_moment += diff.powi(4);
    }
    third_central_moment /= len;
    fourth_central_moment /= len;

    let std_cubed = (std.powi(3)).max(EPS);
    let std_fourth = (std.powi(4)).max(EPS);
    let skewness = third_central_moment / std_cubed;
    let kurtosis = fourth_central_moment / std_fourth;

    let l2_norm = sumsq.sqrt();
    let sparsity = (zero_count as f64 / len) as f32;

    let mut column_variances = Vec::with_capacity(cols);
    let mut min_col_var = f32::MAX;
    let mut max_col_var = 0.0f32;
    for col in 0..cols {
        let column = weights.index_axis(Axis(1), col);
        let mut col_sum = 0.0f64;
        let mut col_sumsq = 0.0f64;
        for &value in column.iter() {
            let value64 = value as f64;
            col_sum += value64;
            col_sumsq += value64 * value64;
        }
        let len_col = column.len() as f64;
        let mean_col = col_sum / len_col;
        let variance_col = (col_sumsq / len_col) - mean_col * mean_col;
        let variance_col = variance_col.max(0.0) as f32;
        if variance_col < min_col_var {
            min_col_var = variance_col;
        }
        if variance_col > max_col_var {
            max_col_var = variance_col;
        }
        column_variances.push(variance_col);
    }
    if min_col_var <= 0.0 {
        min_col_var = EPS as f32;
    }
    let anisotropy = if max_col_var <= EPS as f32 {
        1.0
    } else {
        max_col_var / (min_col_var + EPS as f32)
    };

    let mut row_variances = Vec::with_capacity(rows);
    for row in 0..rows {
        let row_view = weights.index_axis(Axis(0), row);
        let mut row_sum = 0.0f64;
        let mut row_sumsq = 0.0f64;
        for &value in row_view.iter() {
            let value64 = value as f64;
            row_sum += value64;
            row_sumsq += value64 * value64;
        }
        let len_row = row_view.len() as f64;
        let mean_row = row_sum / len_row;
        let variance_row = (row_sumsq / len_row) - mean_row * mean_row;
        row_variances.push(variance_row.max(0.0) as f32);
    }

    Ok(LayerAnalysis {
        rows,
        cols,
        mean: mean as f32,
        variance: variance as f32,
        std: std as f32,
        kurtosis: kurtosis as f32,
        skewness: skewness as f32,
        sparsity,
        max_abs,
        l2_norm: l2_norm as f32,
        anisotropy,
        column_variances,
        row_variances,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn analysis_computes_reasonable_statistics() {
        let weights = array![[1.0f32, 2.0, 3.0], [0.0f32, -1.0, 4.0], [2.0f32, 2.0, 2.0]];
        let analysis = analyze_layer(&weights).unwrap();
        assert_eq!(analysis.rows, 3);
        assert_eq!(analysis.cols, 3);
        assert!(analysis.variance > 0.0);
        assert!(analysis.max_abs >= 4.0);
        assert_eq!(analysis.column_variances.len(), 3);
        assert_eq!(analysis.row_variances.len(), 3);
        assert!(analysis.anisotropy >= 1.0);
    }
}
