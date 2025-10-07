use std::{borrow::Cow, path::Path};

use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelFormat {
    SafeTensors,
    Gguf,
    Onnx,
    PyTorchStateDict,
    HuggingFaceSnapshot,
    Archive,
    Unknown,
}

impl ModelFormat {
    pub fn detect(path: &str) -> Self {
        let lowered = path.to_ascii_lowercase();
        if lowered.contains("hf://") || lowered.contains("huggingface.co") {
            Self::HuggingFaceSnapshot
        } else if lowered.ends_with(".safetensors") {
            Self::SafeTensors
        } else if lowered.ends_with(".gguf") {
            Self::Gguf
        } else if lowered.ends_with(".onnx") {
            Self::Onnx
        } else if lowered.ends_with(".bin") || lowered.ends_with(".pt") {
            Self::PyTorchStateDict
        } else if lowered.ends_with(".tar")
            || lowered.ends_with(".tar.gz")
            || lowered.ends_with(".zip")
        {
            Self::Archive
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelLocator {
    raw: String,
}

impl ModelLocator {
    pub fn new(uri: impl Into<String>) -> Self {
        Self { raw: uri.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub fn scheme(&self) -> Option<&str> {
        self.raw.split_once("://").map(|(scheme, _)| scheme)
    }

    pub fn validate(&self) -> Result<(), Cow<'static, str>> {
        if self.raw.trim().is_empty() {
            return Err(Cow::Borrowed("model locator cannot be empty"));
        }
        if let Some(scheme) = self.scheme() {
            if scheme.is_empty() {
                return Err(Cow::Borrowed("locator scheme is empty"));
            }
        }
        Ok(())
    }

    pub fn as_path(&self) -> Option<&Path> {
        if self.scheme().is_some() {
            return None;
        }
        Some(Path::new(self.raw.as_str()))
    }

    pub fn as_url(&self) -> Option<Url> {
        Url::parse(&self.raw).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_formats() {
        assert_eq!(
            ModelFormat::detect("weights.safetensors"),
            ModelFormat::SafeTensors
        );
        assert_eq!(ModelFormat::detect("model.gguf"), ModelFormat::Gguf);
        assert_eq!(
            ModelFormat::detect("snapshot.bin"),
            ModelFormat::PyTorchStateDict
        );
        assert_eq!(
            ModelFormat::detect(
                "https://huggingface.co/meta-llama/Meta-Llama-3.1-8B-Instruct/resolve/main/consolidated.safetensors"
            ),
            ModelFormat::HuggingFaceSnapshot
        );
        assert_eq!(
            ModelFormat::detect("hf://meta-llama/Meta-Llama-3.1-8B-Instruct/consolidated.gguf"),
            ModelFormat::HuggingFaceSnapshot
        );
    }

    #[test]
    fn validates_locator() {
        let locator = ModelLocator::new("hf://meta/llama-3");
        assert!(locator.validate().is_ok());
        assert_eq!(locator.scheme(), Some("hf"));
        let empty = ModelLocator::new("   ");
        assert!(empty.validate().is_err());
    }
}
