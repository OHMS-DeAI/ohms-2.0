use std::path::Path;

use anyhow::{anyhow, Result};
use reqwest::Client;
use tracing::{info, instrument};

use crate::artifact::ArtifactWriter;
use crate::format::ModelLocator;
use crate::hf_api::{parse_repo_spec, HuggingFaceApiClient};
use crate::progress::{BandwidthMonitor, ProgressTracker};
use crate::streaming_safetensors_v2::StreamingSafeTensorsParserV2;
use novaq_core::{QuantizationConfig, QuantizedModel};

#[derive(Debug, Clone)]
pub struct HuggingFaceConfig {
    pub client: Client,
    pub token: Option<String>,
}

impl Default for HuggingFaceConfig {
    fn default() -> Self {
        const USER_AGENT_VALUE: &str = "novaq-cli/0.1 (+https://github.com/OHMS-DeAI/ohms-2.0)";
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
    api_client: HuggingFaceApiClient,
    config: QuantizationConfig,
    progress: Option<ProgressTracker>,
}

impl HuggingFaceLoader {
    pub fn new(
        hf_config: HuggingFaceConfig,
        quant_config: QuantizationConfig,
    ) -> Result<Self> {
        let api_client = HuggingFaceApiClient::new(hf_config.token.clone())?;
        Ok(Self {
            api_client,
            config: quant_config,
            progress: None,
        })
    }

    pub fn with_progress(mut self, progress: ProgressTracker) -> Self {
        self.progress = Some(progress);
        self
    }

    #[instrument(skip(self, writer))]
    pub async fn load_from_repo(
        &self,
        locator: &ModelLocator,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel> {
        let (repo_id, revision) = parse_repo_spec(locator.as_str())?;
        info!(repo_id, revision, "loading model from HuggingFace");

        let shards = self
            .api_client
            .detect_model_shards(&repo_id, &revision)
            .await?;

        if shards.is_empty() {
            return Err(anyhow!("no model files found in repository"));
        }

        info!(shard_count = shards.len(), "detected model shards");

        let mut all_layers = Vec::new();
        let bandwidth_monitor = BandwidthMonitor::new();

        for (idx, shard) in shards.iter().enumerate() {
            info!(
                shard = idx + 1,
                total = shards.len(),
                file = shard.path,
                size_mb = shard.size / 1_048_576,
                "processing shard"
            );

            let progress_bar = if let Some(ref progress) = self.progress {
                Some(progress.add_download_bar(&shard.path, shard.size))
            } else {
                None
            };

            let mut reader = self
                .api_client
                .download_file_streaming(&shard.download_url)
                .await?;

            let extension = Path::new(&shard.path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let model = match extension.as_str() {
                "safetensors" => {
                    let parser = StreamingSafeTensorsParserV2::new(
                        self.config.clone(),
                        self.progress.clone(),
                    )?;
                    parser.parse_and_quantize(&mut reader, writer).await?
                }
                _ => {
                    return Err(anyhow!(
                        "unsupported file extension: {} (expected .safetensors)",
                        extension
                    ))
                }
            };

            if let Some(ref pb) = progress_bar {
                pb.finish_with_message(format!("Completed {}", shard.path));
            }

            all_layers.extend(model.layers);
        }

        info!(
            total_layers = all_layers.len(),
            bandwidth_mbps = bandwidth_monitor.average_bandwidth_mbps(),
            total_mb = bandwidth_monitor.total_bytes() / 1_048_576,
            "completed streaming quantization"
        );

        Ok(QuantizedModel::from_layers(all_layers))
    }

    #[instrument(skip(self, writer))]
    pub async fn load_snapshot(
        &self,
        locator: &ModelLocator,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel> {
        self.load_from_repo(locator, writer).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HuggingFaceConfig::default();
        assert!(config.token.is_none());
    }
}
