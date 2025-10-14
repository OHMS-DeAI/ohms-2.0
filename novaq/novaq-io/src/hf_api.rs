use anyhow::{anyhow, Context, Result};
use futures::TryStreamExt;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;
use tracing::{debug, info, instrument, warn};

const HF_API_BASE: &str = "https://huggingface.co";
const MAX_RETRIES: u32 = 5;
const INITIAL_RETRY_DELAY_MS: u64 = 500;
const USER_AGENT_VALUE: &str = "novaq/0.1.0 (+https://github.com/OHMS-DeAI/ohms-2.0)";

#[derive(Debug, Clone)]
pub struct HuggingFaceApiClient {
    client: Client,
    token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoFile {
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    #[serde(alias = "rfilename")]
    pub path: Option<String>,
    pub size: Option<u64>,
    pub lfs: Option<LfsInfo>,
    #[serde(rename = "rfilename")]
    pub rfilename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LfsInfo {
    pub oid: Option<String>,
    pub size: Option<u64>,
    pub pointer_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "modelId")]
    pub model_id: Option<String>,
    pub sha: Option<String>,
    #[serde(default)]
    pub siblings: Vec<RepoFile>,
}

#[derive(Debug, Clone)]
pub struct ModelSpec {
    pub repo_id: String,
    pub revision: String,
    pub files: Vec<ModelFile>,
}

#[derive(Debug, Clone)]
pub struct ModelFile {
    pub path: String,
    pub size: u64,
    pub download_url: String,
}

