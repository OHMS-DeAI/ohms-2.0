use criterion::{criterion_group, criterion_main, Criterion};
use novaq_core::QuantizationConfig;
use novaq_io::{ArtifactWriter, ArtifactWriterConfig, SafeTensorsLoader};
use serde_json::json;
use tempfile::tempdir;
use tokio::{io::Cursor, runtime::Runtime};

fn synthetic_payload() -> Vec<u8> {
    let rows = 4;
    let cols = 4;
    let floats: Vec<f32> = (0..(rows * cols)).map(|v| v as f32).collect();
    let mut data_bytes = Vec::with_capacity(floats.len() * 4);
    for value in &floats {
        data_bytes.extend_from_slice(&value.to_le_bytes());
    }
    let header = json!({
        "linear.weight": {
            "dtype": "F32",
            "shape": [rows, cols],
            "data_offsets": [0, data_bytes.len()],
            "data_type": "F32"
        }
    })
    .to_string();
    let mut bytes = Vec::new();
    let header_len = header.as_bytes().len() as u64;
    bytes.extend_from_slice(&header_len.to_le_bytes());
    bytes.extend_from_slice(header.as_bytes());
    bytes.extend_from_slice(&data_bytes);
    bytes
}

fn bench_safetensors_loader(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let payload = synthetic_payload();
    let loader = SafeTensorsLoader::new(QuantizationConfig::default()).unwrap();

    c.bench_function("safetensors_small", |b| {
        b.to_async(&runtime).iter(|| async {
            let dir = tempdir().unwrap();
            let mut writer = ArtifactWriter::new(ArtifactWriterConfig {
                chunk_bytes: 1 << 20,
                output_dir: dir.path().to_path_buf(),
            });
            let cursor = Cursor::new(payload.clone());
            let mut reader = tokio::io::BufReader::new(cursor);
            let _model = loader
                .load_from_reader(&mut reader, &mut writer)
                .await
                .unwrap();
        });
    });
}

criterion_group!(benches, bench_safetensors_loader);
criterion_main!(benches);
