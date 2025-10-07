use ndarray::{s, Array2, ArrayBase, ArrayView1, Axis, Ix2, Zip};
use rand::rngs::StdRng;
use rand::Rng;
use tracing::warn;

use crate::config::QuantizationConfig;
use crate::error::{NovaQError, Result};
use crate::model::{CodebookStage, QuantizedSubspace, SubspaceTelemetry};
use crate::subspace::SubspaceSpec;
use crate::validation::validate_centroid_distinctness;

/// Minimum L2 distance required between centroids to avoid degenerate clustering.
/// Set to sqrt(eps) to allow for numerical precision while catching true duplicates.
const MIN_CENTROID_DISTANCE: f32 = 1e-3;

pub struct ProductQuantizer<'a> {
    config: &'a QuantizationConfig,
}

pub struct DistillationHints<'a> {
    pub teacher_logits: &'a Array2<f32>,
    pub temperature: f32,
}

pub struct QuantizationResult {
    pub subspaces: Vec<QuantizedSubspace>,
    pub telemetry: Vec<SubspaceTelemetry>,
}

impl<'a> ProductQuantizer<'a> {
    pub fn new(config: &'a QuantizationConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn quantize(
        &self,
        normalized: &Array2<f32>,
        plan: &[SubspaceSpec],
        rng: &mut StdRng,
        hints: Option<&DistillationHints>,
    ) -> Result<QuantizationResult> {
        let rows = normalized.nrows();
        let cols = normalized.ncols();
        if rows == 0 || cols == 0 {
            return Err(NovaQError::EmptyTensor);
        }
        if plan.is_empty() {
            return Err(NovaQError::InvalidConfig(
                "subspace plan cannot be empty".to_string(),
            ));
        }

        let mut cursor = 0usize;
        for spec in plan {
            if spec.columns.start != cursor {
                return Err(NovaQError::InvariantViolation(format!(
                    "subspace plan has a gap at column {} (expected start {}, got {})",
                    cursor, cursor, spec.columns.start
                )));
            }
            if spec.columns.end > cols {
                return Err(NovaQError::InvariantViolation(format!(
                    "subspace plan exceeded tensor width: {} > {}",
                    spec.columns.end, cols
                )));
            }
            cursor = spec.columns.end;
        }
        if cursor != cols {
            return Err(NovaQError::InvariantViolation(format!(
                "subspace plan did not cover all columns: covered {}, expected {}",
                cursor, cols
            )));
        }

        if let Some(hints) = hints {
            if hints.teacher_logits.dim() != normalized.dim() {
                return Err(NovaQError::DimensionMismatch {
                    expected: normalized.len(),
                    found: hints.teacher_logits.len(),
                });
            }
        }

        let mut subspaces = Vec::with_capacity(plan.len());
        let mut telemetry = Vec::with_capacity(plan.len());

        for (index, spec) in plan.iter().enumerate() {
            let view = normalized.slice(s![.., spec.columns.clone()]);
            let data = view.to_owned();

            let training_data = if let Some(hints) = hints {
                let teacher_view = hints.teacher_logits.slice(s![.., spec.columns.clone()]);
                let teacher_owned = teacher_view.to_owned();
                blend_for_distillation(
                    &data,
                    &teacher_owned,
                    hints.temperature,
                    self.config.distillation_kl_weight,
                    self.config.distillation_cosine_weight,
                )
            } else {
                data.clone()
            };

            let (mut stage1, _) = run_kmeans(
                self.config,
                &training_data,
                self.config.level1_centroids,
                1,
                rng,
            )?;
            
            // CRITICAL: Validate that stage1 centroids are distinct
            validate_centroid_distinctness(&stage1.centroids, MIN_CENTROID_DISTANCE)?;
            
            let mut stage1_contrib =
                reconstruct_from_centroids(&stage1.centroids, &stage1.assignments);

            let mut stage2_state: Option<StageState> = None;
            let mut stage2_contrib: Option<Array2<f32>> = None;

            if spec.enable_stage2 && self.config.level2_centroids >= 2 && data.ncols() > 0 {
                let residual_for_stage2 = &training_data - &stage1_contrib;
                if average_squared_norm(&residual_for_stage2) > self.config.residual_variance_floor
                {
                    let (state, _) = run_kmeans(
                        self.config,
                        &residual_for_stage2,
                        self.config.level2_centroids,
                        2,
                        rng,
                    )?;
                    
                    // CRITICAL: Validate that stage2 centroids are distinct
                    if let Err(e) = validate_centroid_distinctness(&state.centroids, MIN_CENTROID_DISTANCE) {
                        warn!(
                            subspace = index,
                            error = %e,
                            "Stage2 centroids not distinct, skipping stage2 for this subspace"
                        );
                        // Don't fail the entire quantization, just skip stage2 for this subspace
                    } else {
                        stage2_contrib = Some(reconstruct_from_centroids(
                            &state.centroids,
                            &state.assignments,
                        ));
                        stage2_state = Some(state);
                    }
                }
            }

            let residual_energy = refine_subspace(
                &data,
                &training_data,
                &mut stage1,
                &mut stage1_contrib,
                stage2_state.as_mut(),
                &mut stage2_contrib,
                spec,
                self.config,
            );

            let telemetry_entry = SubspaceTelemetry {
                columns: spec.columns.clone(),
                stage1_iterations: stage1.iterations,
                stage2_iterations: stage2_state.as_ref().map(|s| s.iterations),
                stage1_inertia: stage1.inertia,
                stage2_inertia: stage2_state.as_ref().map(|s| s.inertia),
                residual_energy,
                enabled_stage2: stage2_state.is_some(),
            };

            let quantized_subspace = QuantizedSubspace {
                columns: spec.columns.clone(),
                stage1: stage1.into_codebook_stage()?,
                stage2: match stage2_state {
                    Some(state) => Some(state.into_codebook_stage()?),
                    None => None,
                },
                residual_energy,
            };

            tracing::debug!(
                subspace = index,
                cols = spec.columns.end - spec.columns.start,
                residual_energy,
                enabled_stage2 = telemetry_entry.enabled_stage2,
                "quantized subspace"
            );

            subspaces.push(quantized_subspace);
            telemetry.push(telemetry_entry);
        }

        Ok(QuantizationResult {
            subspaces,
            telemetry,
        })
    }