impl HuggingFaceApiClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT_VALUE)
            .timeout(Duration::from_secs(3600))
            .connect_timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(0)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self { client, token })
    }

    #[instrument(skip(self))]
    pub async fn get_repo_info(&self, repo_id: &str, revision: &str) -> Result<RepoInfo> {
        let url = format!("{}/api/models/{}", HF_API_BASE, repo_id);
        let mut retries = 0;

        loop {
            let mut request = self.client.get(&url).header(USER_AGENT, USER_AGENT_VALUE);

            if let Some(ref token) = self.token {
                request = request.header(AUTHORIZATION, format!("Bearer {}", token));
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let repo_info: RepoInfo = response
                            .json()
                            .await
                            .context("failed to parse repository info")?;
                        return Ok(repo_info);
                    } else if response.status() == StatusCode::UNAUTHORIZED {
                        let error_text = response.text().await.unwrap_or_default();
                        return Err(anyhow!(
                            "Authentication required. This model may be private or gated.\n\
                             Please provide a HuggingFace token:\n\
                             1. Get a token from https://huggingface.co/settings/tokens\n\
                             2. Set it via: export HUGGINGFACE_TOKEN=\"hf_xxxxx\"\n\
                             3. Or use: --hf-token \"hf_xxxxx\"\n\
                             Error details: {}",
                            error_text
                        ));
                    } else if Self::should_retry(response.status()) && retries < MAX_RETRIES {
                        retries += 1;
                        let delay = Self::calculate_retry_delay(retries);
                        warn!(
                            status = %response.status(),
                            retry = retries,
                            delay_ms = delay.as_millis(),
                            "API request failed, retrying"
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        return Err(anyhow!(
                            "API request failed with status {}: {}",
                            status,
                            error_text
                        ));
                    }
                }
                Err(e) if retries < MAX_RETRIES => {
                    retries += 1;
                    let delay = Self::calculate_retry_delay(retries);
                    warn!(
                        error = %e,
                        retry = retries,
                        delay_ms = delay.as_millis(),
                        "Network error, retrying"
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn discover_model_files(
        &self,
        repo_id: &str,
        revision: &str,
        pattern: Option<&str>,
    ) -> Result<ModelSpec> {
        info!(repo_id, revision, "discovering model files");
        let repo_info = self.get_repo_info(repo_id, revision).await?;

        let mut files = Vec::new();
        for sibling in &repo_info.siblings {
            if let Some(ref ft) = sibling.file_type {
                if ft != "file" {
                    continue;
                }
            }

            let file_path = sibling
                .path
                .as_ref()
                .or(sibling.rfilename.as_ref())
                .ok_or_else(|| anyhow!("file entry missing path"))?;

            if let Some(pattern) = pattern {
                if !file_path.contains(pattern) {
                    continue;
                }
            }

            let size = sibling
                .size
                .or_else(|| sibling.lfs.as_ref().and_then(|lfs| lfs.size))
                .unwrap_or(0);

            if size == 0 {
                debug!("file has no size, likely will be retrieved from LFS: {}", file_path);
            }

            let download_url = format!(
                "{}/{}/resolve/{}/{}",
                HF_API_BASE, repo_id, revision, file_path
            );

            info!("adding file: {} (size: {} bytes, url: {})", file_path, size, download_url);

            files.push(ModelFile {
                path: file_path.clone(),
                size,
                download_url,
            });
        }

        Ok(ModelSpec {
            repo_id: repo_id.to_string(),
            revision: revision.to_string(),
            files,
        })
    }

    #[instrument(skip(self))]
    pub async fn detect_model_shards(
        &self,
        repo_id: &str,
        revision: &str,
    ) -> Result<Vec<ModelFile>> {
        let spec = self.discover_model_files(repo_id, revision, None).await?;

        let safetensors_shards: Vec<_> = spec
            .files
            .iter()
            .filter(|f| {
                f.path.ends_with(".safetensors")
                    && (f.path.contains("-of-") || f.path == "model.safetensors")
            })
            .cloned()
            .collect();

        let gguf_files: Vec<_> = spec
            .files
            .iter()
            .filter(|f| f.path.ends_with(".gguf"))
            .cloned()
            .collect();

        if !safetensors_shards.is_empty() {
            let mut sorted = safetensors_shards;
            sorted.sort_by(|a, b| a.path.cmp(&b.path));
            info!(count = sorted.len(), "detected safetensors shards");
            return Ok(sorted);
        }

        if !gguf_files.is_empty() {
            info!(count = gguf_files.len(), "detected GGUF files");
            return Ok(gguf_files);
        }

        Err(anyhow!(
            "no supported model files found in repository (searched for .safetensors and .gguf)"
        ))
    }

    #[instrument(skip(self))]
    pub async fn download_file_streaming(
        &self,
        url: &str,
    ) -> Result<impl AsyncRead + Unpin> {
        let mut retries = 0;

        loop {
            let mut request = self
                .client
                .get(url)
                .header(USER_AGENT, USER_AGENT_VALUE)
                .header("Accept", "application/octet-stream");

            if let Some(ref token) = self.token {
                request = request.header(AUTHORIZATION, format!("Bearer {}", token));
            }

            match request.send().await {
                Ok(mut response) => {
                    let status = response.status();
                    debug!("download response status: {}", status);
                    
                    if status.is_redirection() {
                        if let Some(location) = response.headers().get("location") {
                            let redirect_url = location.to_str().context("invalid redirect location")?.to_string();
                            info!("following redirect to: {}", redirect_url);
                            
                            let redirect_request = self.client
                                .get(&redirect_url)
                                .header(USER_AGENT, USER_AGENT_VALUE);
                            
                            response = redirect_request.send().await?;
                            let redirect_status = response.status();
                            
                            if !redirect_status.is_success() {
                                return Err(anyhow!("redirect failed with status {}", redirect_status));
                            }
                        } else {
                            return Err(anyhow!("redirect response missing location header"));
                        }
                    }
                    
                    if let Some(content_length) = response.content_length() {
                        info!("content length: {} bytes ({} MB)", content_length, content_length / 1_048_576);
                    } else {
                        warn!("no content-length header in response");
                    }
                    
                    if response.status().is_success() {
                        let stream = response
                            .bytes_stream()
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
                        
                        let reader = StreamReader::new(stream);
                        
                        info!("created async reader from response");
                        return Ok(reader);
                    } else if Self::should_retry(response.status()) && retries < MAX_RETRIES {
                        retries += 1;
                        let delay = Self::calculate_retry_delay(retries);
                        warn!(
                            status = %response.status(),
                            retry = retries,
                            "download failed, retrying"
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    } else {
                        return Err(anyhow!(
                            "download failed with status {}",
                            response.status()
                        ));
                    }
                }
                Err(e) if retries < MAX_RETRIES => {
                    retries += 1;
                    let delay = Self::calculate_retry_delay(retries);
                    warn!(error = %e, retry = retries, "download error, retrying");
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn should_retry(status: StatusCode) -> bool {
        matches!(
            status,
            StatusCode::REQUEST_TIMEOUT
                | StatusCode::TOO_MANY_REQUESTS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::BAD_GATEWAY
                | StatusCode::SERVICE_UNAVAILABLE
                | StatusCode::GATEWAY_TIMEOUT
        )
    }

    fn calculate_retry_delay(retry_count: u32) -> Duration {
        let delay_ms = INITIAL_RETRY_DELAY_MS * 2u64.pow(retry_count.saturating_sub(1));
        let max_delay_ms = 30_000;
        Duration::from_millis(delay_ms.min(max_delay_ms))
    }
}

pub fn parse_repo_spec(locator: &str) -> Result<(String, String)> {
    if let Some(url) = locator.strip_prefix("hf://") {
        let parts: Vec<&str> = url.split('@').collect();
        let repo_part = parts[0];
        let revision = parts.get(1).unwrap_or(&"main").to_string();

        let repo_id = repo_part.replace('/', "/");
        Ok((repo_id, revision))
    } else if locator.starts_with("https://huggingface.co/") {
        let without_prefix = locator
            .strip_prefix("https://huggingface.co/")
            .ok_or_else(|| anyhow!("invalid HF URL"))?;

        let parts: Vec<&str> = without_prefix.split('/').collect();
        if parts.len() < 2 {
            return Err(anyhow!("invalid HF URL format"));
        }

        let repo_id = format!("{}/{}", parts[0], parts[1]);
        let revision = if parts.len() > 3 && (parts[2] == "resolve" || parts[2] == "blob") {
            parts[3].to_string()
        } else {
            "main".to_string()
        };

        Ok((repo_id, revision))
    } else {
        Err(anyhow!(
            "locator must start with 'hf://' or 'https://huggingface.co/'"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hf_scheme() {
        let (repo, rev) = parse_repo_spec("hf://meta-llama/Llama-3.1-8B@main").unwrap();
        assert_eq!(repo, "meta-llama/Llama-3.1-8B");
        assert_eq!(rev, "main");
    }

    #[test]
    fn test_parse_hf_scheme_default_revision() {
        let (repo, rev) = parse_repo_spec("hf://openai/gpt-2").unwrap();
        assert_eq!(repo, "openai/gpt-2");
        assert_eq!(rev, "main");
    }

    #[test]
    fn test_parse_https_url() {
        let (repo, rev) =
            parse_repo_spec("https://huggingface.co/meta-llama/Llama-3.1-8B").unwrap();
        assert_eq!(repo, "meta-llama/Llama-3.1-8B");
        assert_eq!(rev, "main");
    }

    #[test]
    fn test_calculate_retry_delay() {
        assert_eq!(
            HuggingFaceApiClient::calculate_retry_delay(1),
            Duration::from_millis(500)
        );
        assert_eq!(
            HuggingFaceApiClient::calculate_retry_delay(2),
            Duration::from_millis(1000)
        );
        assert_eq!(
            HuggingFaceApiClient::calculate_retry_delay(3),
            Duration::from_millis(2000)
        );
        assert_eq!(
            HuggingFaceApiClient::calculate_retry_delay(10),
            Duration::from_millis(30_000)
        );
    }
}

