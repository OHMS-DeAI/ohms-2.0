//! Streaming model ingestion and artifact emission for NOVAQ.

pub mod artifact;
pub mod format;
pub mod gguf;
pub mod hf_api;
pub mod huggingface;
pub mod manifest;
pub mod progress;
pub mod safetensors;
pub mod streaming_gguf;
pub mod streaming_reader;
pub mod streaming_safetensors;
pub mod streaming_safetensors_v2;

pub use artifact::{ArtifactManifest, ArtifactWriter, ArtifactWriterConfig, ChunkInfo};
pub use format::{ModelFormat, ModelLocator};
pub use gguf::GgufLoader;
pub use hf_api::{HuggingFaceApiClient, ModelFile, ModelSpec};
pub use huggingface::{HuggingFaceConfig, HuggingFaceLoader};
pub use manifest::assemble_manifest;
pub use progress::{BandwidthMonitor, ProgressTracker};
pub use safetensors::SafeTensorsLoader;
pub use streaming_gguf::StreamingGgufParser;
pub use streaming_safetensors::StreamingSafeTensorsParser;
pub use streaming_safetensors_v2::StreamingSafeTensorsParserV2;

#[cfg(test)]
mod tests;
