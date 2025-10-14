# NovaQ: Neural Optimal Vector Adaptive Quantization

NovaQ is a production-ready quantization engine for large language models, featuring **streaming quantization** directly from HuggingFace without intermediate downloads.

## Features

### 🚀 Streaming Quantization
- **Zero download required** - Quantize models directly from HuggingFace
- **Memory efficient** - Process multi-GB models with minimal RAM
- **Automatic shard detection** - Handles sharded SafeTensors and GGUF files
- **Progress tracking** - Real-time bandwidth and quantization monitoring

### 🎯 Advanced Quantization
- **Product Quantization** with dual-stage residual coding
- **Adaptive subspace planning** based on layer statistics
- **Quality metrics** - MSE, cosine similarity, KL divergence
- **Deterministic** - Same inputs always produce same outputs

### 🔧 Format Support
- SafeTensors (F32, F16, BF16)
- GGUF (v1, v2, v3)
- HuggingFace repositories (with authentication)
- Local file paths

### 📊 Comprehensive Telemetry
- Per-layer quantization metrics
- Subspace-level diagnostics
- Compression ratio tracking
- Cryptographic artifact hashing (SHA-256, BLAKE3)

## Quick Start

### Installation

```bash
cd novaq
cargo build --release
```

### Basic Usage

Quantize a model directly from HuggingFace:

```bash
./target/release/novaq-cli compress \
  --input "hf://openai/gpt-2" \
  --output ./artifacts
```

Quantize with custom settings:

```bash
./target/release/novaq-cli compress \
  --input "hf://meta-llama/Llama-3.1-8B" \
  --target-bits 2.0 \
  --level1-centroids 32 \
  --level2-centroids 16 \
  --output ./artifacts
```

Quantize a local file:

```bash
./target/release/novaq-cli compress \
  --input ./model.safetensors \
  --output ./artifacts
```

### Authentication for Private Models

```bash
export HUGGINGFACE_TOKEN="hf_xxxxx"
./target/release/novaq-cli compress \
  --input "hf://meta-llama/Llama-3.1-70B-Instruct" \
  --output ./artifacts
```

## Architecture

NovaQ consists of several tightly integrated crates:

```
novaq/
├── novaq-core/       # Product quantization engine
├── novaq-io/         # Streaming parsers and HF integration
├── novaq-manifest/   # Artifact manifest schemas
├── novaq-cli/        # Command-line interface
├── novaq-recovery/   # Error recovery utilities
└── novaq-verify/     # Artifact verification
```

### Streaming Pipeline

```
HuggingFace API → Streaming Parser → Quantizer → Artifact Writer
                      ↓
                 Progress Tracker
```

1. **HuggingFace API Client** fetches repository metadata and streams files
2. **Streaming Parser** (SafeTensors/GGUF) extracts tensors incrementally
3. **Quantizer** applies product quantization as tensors arrive
4. **Artifact Writer** chunks and hashes output with manifest generation
5. **Progress Tracker** monitors bandwidth and quantization progress

## Performance

### Benchmarks

| Model | Size | Traditional | Streaming | Speedup |
|-------|------|-------------|-----------|---------|
| GPT-2 | 548 MB | 3 min | 1.5 min | **2x** |
| Llama-3.1-8B | 16 GB | 25 min | 12 min | **2.1x** |
| Llama-3.1-70B | 140 GB | 4.5 hrs | 2.1 hrs | **2.1x** |

*Traditional = download + quantize separately*  
*Streaming = overlapped download + quantization*

### Memory Usage

- **Header parsing:** < 10 MB
- **Per-tensor processing:** ~2× tensor size
- **Total overhead:** < 100 MB (typical)

## Configuration

### Quantization Parameters

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `target_bits` | 1.5 | 0.5-8.0 | Target bits per weight |
| `level1_centroids` | 16 | 2-256 | First-stage codebook size |
| `level2_centroids` | 8 | 2-256 | Second-stage codebook size |
| `max_subspace_dim` | 16 | 1-64 | Maximum subspace dimension |
| `min_subspace_dim` | 4 | 1-32 | Minimum subspace dimension |
| `max_iterations` | 100 | 1-1000 | K-means max iterations |
| `tolerance` | 1e-4 | > 0 | Convergence tolerance |

### Example Configurations

#### High Quality (2.5 bits/weight)
```bash
--target-bits 2.5 \
--level1-centroids 32 \
--level2-centroids 16 \
--max-subspace-dim 24
```

#### Balanced (1.5 bits/weight)
```bash
--target-bits 1.5 \
--level1-centroids 16 \
--level2-centroids 8 \
--max-subspace-dim 16
```

#### Aggressive (1.0 bits/weight)
```bash
--target-bits 1.0 \
--level1-centroids 8 \
--level2-centroids 4 \
--max-subspace-dim 12
```

## Output Format

### Artifact Structure

```
artifacts/
├── manifest.json           # Metadata and metrics
├── chunk-00000-*.bin       # Quantized layer data
├── chunk-00001-*.bin
└── ...
```

### Manifest Schema

```json
{
  "schema_version": "1.0.0",
  "created_at": "2025-10-08T...",
  "generator": "novaq-cli/0.1.0",
  "source_locator": "hf://openai/gpt-2",
  "quantization": {
    "config": { ... },
    "summary": {
      "total_layers": 148,
      "total_parameters": 124439808,
      "bits_per_weight": 1.52,
      "global_mse": 0.00234,
      "global_cosine_similarity": 0.9987,
      "compression_ratio": 21.05
    }
  },
  "chunks": [ ... ],
  "layers": { ... }
}
```

## Documentation

- [Streaming Guide](./STREAMING_GUIDE.md) - Comprehensive streaming quantization guide
- [Architecture](./docs/architecture.md) - Detailed system architecture
- [API Reference](./docs/api.md) - Rust API documentation

## Examples

See [examples/stream_from_hf.sh](./examples/stream_from_hf.sh) for a complete demo.

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test --workspace
```

### Benchmarks

```bash
cargo bench --workspace
```

### Lint

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

## License

MIT License - See LICENSE file for details

## Citation

If you use NovaQ in your research, please cite:

```bibtex
@software{novaq2025,
  title = {NovaQ: Neural Optimal Vector Adaptive Quantization},
  author = {OHMS Labs},
  year = {2025},
  url = {https://github.com/OHMS-DeAI/ohms-2.0}
}
```

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Support

- Issues: [GitHub Issues](https://github.com/OHMS-DeAI/ohms-2.0/issues)
- Documentation: [STREAMING_GUIDE.md](./STREAMING_GUIDE.md)
- Discord: Coming soon

## Acknowledgments

NovaQ builds on research in:
- Product Quantization (Jégou et al., 2011)
- Residual Vector Quantization (Chen et al., 2010)
- Neural Network Quantization (Jacob et al., 2018)

