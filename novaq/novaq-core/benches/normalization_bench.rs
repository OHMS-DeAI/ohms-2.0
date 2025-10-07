use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use ndarray::Array2;
use novaq_core::Normalizer;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn synthetic_layer(rows: usize, cols: usize, seed: u64) -> Array2<f32> {
    let mut rng = StdRng::seed_from_u64(seed);
    Array2::from_shape_fn((rows, cols), |_| rng.gen_range(-5.0..5.0))
}

fn bench_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalization");
    let normalizer = Normalizer::new(0.01).expect("valid percentile");

    for &(rows, cols) in &[(64usize, 64usize), (128, 128), (256, 128)] {
        let id = BenchmarkId::new("layer", format!("{}x{}", rows, cols));
        group.bench_function(id, |b| {
            let weights = synthetic_layer(rows, cols, 13);
            b.iter_batched(
                || weights.clone(),
                |matrix| {
                    let _ = normalizer
                        .normalize(&matrix)
                        .expect("normalization succeeds");
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(normalization, bench_normalization);
criterion_main!(normalization);
