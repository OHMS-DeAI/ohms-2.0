use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use ndarray::Array2;
use novaq_core::{QuantizationConfig, Quantizer};
use novaq_io::{
    assemble_manifest, ArtifactWriter, ArtifactWriterConfig, GgufLoader, HuggingFaceConfig,
    HuggingFaceLoader, ModelFormat, ModelLocator, ProgressTracker, SafeTensorsLoader,
};
use novaq_manifest::Manifest;
use rand::{Rng, SeedableRng};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about = "NOVAQ Toolkit CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CompressMatrix {
        #[arg(long, default_value_t = 32)]
        rows: usize,
        #[arg(long, default_value_t = 32)]
        cols: usize,
        #[arg(long, default_value_t = 7)]
        seed: u64,
    },
    Compress {
        #[arg(long)]
        input: String,

        #[arg(long, default_value = "artifacts")]
        output: PathBuf,

        #[arg(long)]
        hf_token: Option<String>,

        #[arg(long, default_value_t = 1.5)]
        target_bits: f32,

        #[arg(long, default_value_t = 16)]
        level1_centroids: usize,

        #[arg(long, default_value_t = 8)]
        level2_centroids: usize,

        #[arg(long, default_value_t = 16)]
        max_subspace_dim: usize,

        #[arg(long)]
        disable_progress: bool,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::CompressMatrix { rows, cols, seed } => {
            run_compress_matrix(rows, cols, seed)?;
        }
        Commands::Compress {
            input,
            output,
            hf_token,
            target_bits,
            level1_centroids,
            level2_centroids,
            max_subspace_dim,
            disable_progress,
        } => {
            run_compress_model(
                &input,
                &output,
                hf_token,
                target_bits,
                level1_centroids,
                level2_centroids,
                max_subspace_dim,
                !disable_progress,
            )?;
        }
    }
    Ok(())
}

fn run_compress_matrix(rows: usize, cols: usize, seed: u64) -> Result<()> {
    if rows == 0 || cols == 0 {
        return Err(anyhow!("rows and cols must be greater than zero"));
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let matrix = Array2::from_shape_fn((rows, cols), |_| rng.gen_range(-2.0..2.0));

    let quantizer = Quantizer::new(QuantizationConfig::default())?;
    let layer = quantizer.quantize_layer("synthetic", 0, &matrix)?;
    let summary = serde_json::to_string_pretty(&layer.metrics)?;

    info!(
        layer = layer.name,
        mse = layer.metrics.mse,
        cosine = layer.metrics.cosine_similarity,
        kl = layer.metrics.kl_divergence,
        bits_per_weight = layer.metrics.bits_per_weight,
        "quantization completed"
    );

    println!("NOVAQ synthetic compression metrics:\n{}", summary);
    Ok(())
}

fn run_compress_model(
    input: &str,
    output: &Path,
    hf_token: Option<String>,
    target_bits: f32,
    level1_centroids: usize,
    level2_centroids: usize,
    max_subspace_dim: usize,
    enable_progress: bool,
) -> Result<()> {
    let locator = ModelLocator::new(input);
    locator
        .validate()
        .map_err(|err| anyhow!(err.into_owned()))?;
    std::fs::create_dir_all(output)?;

    let format = ModelFormat::detect(locator.as_str());
    let mut writer = ArtifactWriter::new(ArtifactWriterConfig {
        chunk_bytes: 1 << 20,
        output_dir: output.to_path_buf(),
    });
    writer.set_metadata("source", locator.as_str());
    writer.set_metadata("format", format_string(format));

    let config = QuantizationConfig {
        target_bits,
        level1_centroids,
        level2_centroids,
        max_subspace_dim,
        ..QuantizationConfig::default()
    };

    let runtime = tokio::runtime::Runtime::new()?;
    let model = match format {
        ModelFormat::SafeTensors => {
            let loader = SafeTensorsLoader::new(config.clone())?;
            runtime.block_on(async {
                let file = tokio::fs::File::open(locator.as_str()).await?;
                let mut reader = tokio::io::BufReader::new(file);
                loader.load_from_reader(&mut reader, &mut writer).await
            })?
        }
        ModelFormat::Gguf => {
            let loader = GgufLoader::new(config.clone())?;
            runtime.block_on(async {
                let file = tokio::fs::File::open(locator.as_str()).await?;
                let mut reader = tokio::io::BufReader::new(file);
                loader.load_from_reader(&mut reader, &mut writer).await
            })?
        }
        ModelFormat::HuggingFaceSnapshot => {
            let token = hf_token
                .or_else(|| std::env::var("HUGGINGFACE_TOKEN").ok())
                .or_else(|| std::env::var("HF_TOKEN").ok());

            let mut hf_cfg = HuggingFaceConfig::default();
            hf_cfg.token = token;

            let mut loader = HuggingFaceLoader::new(hf_cfg, config.clone())?;

            if enable_progress {
                let progress = ProgressTracker::new();
                loader = loader.with_progress(progress);
            }

            runtime.block_on(async { loader.load_from_repo(&locator, &mut writer).await })?
        }
        _ => {
            return Err(anyhow!(
                "unsupported model format: {:?}. Supported inputs are safetensors, gguf, and huggingface snapshots",
                format
            ));
        }
    };

    let artifact_manifest = writer.into_manifest();
    let manifest = assemble_manifest(
        &locator,
        format!("novaq-cli/{}", env!("CARGO_PKG_VERSION")),
        &config,
        &model,
        &artifact_manifest,
    )?;

    write_manifest(output, &manifest)?;
    print_summary(&model);

    Ok(())
}

fn write_manifest(output: &Path, manifest: &Manifest) -> Result<()> {
    let path = output.join("manifest.json");
    let file = File::create(&path)?;
    serde_json::to_writer_pretty(file, manifest)?;
    println!("wrote manifest to {}", path.display());
    Ok(())
}

fn print_summary(model: &novaq_core::QuantizedModel) {
    let summary = &model.summary;
    println!(
        "compressed {} layers → avg bits/weight {:.3}, mse {:.6}, cosine {:.6}",
        summary.total_layers,
        summary.bits_per_weight(),
        summary.global_mse,
        summary.global_cosine_similarity
    );
}

fn format_string(format: ModelFormat) -> &'static str {
    match format {
        ModelFormat::SafeTensors => "safetensors",
        ModelFormat::Gguf => "gguf",
        ModelFormat::Onnx => "onnx",
        ModelFormat::PyTorchStateDict => "pytorch_bin",
        ModelFormat::HuggingFaceSnapshot => "huggingface",
        ModelFormat::Archive => "archive",
        ModelFormat::Unknown => "unknown",
    }
}
