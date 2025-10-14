use anyhow::{anyhow, Context, Result};
use half::{bf16, f16};
use ndarray::Array2;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::{debug, instrument};

use novaq_core::{QuantizationConfig, QuantizedModel, Quantizer};

use crate::artifact::ArtifactWriter;
use crate::progress::ProgressTracker;

#[derive(Debug, Clone)]
struct TensorMetadata {
    name: String,
    shape: Vec<usize>,
    dtype: String,
    data_offsets: [u64; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TensorDType {
    F32,
    F16,
    BF16,
}

impl TensorDType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "F32" => Ok(Self::F32),
            "F16" => Ok(Self::F16),
            "BF16" => Ok(Self::BF16),
            other => Err(anyhow!("unsupported dtype: {}", other)),
        }
    }

    fn bytes_per_element(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F16 | Self::BF16 => 2,
        }
    }
}

pub struct StreamingSafeTensorsParserV2 {
    quantizer: Quantizer,
    progress: Option<ProgressTracker>,
}

impl StreamingSafeTensorsParserV2 {
    pub fn new(config: QuantizationConfig, progress: Option<ProgressTracker>) -> Result<Self> {
        Ok(Self {
            quantizer: Quantizer::new(config)?,
            progress,
        })
    }

    #[instrument(skip(self, reader, writer))]
    pub async fn parse_and_quantize<R>(
        &self,
        reader: &mut R,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel>
    where
        R: AsyncRead + Unpin,
    {
        let header_size = reader.read_u64_le().await?;
        debug!(header_size, "reading header");

        let mut header_bytes = vec![0u8; header_size as usize];
        reader.read_exact(&mut header_bytes).await?;

        let metadata: Value =
            serde_json::from_slice(&header_bytes).context("invalid safetensors header")?;
        let metadata = metadata
            .as_object()
            .ok_or_else(|| anyhow!("safetensors header is not a JSON object"))?;

        let mut data_start = 8 + header_size;
        let padding = data_start % 8;
        if padding != 0 {
            data_start += 8 - padding;
        }

        let mut tensors_metadata = Vec::new();
        for (name, value) in metadata.iter() {
            if name == "__metadata__" {
                continue;
            }
            let tensor_meta = parse_tensor_metadata(name, value)?;
            tensors_metadata.push(tensor_meta);
        }

        debug!(
            tensor_count = tensors_metadata.len(),
            data_start,
            "parsed header"
        );

        let mut layers = Vec::new();
        let mut current_offset = 8 + header_size;

        if padding != 0 {
            let pad_size = 8 - padding;
            let mut pad_buf = vec![0u8; pad_size as usize];
            reader.read_exact(&mut pad_buf).await?;
            current_offset += pad_size;
        }

        for (idx, tensor_meta) in tensors_metadata.iter().enumerate() {
            if tensor_meta.shape.len() != 2 {
                debug!(
                    tensor = tensor_meta.name,
                    shape = ?tensor_meta.shape,
                    "skipping non-matrix tensor"
                );
                
                let start_offset = data_start + tensor_meta.data_offsets[0];
                let end_offset = data_start + tensor_meta.data_offsets[1];
                let tensor_size = (end_offset - start_offset) as usize;
                
                let skip_size = (start_offset - current_offset) as usize + tensor_size;
                let mut skip_buf = vec![0u8; skip_size.min(1024 * 1024)];
                let mut remaining = skip_size;
                while remaining > 0 {
                    let to_skip = remaining.min(skip_buf.len());
                    reader.read_exact(&mut skip_buf[..to_skip]).await?;
                    remaining -= to_skip;
                }
                current_offset = end_offset;
                continue;
            }

            let start_offset = data_start + tensor_meta.data_offsets[0];
            let end_offset = data_start + tensor_meta.data_offsets[1];
            let tensor_size = (end_offset - start_offset) as usize;

            if current_offset < start_offset {
                let skip_bytes = (start_offset - current_offset) as usize;
                let mut skip_buf = vec![0u8; skip_bytes.min(1024 * 1024)];
                let mut remaining = skip_bytes;
                while remaining > 0 {
                    let to_skip = remaining.min(skip_buf.len());
                    reader.read_exact(&mut skip_buf[..to_skip]).await?;
                    remaining -= to_skip;
                }
                current_offset = start_offset;
            }

            let mut tensor_bytes = vec![0u8; tensor_size];
            reader.read_exact(&mut tensor_bytes).await?;
            current_offset += tensor_size as u64;

            let matrix = decode_tensor(&tensor_bytes, &tensor_meta.shape, &tensor_meta.dtype)?;

            let layer_idx = layers.len();
            let quantized = self
                .quantizer
                .quantize_layer(&tensor_meta.name, layer_idx, &matrix)?;

            let serialized =
                serde_json::to_vec(&quantized).context("failed to serialize quantized layer")?;
            writer.write_chunk(&serialized)?;
            layers.push(quantized);

            if idx % 10 == 0 || idx == tensors_metadata.len() - 1 {
                debug!(
                    processed = idx + 1,
                    total = tensors_metadata.len(),
                    tensor = tensor_meta.name,
                    "quantized tensor"
                );
            }
        }

        debug!(total_layers = layers.len(), "completed quantization");
        Ok(QuantizedModel::from_layers(layers))
    }
}

fn parse_tensor_metadata(name: &str, value: &Value) -> Result<TensorMetadata> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow!("tensor entry is not an object"))?;

    let shape = obj
        .get("shape")
        .ok_or_else(|| anyhow!("missing shape"))?
        .as_array()
        .ok_or_else(|| anyhow!("shape is not an array"))?
        .iter()
        .map(|v| {
            v.as_u64()
                .ok_or_else(|| anyhow!("shape dimension is not a number"))
                .map(|n| n as usize)
        })
        .collect::<Result<Vec<_>>>()?;

    let offsets = obj
        .get("data_offsets")
        .ok_or_else(|| anyhow!("missing data_offsets"))?
        .as_array()
        .ok_or_else(|| anyhow!("data_offsets is not an array"))?;

    if offsets.len() != 2 {
        return Err(anyhow!("data_offsets must have exactly 2 elements"));
    }

    let start = offsets[0]
        .as_u64()
        .ok_or_else(|| anyhow!("start offset is not a number"))?;
    let end = offsets[1]
        .as_u64()
        .ok_or_else(|| anyhow!("end offset is not a number"))?;

    let dtype = obj
        .get("dtype")
        .ok_or_else(|| anyhow!("missing dtype"))?
        .as_str()
        .ok_or_else(|| anyhow!("dtype is not a string"))?
        .to_string();

    Ok(TensorMetadata {
        name: name.to_string(),
        shape,
        dtype,
        data_offsets: [start, end],
    })
}

