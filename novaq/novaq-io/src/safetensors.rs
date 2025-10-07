use anyhow::{anyhow, Context, Result};
use ndarray::Array2;
use half::{bf16, f16};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::{debug, instrument};

use novaq_core::{QuantizationConfig, QuantizedModel, Quantizer};

use crate::artifact::ArtifactWriter;

#[derive(Debug)]
struct TensorInfo {
    shape: Vec<usize>,
    data_offsets: [u64; 2],
    dtype: TensorDType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TensorDType {
    F32,
    F16,
    BF16,
}

impl TensorDType {
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "F32" => Ok(Self::F32),
            "F16" => Ok(Self::F16),
            "BF16" => Ok(Self::BF16),
            other => Err(anyhow!("unsupported safetensors dtype: {other}")),
        }
    }

    fn bytes_per_element(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F16 | Self::BF16 => 2,
        }
    }
}

pub struct SafeTensorsLoader {
    quantizer: Quantizer,
}

impl SafeTensorsLoader {
    pub fn new(config: QuantizationConfig) -> Result<Self> {
        Ok(Self {
            quantizer: Quantizer::new(config)?,
        })
    }

    #[instrument(skip(self, reader, writer))]
    pub async fn load_from_reader<R>(
        &self,
        mut reader: R,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel>
    where
        R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin + Send,
    {
        let header_size = reader.read_u64_le().await?;
        let mut header_bytes = vec![0u8; header_size as usize];
        reader.read_exact(&mut header_bytes).await?;
        let metadata: Value = serde_json::from_slice(&header_bytes)
            .context("invalid safetensors header")?;
        let metadata = metadata
            .as_object()
            .ok_or_else(|| anyhow!("safetensors header is not a JSON object"))?;
        let mut data_start = 8 + header_size;
        let padding = data_start % 8;
        if padding != 0 {
            data_start += 8 - padding;
        }

        let mut layers = Vec::new();
        for (tensor_name, value) in metadata.iter() {
            if tensor_name == "__metadata__" {
                continue;
            }
            let info = TensorInfo::from_value(value)
                .with_context(|| format!("invalid tensor header for {tensor_name}"))?;
            debug!(tensor = tensor_name, "loading tensor from safetensors");
            if info.shape.len() != 2 {
                debug!(tensor = tensor_name, shape = ?info.shape, "skipping non-matrix tensor");
                continue;
            }

            let matrix = read_tensor_matrix(&mut reader, &info, data_start).await?;

            let quantized = self.quantizer.quantize_layer(
                tensor_name,
                writer.manifest().chunks.len(),
                &matrix,
            )?;
            debug!(
                tensor = tensor_name,
                subspaces = quantized.subspaces.len(),
                "tensor quantized"
            );

            let serialized = serde_json::to_vec(&quantized).context("serialize quantized layer")?;
            writer.write_chunk(&serialized)?;
            layers.push(quantized);
        }

        Ok(QuantizedModel::from_layers(layers))
    }
}

impl TensorInfo {
    fn from_value(value: &Value) -> Result<Self> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("tensor entry is not an object"))?;

        let shape = obj
            .get("shape")
            .ok_or_else(|| anyhow!("tensor entry missing shape"))?
            .as_array()
            .ok_or_else(|| anyhow!("tensor shape must be an array"))?
            .iter()
            .map(|v| {
                v.as_u64()
                    .ok_or_else(|| anyhow!("tensor dimension must be an integer"))
                    .map(|n| n as usize)
            })
            .collect::<Result<Vec<_>>>()?;

        let offsets = obj
            .get("data_offsets")
            .ok_or_else(|| anyhow!("tensor entry missing data_offsets"))?
            .as_array()
            .ok_or_else(|| anyhow!("tensor data_offsets must be an array"))?;
        if offsets.len() != 2 {
            return Err(anyhow!("tensor data_offsets must contain exactly two entries"));
        }
        let start = offsets[0]
            .as_u64()
            .ok_or_else(|| anyhow!("data_offsets entries must be integers"))?;
        let end = offsets[1]
            .as_u64()
            .ok_or_else(|| anyhow!("data_offsets entries must be integers"))?;

        let dtype = obj
            .get("dtype")
            .ok_or_else(|| anyhow!("tensor entry missing dtype"))?
            .as_str()
            .ok_or_else(|| anyhow!("tensor dtype must be a string"))?;

        Ok(Self {
            shape,
            data_offsets: [start, end],
            dtype: TensorDType::from_str(dtype)?,
        })
    }
}

async fn read_tensor_matrix<R>(
    reader: &mut R,
    info: &TensorInfo,
    data_start: u64,
) -> Result<Array2<f32>>
where
    R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin,
{
    let start = data_start + info.data_offsets[0];
    let end = data_start + info.data_offsets[1];
    let len = (end - start) as usize;
    let rows = info.shape[0];
    let cols = info.shape[1];
    let bytes_per_element = info.dtype.bytes_per_element();
    let expected_len = rows
        .checked_mul(cols)
        .ok_or_else(|| anyhow!("tensor shape overflow"))?
        * bytes_per_element;
    if len != expected_len {
        return Err(anyhow!(
            "tensor byte length {} does not match shape {:?}",
            len,
            info.shape
        ));
    }

    reader.seek(std::io::SeekFrom::Start(start)).await?;
    let mut matrix = Array2::<f32>::zeros((rows, cols));
    let mut row_bytes = vec![0u8; cols * bytes_per_element];
    for row in 0..rows {
        reader.read_exact(&mut row_bytes).await?;
        match info.dtype {
            TensorDType::F32 => {
                for (col, chunk) in row_bytes.chunks_exact(4).enumerate() {
                    let value = f32::from_le_bytes(chunk.try_into().unwrap());
                    matrix[(row, col)] = value;
                }
            }
            TensorDType::F16 => {
                for (col, chunk) in row_bytes.chunks_exact(2).enumerate() {
                    let bits = u16::from_le_bytes(chunk.try_into().unwrap());
                    let value = f16::from_bits(bits).to_f32();
                    matrix[(row, col)] = value;
                }
            }
            TensorDType::BF16 => {
                for (col, chunk) in row_bytes.chunks_exact(2).enumerate() {
                    let bits = u16::from_le_bytes(chunk.try_into().unwrap());
                    let value = bf16::from_bits(bits).to_f32();
                    matrix[(row, col)] = value;
                }
            }
        }
    }
    Ok(matrix)
}