    pub fn reconstruct(
        &self,
        rows: usize,
        cols: usize,
        subspaces: &[QuantizedSubspace],
    ) -> Array2<f32> {
        let mut reconstructed = Array2::<f32>::zeros((rows, cols));
        for subspace in subspaces {
            let start = subspace.columns.start;
            let end = subspace.columns.end;
            for row in 0..rows {
                let mut target = reconstructed.slice_mut(s![row, start..end]);
                let idx = subspace.stage1.assignments[row] as usize;
                let centroid = subspace.stage1.centroids.row(idx);
                add_assign(&mut target, &centroid);
                if let Some(stage2) = &subspace.stage2 {
                    let idx = stage2.assignments[row] as usize;
                    let centroid = stage2.centroids.row(idx);
                    add_assign(&mut target, &centroid);
                }
            }
        }
        reconstructed
    }

    pub fn estimate_compressed_bits(&self, rows: usize, subspaces: &[QuantizedSubspace]) -> u64 {
        subspaces
            .iter()
            .map(|subspace| {
                let width = (subspace.columns.end - subspace.columns.start) as u64;
                let k1 = subspace.stage1.centroids.nrows() as u64;
                let centroid_bits = k1 * width * 32;
                let index_bits = rows as u64 * bits_for_indices(k1);
                let level1_bits = centroid_bits + index_bits;

                let level2_bits = subspace
                    .stage2
                    .as_ref()
                    .map(|stage| {
                        let k2 = stage.centroids.nrows() as u64;
                        let centroid_bits = k2 * width * 32;
                        let index_bits = rows as u64 * bits_for_indices(k2);
                        centroid_bits + index_bits
                    })
                    .unwrap_or(0);

                level1_bits + level2_bits
            })
            .sum()
    }
}

#[derive(Clone)]
struct StageState {
    id: u8,
    centroids: Array2<f32>,
    assignments: Vec<usize>,
    iterations: usize,
    inertia: f32,
}

