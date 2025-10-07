use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Client;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::instrument;

use crate::artifact::ArtifactWriter;
use crate::format::ModelLocator;
use crate::gguf::GgufLoader;
use crate::safetensors::SafeTensorsLoader;
use novaq_core::{QuantizationConfig, QuantizedModel};

#[derive(Debug, Clone)]
pub struct HuggingFaceConfig {
    pub client: Client,
    pub token: Option<String>,
}

impl Default for HuggingFaceConfig {
    fn default() -> Self {
        const USER_AGENT_VALUE: &str = "novaq-cli/0.1 (+https://github.com/ohms-labs/ohms)";
        let client = Client::builder()
            .user_agent(USER_AGENT_VALUE)
            .build()
            .unwrap_or_else(|_| Client::new());
        Self {
            client,
            token: None,
        }
    }
}

pub struct HuggingFaceLoader {
    cfg: HuggingFaceConfig,
    safetensors: SafeTensorsLoader,
    gguf: GgufLoader,
}

impl HuggingFaceLoader {
    pub fn new(cfg: HuggingFaceConfig, config: QuantizationConfig) -> Result<Self> {
        Ok(Self {
            cfg,
            safetensors: SafeTensorsLoader::new(config.clone())?,
            gguf: GgufLoader::new(config)?,
        })
    }

    #[instrument(skip(self, writer))]
    pub async fn load_snapshot(
        &self,
        locator: &ModelLocator,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel> {
        let spec = parse_snapshot(locator)?;

        let mut request = self
            .cfg
            .client
            .get(spec.download_url.clone())
            .header(ACCEPT, "application/octet-stream");
        if let Some(token) = &self.cfg.token {
            request = request.bearer_auth(token);
        }

        request = request.header(USER_AGENT, "novaq-cli/0.1");

        let response = request.send().await?.error_for_status()?;
        let tmp_path = unique_temp_path(&spec.file_path);
        let mut file = File::create(&tmp_path).await?;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }
        file.flush().await?;
        drop(file);

        let reader = tokio::fs::File::open(&tmp_path).await?;
        let mut reader = tokio::io::BufReader::new(reader);
        let extension = Path::new(&spec.file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let model = match extension.as_str() {
            "safetensors" => {
                self.safetensors
                    .load_from_reader(&mut reader, writer)
                    .await?
            }
            "gguf" => self.gguf.load_from_reader(&mut reader, writer).await?,
            other => {
                return Err(anyhow!(
                    "unsupported Hugging Face artifact extension: {}",
                    other
                ))
            }
        };
        tokio::fs::remove_file(&tmp_path).await.ok();
        Ok(model)
    }
}

#[cfg_attr(not(test), allow(dead_code))]
struct SnapshotSpec {
    repo: String,
    revision: String,
    file_path: String,
    download_url: String,
}

fn parse_snapshot(locator: &ModelLocator) -> Result<SnapshotSpec> {
    let url = locator
        .as_url()
        .context("expected huggingface URL with scheme")?;

    let scheme = url.scheme();
    if scheme != "hf" && scheme != "https" {
        return Err(anyhow!(
            "huggingface locator must use hf:// or https://huggingface.co"
        ));
    }

    if scheme == "https" && url.host_str() != Some("huggingface.co") {
        return Err(anyhow!("huggingface https locator must target huggingface.co"));
    }

    let mut segments: Vec<String> = url
        .path_segments()
        .map(|s| s.map(str::to_string).collect())
        .ok_or_else(|| anyhow!("invalid huggingface path"))?;

    let mut revision_from_query = false;
    let mut revision = url
        .query_pairs()
        .find(|(key, _)| key == "revision" || key == "ref")
        .map(|(_, value)| {
            revision_from_query = true;
            value.into_owned()
        })
        .unwrap_or_else(|| "main".to_string());

    let (owner, mut repo_name, mut remaining_segments): (String, String, Vec<String>) =
        if scheme == "hf" {
            let owner = url
                .host_str()
                .ok_or_else(|| anyhow!("huggingface locator missing owner"))?
                .to_string();
            if segments.is_empty() {
                return Err(anyhow!("huggingface locator missing repository name"));
            }
            let repo_part = segments.remove(0);
            (owner, repo_part, segments)
        } else {
            if segments.len() < 3 {
                return Err(anyhow!(
                    "huggingface snapshot requires owner, repo, and file path"
                ));
            }
            let owner = segments.remove(0);
            let repo_part = segments.remove(0);
            (owner, repo_part, segments)
        };

    if let Some(at_index) = repo_name.find('@') {
        let rev = repo_name[at_index + 1..].to_string();
        repo_name.truncate(at_index);
        revision = rev;
    }

    if !remaining_segments.is_empty() {
        match remaining_segments[0].as_str() {
            "resolve" | "blob" | "raw" => {
                if remaining_segments.len() < 3 {
                    return Err(anyhow!(
                        "huggingface locator missing revision or file name"
                    ));
                }
                if !revision_from_query {
                    revision = remaining_segments[1].clone();
                }
                remaining_segments.drain(0..2);
            }
            _ => {}
        }
    }

    if remaining_segments.is_empty() {
        return Err(anyhow!("huggingface snapshot missing file path"));
    }

    let file_path = remaining_segments.join("/");
    let repo = format!("{owner}/{repo_name}");
    let download_url = format!(
        "https://huggingface.co/{repo}/resolve/{revision}/{file_path}"
    );

    Ok(SnapshotSpec {
        repo,
        revision,
        file_path,
        download_url,
    })
}

fn unique_temp_path(source: &str) -> PathBuf {
    let suffix = Path::new(source)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext))
        .unwrap_or_default();
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros())
        .unwrap_or(0);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("novaq-hf-{}-{}{}", pid, micros, suffix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::ModelLocator;

    #[test]
    fn parses_hf_scheme_with_revision_override() {
        let locator = ModelLocator::new(
            "hf://meta-llama/Meta-Llama-3.1-8B-Instruct@dev/consolidated.safetensors",
        );
        let spec = parse_snapshot(&locator).expect("parse should succeed");
        assert_eq!(spec.repo, "meta-llama/Meta-Llama-3.1-8B-Instruct");
        assert_eq!(spec.revision, "dev");
        assert_eq!(spec.file_path, "consolidated.safetensors");
        assert!(spec
            .download_url
            .ends_with("/dev/consolidated.safetensors"));
    }

    #[test]
    fn parses_https_resolve_path() {
        let locator = ModelLocator::new("https://huggingface.co/meta-llama/Meta-Llama-3.1-8B-Instruct/resolve/main/consolidated.safetensors");
        let spec = parse_snapshot(&locator).expect("parse should succeed");
        assert_eq!(spec.repo, "meta-llama/Meta-Llama-3.1-8B-Instruct");
        assert_eq!(spec.revision, "main");
        assert_eq!(spec.file_path, "consolidated.safetensors");
    }

    #[test]
    fn parses_blob_path_and_revision_query() {
        let locator = ModelLocator::new("https://huggingface.co/meta-llama/Meta-Llama-3.1-8B-Instruct/blob/main/consolidated.gguf?revision=release");
        let spec = parse_snapshot(&locator).expect("parse should succeed");
        assert_eq!(spec.revision, "release");
        assert_eq!(spec.file_path, "consolidated.gguf");
    }
}
