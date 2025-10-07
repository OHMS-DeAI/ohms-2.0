use anyhow::{Context, Result};
use novaq_core::{QuantizationConfig, QuantizedModel};
use novaq_manifest::{ChunkEntry, LayerEntry, Manifest, QuantizationSection};
use serde_json::Value;

use crate::artifact::ArtifactManifest;
use crate::format::ModelLocator;

pub fn assemble_manifest(
    locator: &ModelLocator,
    generator: impl Into<String>,
    config: &QuantizationConfig,
    model: &QuantizedModel,
    artifact: &ArtifactManifest,
) -> Result<Manifest> {
    let quant_section = QuantizationSection {
        config: serde_json::to_value(config).context("serialize quantization config")?,
        summary: serde_json::to_value(&model.summary).context("serialize quantization summary")?,
    };

    let mut manifest = Manifest::new("1.0.0", generator, locator.as_str(), quant_section);

    for chunk in &artifact.chunks {
        manifest.add_chunk(ChunkEntry {
            index: chunk.index,
            path: chunk.path.clone(),
            bytes: chunk.bytes,
            sha256: chunk.sha256.clone(),
            blake3: chunk.blake3.clone(),
        });
    }

    manifest.metadata.extend(artifact.metadata.clone());

    for layer in &model.layers {
        let residual_avg = if layer.telemetry.subspaces.is_empty() {
            0.0
        } else {
            layer
                .telemetry
                .subspaces
                .iter()
                .map(|s| s.residual_energy)
                .sum::<f32>()
                / (layer.telemetry.subspaces.len() as f32)
        };
        let subspaces: Vec<Value> = layer
            .telemetry
            .subspaces
            .iter()
            .map(|s| serde_json::to_value(s).context("serialize subspace telemetry"))
            .collect::<Result<_, _>>()?;

        manifest.insert_layer(
            &layer.name,
            LayerEntry {
                mse: layer.metrics.mse,
                cosine_similarity: layer.metrics.cosine_similarity,
                kl_divergence: layer.metrics.kl_divergence,
                residual_energy: residual_avg,
                bits_per_weight: layer.metrics.bits_per_weight,
                subspaces,
            },
        );
    }

    Ok(manifest)
}
