use anyhow::Result;
use novaq_core::QuantizationConfig;
use std::io::Cursor;
use tempfile::tempdir;

use crate::artifact::{ArtifactWriter, ArtifactWriterConfig};
use crate::gguf::GgufLoader;
use crate::manifest::assemble_manifest;
use crate::safetensors::SafeTensorsLoader;
use serde_json::json;

fn synthetic_safetensors() -> Vec<u8> {
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
    let pad = (8 - (bytes.len() % 8)) % 8;
    bytes.extend(std::iter::repeat(0u8).take(pad));
    bytes.extend_from_slice(&data_bytes);
    bytes
}

#[tokio::test]
async fn safetensors_loader_quantizes() -> Result<()> {
    let dir = tempdir()?;
    let mut writer = ArtifactWriter::new(ArtifactWriterConfig {
        chunk_bytes: 1 << 20,
        output_dir: dir.path().to_path_buf(),
    });
    let loader = SafeTensorsLoader::new(QuantizationConfig::default())?;
    let data = synthetic_safetensors();
    let cursor = Cursor::new(data);
    let mut reader = tokio::io::BufReader::new(cursor);
    let model = loader.load_from_reader(&mut reader, &mut writer).await?;
    assert!(!writer.manifest().chunks.is_empty());
    assert_eq!(model.layers.len(), 1);

    let manifest = assemble_manifest(
        &crate::format::ModelLocator::new("synthetic.safetensors"),
        "test",
        &QuantizationConfig::default(),
        &model,
        writer.manifest(),
    )?;
    assert_eq!(manifest.layers.len(), 1);
    assert_eq!(manifest.chunks.len(), 1);
    Ok(())
}

fn synthetic_gguf() -> Vec<u8> {
    let rows = 4usize;
    let cols = 4usize;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"GGUF");
    bytes.extend_from_slice(&1u32.to_le_bytes());
    bytes.extend_from_slice(&1u64.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());

    let name = b"linear.weight";
    bytes.extend_from_slice(&(name.len() as u32).to_le_bytes());
    bytes.extend_from_slice(name);
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&(rows as u64).to_le_bytes());
    bytes.extend_from_slice(&(cols as u64).to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());

    for value in 0..(rows * cols) {
        let f = value as f32;
        bytes.extend_from_slice(&f.to_le_bytes());
    }

    bytes
}

#[tokio::test]
async fn gguf_loader_quantizes() -> Result<()> {
    let dir = tempdir()?;
    let mut writer = ArtifactWriter::new(ArtifactWriterConfig {
        chunk_bytes: 1 << 20,
        output_dir: dir.path().to_path_buf(),
    });
    let loader = GgufLoader::new(QuantizationConfig::default())?;
    let data = synthetic_gguf();
    let cursor = Cursor::new(data);
    let mut reader = tokio::io::BufReader::new(cursor);
    let model = loader.load_from_reader(&mut reader, &mut writer).await?;
    assert_eq!(model.layers.len(), 1);
    assert!(!writer.manifest().chunks.is_empty());
    Ok(())
}