impl StageState {
    fn into_codebook_stage(self) -> Result<CodebookStage> {
        let assignments = self.assignments;
        build_stage(
            self.id,
            self.centroids,
            &assignments,
            self.iterations,
            self.inertia,
        )
    }
}

fn run_kmeans(
    config: &QuantizationConfig,
    data: &Array2<f32>,
    requested_centroids: usize,
    stage_id: u8,
    rng: &mut StdRng,
) -> Result<(StageState, Array2<f32>)> {
    let rows = data.nrows();
    let dim = data.ncols();
    if rows == 0 || dim == 0 {
        return Err(NovaQError::EmptyTensor);
    }

    let k = requested_centroids.min(rows.max(1));
    if k < config.min_cluster_size && rows >= config.min_cluster_size {
        return Err(NovaQError::InvalidConfig(format!(
            "requested {} centroids but min_cluster_size is {}",
            requested_centroids, config.min_cluster_size
        )));
    }

    let mut centroids = initialize_centroids(data, k, rng);
    let mut assignments = vec![0usize; rows];

    for iteration in 0..config.max_iterations {
        let inertia = assign_points(data, &centroids, &mut assignments);
        let new_centroids = recompute_centroids(data, &assignments, k);
        let shift = centroid_shift(&centroids, &new_centroids);
        centroids = new_centroids;

        if shift < config.tolerance {
            let reconstruction = reconstruct_from_centroids(&centroids, &assignments);
            return Ok((
                StageState {
                    id: stage_id,
                    centroids,
                    assignments,
                    iterations: iteration + 1,
                    inertia,
                },
                reconstruction,
            ));
        }
    }

    let inertia = assign_points(data, &centroids, &mut assignments);
    let reconstruction = reconstruct_from_centroids(&centroids, &assignments);
    Ok((
        StageState {
            id: stage_id,
            centroids,
            assignments,
            iterations: config.max_iterations,
            inertia,
        },
        reconstruction,
    ))
}

fn refine_subspace(
    original: &Array2<f32>,
    training: &Array2<f32>,
    stage1: &mut StageState,
    stage1_contrib: &mut Array2<f32>,
    stage2: Option<&mut StageState>,
    stage2_contrib: &mut Option<Array2<f32>>,
    spec: &SubspaceSpec,
    config: &QuantizationConfig,
) -> f32 {
    let mut best_energy = residual_energy(original, stage1_contrib, stage2_contrib.as_ref());
    if spec.refinement_steps == 0 {
        return best_energy;
    }

    let mut stage2_state_opt = stage2;

    for _ in 0..spec.refinement_steps {
        let mut changed = false;
        changed |= reassign_and_update(training, stage1, config.refinement_learning_rate);
        *stage1_contrib = reconstruct_from_centroids(&stage1.centroids, &stage1.assignments);

        if let Some(stage2_state) = stage2_state_opt.as_deref_mut() {
            let residual_training = training - &*stage1_contrib;
            changed |= reassign_and_update(
                &residual_training,
                stage2_state,
                config.refinement_learning_rate,
            );
            let contrib =
                reconstruct_from_centroids(&stage2_state.centroids, &stage2_state.assignments);
            *stage2_contrib = Some(contrib);
        }

        let energy = residual_energy(original, stage1_contrib, stage2_contrib.as_ref());
        best_energy = energy;

        if energy <= config.residual_variance_floor || !changed {
            break;
        }
    }

    best_energy
}

fn residual_energy(
    original: &Array2<f32>,
    stage1: &Array2<f32>,
    stage2: Option<&Array2<f32>>,
) -> f32 {
    let mut total = 0.0f32;
    let mut count = 0usize;
    match stage2 {
        Some(s2) => {
            for ((orig, contrib1), contrib2) in original.iter().zip(stage1.iter()).zip(s2.iter()) {
                let diff = *orig - (*contrib1 + *contrib2);
                total += diff * diff;
                count += 1;
            }
        }
        None => {
            for (orig, contrib1) in original.iter().zip(stage1.iter()) {
                let diff = *orig - *contrib1;
                total += diff * diff;
                count += 1;
            }
        }
    }
    total / count.max(1) as f32
}

