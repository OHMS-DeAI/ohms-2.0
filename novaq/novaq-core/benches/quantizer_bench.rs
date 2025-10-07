use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use ndarray::Array2;
use novaq_core::{QuantizationConfig, Quantizer};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn synthetic_layer(rows: usize, cols: usize, seed: u64) -> Array2<f32> {
    let mut rng = StdRng::seed_from_u64(seed);
    Array2::from_shape_fn((rows, cols), |_| rng.gen_range(-3.0..3.0))
}

fn bench_quantizer(c: &mut Criterion) {
    let mut group = c.benchmark_group("quantizer_layers");
    let config = QuantizationConfig {
        max_subspace_dim: 16,
        level1_centroids: 16,
        level2_centroids: 8,
        ..QuantizationConfig::default()
    };
    let quantizer = Quantizer::new(config).expect("valid config");

    for &(rows, cols) in &[(64usize, 64usize), (128, 64), (128, 128)] {
        let id = BenchmarkId::new("layer", format!("{}x{}", rows, cols));
        group.bench_function(id, |b| {
            let weights = synthetic_layer(rows, cols, 42);
            b.iter_batched(
                || weights.clone(),
                |matrix| {
                    let _ = quantizer
                        .quantize_layer("bench", 0, &matrix)
                        .expect("quantization succeeds");
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(quantizer, bench_quantizer);
criterion_main!(quantizer);
