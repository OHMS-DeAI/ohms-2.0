#!/bin/bash
set -e

echo "==================================="
echo "NovaQ Streaming Quantization Demo"
echo "==================================="
echo ""

# Build the CLI
echo "Building NovaQ CLI..."
cargo build --release --package novaq-cli
echo "✓ Build complete"
echo ""

# Example 1: Small model (GPT-2)
echo "Example 1: Quantizing GPT-2 (small model)"
echo "Command: novaq-cli compress --input hf://openai/gpt-2 --output ./artifacts/gpt2"
echo ""
./target/release/novaq-cli compress \
  --input "hf://openai/gpt-2" \
  --output ./artifacts/gpt2 \
  --target-bits 1.5 \
  --level1-centroids 16 \
  --level2-centroids 8

echo ""
echo "✓ GPT-2 quantization complete!"
echo "Artifacts saved to: ./artifacts/gpt2"
echo ""

# Example 2: Larger model with custom settings
echo "Example 2: Quantizing TinyLlama with custom settings"
echo "Command: novaq-cli compress --input hf://TinyLlama/TinyLlama-1.1B-Chat-v1.0 --target-bits 2.0"
echo ""
./target/release/novaq-cli compress \
  --input "hf://TinyLlama/TinyLlama-1.1B-Chat-v1.0" \
  --output ./artifacts/tinyllama \
  --target-bits 2.0 \
  --level1-centroids 32 \
  --level2-centroids 16

echo ""
echo "✓ TinyLlama quantization complete!"
echo "Artifacts saved to: ./artifacts/tinyllama"
echo ""

# Show results
echo "==================================="
echo "Results Summary"
echo "==================================="
echo ""

if [ -f ./artifacts/gpt2/manifest.json ]; then
    echo "GPT-2 Manifest:"
    cat ./artifacts/gpt2/manifest.json | jq '.quantization.summary'
    echo ""
fi

if [ -f ./artifacts/tinyllama/manifest.json ]; then
    echo "TinyLlama Manifest:"
    cat ./artifacts/tinyllama/manifest.json | jq '.quantization.summary'
    echo ""
fi

echo "✓ Demo complete!"
echo ""
echo "To quantize your own models, use:"
echo "  ./target/release/novaq-cli compress --input hf://OWNER/MODEL --output ./artifacts/output"
echo ""