fn decode_tensor(bytes: &[u8], shape: &[usize], dtype_str: &str) -> Result<Array2<f32>> {
    if shape.len() != 2 {
        return Err(anyhow!("expected 2D tensor, got shape {:?}", shape));
    }

    let rows = shape[0];
    let cols = shape[1];
    let dtype = TensorDType::from_str(dtype_str)?;
    let bytes_per_elem = dtype.bytes_per_element();
    let expected_size = rows * cols * bytes_per_elem;

    if bytes.len() != expected_size {
        return Err(anyhow!(
            "tensor size mismatch: expected {} bytes, got {}",
            expected_size,
            bytes.len()
        ));
    }

    let mut matrix = Array2::<f32>::zeros((rows, cols));

    match dtype {
        TensorDType::F32 => {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row * cols + col) * 4;
                    let value = f32::from_le_bytes([
                        bytes[idx],
                        bytes[idx + 1],
                        bytes[idx + 2],
                        bytes[idx + 3],
                    ]);
                    matrix[(row, col)] = value;
                }
            }
        }
        TensorDType::F16 => {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row * cols + col) * 2;
                    let bits = u16::from_le_bytes([bytes[idx], bytes[idx + 1]]);
                    matrix[(row, col)] = f16::from_bits(bits).to_f32();
                }
            }
        }
        TensorDType::BF16 => {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row * cols + col) * 2;
                    let bits = u16::from_le_bytes([bytes[idx], bytes[idx + 1]]);
                    matrix[(row, col)] = bf16::from_bits(bits).to_f32();
                }
            }
        }
    }

    Ok(matrix)
}

