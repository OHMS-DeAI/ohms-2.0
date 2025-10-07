use std::ops::Range;

use ndarray::Array2;
use tracing::{debug, instrument};

use crate::analysis::analyze_layer;
use crate::config::QuantizationConfig;
use crate::error::Result;
use crate::model::LayerAnalysis;

#[derive(Debug, Clone)]
pub struct SubspaceSpec {
    pub columns: Range<usize>,
    pub enable_stage2: bool,
    pub refinement_steps: usize,
}

#[derive(Debug)]
pub struct SubspacePlanner<'a> {
    config: &'a QuantizationConfig,
}

impl<'a> SubspacePlanner<'a> {
    pub fn new(config: &'a QuantizationConfig) -> Self {
        Self { config }
    }

    #[instrument(skip(self, analysis))]
    pub fn plan(&self, analysis: &LayerAnalysis) -> Vec<SubspaceSpec> {
        let mut plan = Vec::new();
        let mut start = 0usize;
        let cols = analysis.cols;
        let base_width = self.config.max_subspace_dim;
        let min_width = self.config.min_subspace_dim;

        while start < cols {
            let remaining = cols - start;
            let mut width = base_width.min(remaining);

            if analysis.kurtosis > 6.0 {
                width = ((width as f32) * 0.75).ceil() as usize;
            }
            if analysis.anisotropy > 10.0 {
                width = ((width as f32) * 0.5).ceil() as usize;
            }
            if analysis.sparsity > 0.85 {
                width = ((width as f32) * 1.25).ceil() as usize;
            }

            width = width.max(min_width);
            if width > remaining {
                width = remaining;
            }
            if width == 0 {
                width = remaining;
            }

            let enable_stage2 = analysis.kurtosis > 3.5 || analysis.anisotropy > 4.0;
            let refinement_steps = if enable_stage2 {
                self.config.max_refinement_steps
            } else {
                (self.config.max_refinement_steps / 2).max(1)
            };

            plan.push(SubspaceSpec {
                columns: start..(start + width),
                enable_stage2,
                refinement_steps,
            });
            debug!(
                columns.start = start,
                columns.end = start + width,
                enable_stage2,
                refinement_steps,
                "planned subspace"
            );
            start += width;
        }

        plan
    }
}

#[instrument(skip(config, weights))]
pub fn plan_subspaces(
    config: &QuantizationConfig,
    weights: &Array2<f32>,
) -> Result<(LayerAnalysis, Vec<SubspaceSpec>)> {
    let analysis = analyze_layer(weights)?;
    let planner = SubspacePlanner::new(config);
    let plan = planner.plan(&analysis);
    Ok((analysis, plan))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn planner_generates_covering_plan() {
        let weights = array![[1.0f32, 2.0, 3.0, 4.0], [2.0f32, 2.0, 2.0, 2.0]];
        let config = QuantizationConfig {
            max_subspace_dim: 3,
            min_subspace_dim: 2,
            ..QuantizationConfig::default()
        };
        let (analysis, plan) = plan_subspaces(&config, &weights).unwrap();
        assert_eq!(analysis.cols, 4);
        let covered: usize = plan.iter().map(|spec| spec.columns.len()).sum();
        assert_eq!(covered, 4);
        assert!(plan
            .iter()
            .all(|spec| spec.columns.len() >= config.min_subspace_dim
                || spec.columns.end == analysis.cols));
    }
}
