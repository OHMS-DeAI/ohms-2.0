//! Canonical manifest schema for NOVAQ artifacts.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    pub schema_version: String,
    pub created_at: DateTime<Utc>,
    pub generator: String,
    pub source_locator: String,
    pub quantization: QuantizationSection,
    pub chunks: Vec<ChunkEntry>,
    pub layers: BTreeMap<String, LayerEntry>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuantizationSection {
    pub config: serde_json::Value,
    pub summary: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChunkEntry {
    pub index: usize,
    pub path: String,
    pub bytes: usize,
    pub sha256: String,
    pub blake3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerEntry {
    pub mse: f32,
    pub cosine_similarity: f32,
    pub kl_divergence: f32,
    pub residual_energy: f32,
    pub bits_per_weight: f32,
    pub subspaces: Vec<serde_json::Value>,
}

impl Manifest {
    pub fn new(
        schema_version: impl Into<String>,
        generator: impl Into<String>,
        source_locator: impl Into<String>,
        quantization: QuantizationSection,
    ) -> Self {
        Self {
            schema_version: schema_version.into(),
            created_at: Utc::now(),
            generator: generator.into(),
            source_locator: source_locator.into(),
            quantization,
            chunks: Vec::new(),
            layers: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn insert_layer(&mut self, name: impl Into<String>, entry: LayerEntry) {
        self.layers.insert(name.into(), entry);
    }

    pub fn add_chunk(&mut self, entry: ChunkEntry) {
        self.chunks.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_schema() {
        let schema = schemars::schema_for!(Manifest);
        assert!(schema.schema.object.is_some());
    }
}
