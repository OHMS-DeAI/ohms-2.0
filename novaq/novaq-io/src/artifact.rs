use std::{collections::BTreeMap, io::Write};

use anyhow::{Context, Result};
use blake3::Hasher as Blake3;
use sha2::{Digest, Sha256};
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct ArtifactWriterConfig {
    pub chunk_bytes: usize,
    pub output_dir: std::path::PathBuf,
}

impl Default for ArtifactWriterConfig {
    fn default() -> Self {
        Self {
            chunk_bytes: 1 << 20,
            output_dir: std::env::temp_dir(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkInfo {
    pub index: usize,
    pub path: String,
    pub sha256: String,
    pub blake3: String,
    pub bytes: usize,
}

#[derive(Debug, Default, Clone)]
pub struct ArtifactManifest {
    pub chunks: Vec<ChunkInfo>,
    pub metadata: BTreeMap<String, String>,
}

pub struct ArtifactWriter {
    cfg: ArtifactWriterConfig,
    manifest: ArtifactManifest,
}

impl ArtifactWriter {
    pub fn new(cfg: ArtifactWriterConfig) -> Self {
        Self {
            cfg,
            manifest: ArtifactManifest::default(),
        }
    }

    #[instrument(skip(self, bytes), fields(bytes = bytes.len()))]
    pub fn write_chunk(&mut self, bytes: &[u8]) -> Result<&ChunkInfo> {
        let index = self.manifest.chunks.len();
        let mut sha = Sha256::new();
        sha.update(bytes);
        let mut blake = Blake3::new();
        blake.update(bytes);

        let sha_hex = hex::encode(sha.finalize());
        let blake_hex = blake.finalize().to_hex().to_string();

        let filename = format!("chunk-{:05}-{}.bin", index, &sha_hex[..16]);
        let path = self.cfg.output_dir.join(&filename);
        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("unable to create artifact chunk at {}", path.display()))?;
        file.write_all(bytes)
            .with_context(|| format!("unable to write artifact chunk at {}", path.display()))?;

        self.manifest.chunks.push(ChunkInfo {
            index,
            path: filename,
            sha256: sha_hex,
            blake3: blake_hex,
            bytes: bytes.len(),
        });
        Ok(self.manifest.chunks.last().expect("just pushed"))
    }

    pub fn manifest(&self) -> &ArtifactManifest {
        &self.manifest
    }

    pub fn into_manifest(self) -> ArtifactManifest {
        self.manifest
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.manifest.metadata.insert(key.into(), value.into());
    }

    pub fn config(&self) -> &ArtifactWriterConfig {
        &self.cfg
    }

    pub fn output_dir(&self) -> &std::path::Path {
        &self.cfg.output_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn writes_chunks_and_hashes() {
        let dir = tempdir().unwrap();
        let mut writer = ArtifactWriter::new(ArtifactWriterConfig {
            chunk_bytes: 16,
            output_dir: dir.path().to_path_buf(),
        });
        let info = writer.write_chunk(b"hello world").unwrap();
        assert_eq!(info.index, 0);
        assert_eq!(info.bytes, 11);
        assert!(info.sha256.len() == 64);
        assert!(info.blake3.len() == 64);
        let path = dir.path().join(&info.path);
        assert!(path.exists());
    }
}
