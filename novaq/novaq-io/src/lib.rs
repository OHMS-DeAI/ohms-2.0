//! Streaming model ingestion and artifact emission for NOVAQ.

pub mod artifact;
pub mod format;
pub mod gguf;
pub mod huggingface;
pub mod manifest;
pub mod safetensors;

pub use artifact::{ArtifactManifest, ArtifactWriter, ArtifactWriterConfig, ChunkInfo};
pub use format::{ModelFormat, ModelLocator};
pub use gguf::GgufLoader;
pub use huggingface::{HuggingFaceConfig, HuggingFaceLoader};
pub use manifest::assemble_manifest;
pub use safetensors::SafeTensorsLoader;

#[cfg(test)]
mod tests;