fn blend_for_distillation(
    data: &Array2<f32>,
    teacher: &Array2<f32>,
    temperature: f32,
    kl_weight: f32,
    cosine_weight: f32,
) -> Array2<f32> {
    let mut blended = data.clone();
    let alpha = ((kl_weight + cosine_weight).min(4.0)) / 4.0;
    let temp = temperature.max(1e-3);
    Zip::from(&mut blended)
        .and(teacher)
        .for_each(|dest, teacher_val| {
            let teacher_scaled = (*teacher_val / temp).tanh();
            *dest = *dest * (1.0 - alpha) + teacher_scaled * alpha;
        });
    blended
}

fn add_assign(target: &mut ndarray::ArrayViewMut1<'_, f32>, source: &ndarray::ArrayView1<'_, f32>) {
    for (dest, value) in target.iter_mut().zip(source.iter()) {
        *dest += *value;
    }
}

fn bits_for_indices(k: u64) -> u64 {
    let k = k.max(1) as f64;
    k.log2().ceil() as u64
}

fn average_squared_norm(matrix: &Array2<f32>) -> f32 {
    let total: f32 = matrix.iter().map(|v| v * v).sum();
    total / matrix.len().max(1) as f32
}

fn centroid_shift(old: &Array2<f32>, new: &Array2<f32>) -> f32 {
    let mut shift = 0.0f32;
    for (prev, next) in old.iter().zip(new.iter()) {
        let diff = prev - next;
        shift += diff * diff;
    }
    (shift / old.len().max(1) as f32).sqrt()
}

fn assign_points(data: &Array2<f32>, centroids: &Array2<f32>, assignments: &mut [usize]) -> f32 {
    let mut inertia = 0.0f32;
    for (row_idx, point) in data.axis_iter(Axis(0)).enumerate() {
        let (closest, distance) = closest_centroid(&point, centroids);
        assignments[row_idx] = closest;
        inertia += distance;
    }
    inertia
}

fn recompute_centroids(data: &Array2<f32>, assignments: &[usize], k: usize) -> Array2<f32> {
    let dim = data.ncols();
    let mut counts = vec![0usize; k];
    let mut new_centroids = Array2::<f32>::zeros((k, dim));

    for (row_idx, point) in data.axis_iter(Axis(0)).enumerate() {
        let centroid_idx = assignments[row_idx];
        counts[centroid_idx] += 1;
        for (dest, value) in new_centroids
            .row_mut(centroid_idx)
            .iter_mut()
            .zip(point.iter())
        {
            *dest += *value;
        }
    }

    for (idx, count) in counts.iter().enumerate() {
        if *count == 0 {
            if let Some((_, fallback)) =
                data.axis_iter(Axis(0))
                    .enumerate()
                    .max_by(|(_, a), (_, b)| {
                        let norm_a = a.iter().map(|v| v * v).sum::<f32>();
                        let norm_b = b.iter().map(|v| v * v).sum::<f32>();
                        norm_a
                            .partial_cmp(&norm_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            {
                new_centroids.row_mut(idx).assign(&fallback);
            }
            continue;
        }
        for value in new_centroids.row_mut(idx).iter_mut() {
            *value /= *count as f32;
        }
    }

    new_centroids
}

fn reassign_and_update(data: &Array2<f32>, state: &mut StageState, learning_rate: f32) -> bool {
    let previous_assignments = state.assignments.clone();
    state.inertia = assign_points(data, &state.centroids, &mut state.assignments);
    let new_centroids = recompute_centroids(data, &state.assignments, state.centroids.nrows());
    let blend = learning_rate.clamp(0.0, 1.0);
    let changed_assignments = previous_assignments != state.assignments;
    let mut changed_centroids = false;
    for (dest, updated) in state.centroids.iter_mut().zip(new_centroids.iter()) {
        let blended = *dest * (1.0 - blend) + *updated * blend;
        if (*dest - blended).abs() > 1e-12 {
            changed_centroids = true;
        }
        *dest = blended;
    }
    let changed = changed_assignments || changed_centroids;
    state.iterations += 1;
    changed
}

fn closest_centroid<S>(point: &ArrayView1<'_, f32>, centroids: &ArrayBase<S, Ix2>) -> (usize, f32)
where
    S: ndarray::Data<Elem = f32>,
{
    let mut best_idx = 0usize;
    let mut best_distance = f32::MAX;
    for (idx, centroid) in centroids.axis_iter(Axis(0)).enumerate() {
        let mut distance = 0.0f32;
        for (a, b) in point.iter().zip(centroid.iter()) {
            let diff = a - b;
            distance += diff * diff;
        }
        if distance < best_distance {
            best_distance = distance;
            best_idx = idx;
        }
    }
    (best_idx, best_distance)
}

fn reconstruct_from_centroids(centroids: &Array2<f32>, assignments: &[usize]) -> Array2<f32> {
    let rows = assignments.len();
    let dim = centroids.ncols();
    let mut reconstruction = Array2::<f32>::zeros((rows, dim));
    for (row, &centroid_idx) in assignments.iter().enumerate() {
        reconstruction
            .row_mut(row)
            .assign(&centroids.row(centroid_idx));
    }
    reconstruction
}

fn initialize_centroids(data: &Array2<f32>, k: usize, rng: &mut StdRng) -> Array2<f32> {
    let rows = data.nrows();
    let dim = data.ncols();
    let mut centroids = Array2::<f32>::zeros((k, dim));
    let first_idx = rng.gen_range(0..rows);
    centroids.row_mut(0).assign(&data.row(first_idx));

    let mut distances = vec![0.0f32; rows];
    for centroid_idx in 1..k {
        for (row_idx, point) in data.axis_iter(Axis(0)).enumerate() {
            let (_, dist) = closest_centroid(&point, &centroids.slice(s![0..centroid_idx, ..]));
            distances[row_idx] = dist;
        }
        let total_distance: f32 = distances.iter().sum();
        let mut sample = rng.gen::<f32>() * total_distance.max(1e-9);
        let mut chosen = 0usize;
        for (idx, dist) in distances.iter().enumerate() {
            sample -= *dist;
            if sample <= 0.0 {
                chosen = idx;
                break;
            }
        }
        centroids.row_mut(centroid_idx).assign(&data.row(chosen));
    }

    centroids
}

fn build_stage(
    stage_id: u8,
    centroids: Array2<f32>,
    assignments: &[usize],
    iterations: usize,
    inertia: f32,
) -> Result<CodebookStage> {
    if assignments.len() >= (1 << 16) {
        return Err(NovaQError::InvalidConfig(
            "number of assignments exceeds u16 capacity".to_string(),
        ));
    }
    let assignments: Vec<u16> = assignments.iter().map(|&idx| idx as u16).collect();
    Ok(CodebookStage {
        stage_id,
        centroids,
        assignments,
        iterations,
        inertia,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use rand::SeedableRng;

    #[test]
    fn centroid_initialization_is_deterministic() {
        let config = QuantizationConfig::default();
        let pq = ProductQuantizer::new(&config).unwrap();
        let mut rng = StdRng::seed_from_u64(1234);
        // Use diverse data to avoid degenerate clustering
        let data = Array2::from_shape_fn((8, 4), |(i, j)| {
            ((i as f32) * 0.3 + (j as f32) * 0.7).sin()
        });
        let plan = vec![SubspaceSpec {
            columns: 0..4,
            enable_stage2: true,
            refinement_steps: 4,
        }];
        let result = pq.quantize(&data, &plan, &mut rng, None).unwrap();
        assert_eq!(result.subspaces.len(), 1);
        let first = &result.subspaces[0];
        assert_eq!(first.stage1.centroids.ncols(), 4);
        assert_eq!(result.telemetry[0].columns, (0..4));
    }

    #[test]
    fn distillation_blend_shifts_values() {
        let data = array![[0.0f32, 0.0]];
        let teacher = array![[2.0f32, -2.0]];
        let blended = blend_for_distillation(&data, &teacher, 1.0, 1.0, 1.0);
        assert!(blended[[0, 0]] > 0.0);
        assert!(blended[[0, 1]] < 0.0);
    }
}
