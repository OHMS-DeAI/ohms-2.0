use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use half::{bf16, f16};
use ndarray::Array2;
use serde_json::Value;
use std::pin::Pin;
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

pub struct StreamingSafeTensorsParser {
    quantizer: Quantizer,
    progress: Option<ProgressTracker>,
}

impl StreamingSafeTensorsParser {
    pub fn new(config: QuantizationConfig, progress: Option<ProgressTracker>) -> Result<Self> {
        Ok(Self {
            quantizer: Quantizer::new(config)?,
            progress,
        })
    }

    #[instrument(skip(self, stream, writer))]
    pub async fn parse_and_quantize<S>(
        &self,
        mut stream: Pin<Box<S>>,
        writer: &mut ArtifactWriter,
    ) -> Result<QuantizedModel>
    where
        S: Stream<Item = Result<Bytes>> + Send,
    {
        let mut buffer = ByteBuffer::new();
        let mut state = ParseState::ReadingHeaderSize;
        let mut header_size = 0u64;
        let mut tensors_metadata: Vec<TensorMetadata> = Vec::new();
        let mut data_start = 0u64;
        let mut current_offset = 0u64;
        let mut layers = Vec::new();
        let mut total_bytes_received = 0u64;
        let mut chunk_count = 0u64;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("error reading chunk from stream")?;
            chunk_count += 1;
            total_bytes_received += chunk.len() as u64;
            
            if chunk_count % 100 == 0 {
                debug!(
                    chunks_received = chunk_count,
                    bytes_received = total_bytes_received,
                    buffer_size = buffer.len(),
                    "stream progress"
                );
            }
            
            buffer.extend(chunk);

            match state {
                ParseState::ReadingHeaderSize => {
                    if buffer.len() >= 8 {
                        header_size = buffer.read_u64_le();
                        current_offset = 8;
                        state = ParseState::ReadingHeader { header_size };
                        debug!(header_size, "read header size");
                    }
                }
                ParseState::ReadingHeader { header_size } => {
                    if buffer.len() >= header_size as usize {
                        let header_bytes = buffer.read_bytes(header_size as usize);
                        let header: Value = serde_json::from_slice(&header_bytes)
                            .context("failed to parse safetensors header")?;

                        let header_obj = header
                            .as_object()
                            .ok_or_else(|| anyhow!("header is not an object"))?;

                        for (name, value) in header_obj.iter() {
                            if name == "__metadata__" {
                                continue;
                            }
                            let metadata = parse_tensor_metadata(name, value)?;
                            tensors_metadata.push(metadata);
                        }

                        current_offset += header_size;
                        data_start = current_offset;
                        let padding = data_start % 8;
                        if padding != 0 {
                            let pad_size = 8 - padding;
                            data_start += pad_size;
                            current_offset += pad_size;
                        }

                        state = ParseState::ReadingTensors {
                            tensors: tensors_metadata.clone(),
                            current_tensor_idx: 0,
                            data_start,
                        };

                        if let Some(ref progress) = self.progress {
                            let bar = progress.add_file_processing_bar(
                                "safetensors",
                                tensors_metadata.len() as u64,
                            );
                            bar.set_position(0);
                        }

                        debug!(
                            tensor_count = tensors_metadata.len(),
                            data_start,
                            current_offset,
                            "parsed header, transitioning to ReadingTensors"
                        );
                    }
                }
                ParseState::ReadingTensors {
                    ref tensors,
                    current_tensor_idx,
                    data_start,
                } => {
                    if current_tensor_idx >= tensors.len() {
                        debug!("all tensors processed, exiting");
                        break;
                    }

                    debug!(
                        current_tensor_idx,
                        total_tensors = tensors.len(),
                        buffer_len = buffer.len(),
                        current_offset,
                        "processing tensor"
                    );

                    let tensor_meta = &tensors[current_tensor_idx];
                    let start_offset = data_start + tensor_meta.data_offsets[0];
                    let end_offset = data_start + tensor_meta.data_offsets[1];
                    let tensor_size = (end_offset - start_offset) as usize;

                    if current_offset < start_offset {
                        let skip_bytes = (start_offset - current_offset) as usize;
                        if buffer.len() >= skip_bytes {
                            buffer.consume(skip_bytes);
                            current_offset = start_offset;
                        } else {
                            continue;
                        }
                    }

                    if buffer.len() >= tensor_size {
                        let tensor_bytes = buffer.read_bytes(tensor_size);
                        current_offset += tensor_size as u64;

                        if tensor_meta.shape.len() == 2 {
                            let matrix =
                                decode_tensor(&tensor_bytes, &tensor_meta.shape, &tensor_meta.dtype)?;

                            let layer_idx = layers.len();
                            let quantized = self.quantizer.quantize_layer(
                                &tensor_meta.name,
                                layer_idx,
                                &matrix,
                            )?;

                            let serialized = serde_json::to_vec(&quantized)
                                .context("failed to serialize quantized layer")?;
                            writer.write_chunk(&serialized)?;
                            layers.push(quantized);

                            debug!(
                                tensor = tensor_meta.name,
                                shape = ?tensor_meta.shape,
                                "quantized tensor"
                            );

                            if let Some(ref progress) = self.progress {
                                let bar = progress
                                    .add_file_processing_bar("safetensors", tensors.len() as u64);
                                bar.set_position((current_tensor_idx + 1) as u64);
                            }
                        } else {
                            debug!(
                                tensor = tensor_meta.name,
                                shape = ?tensor_meta.shape,
                                "skipping non-matrix tensor"
                            );
                        }

                        state = ParseState::ReadingTensors {
                            tensors: tensors.clone(),
                            current_tensor_idx: current_tensor_idx + 1,
                            data_start,
                        };
                    } else {
                        break;
                    }
                }
            }
        }

        debug!(
            total_chunks = chunk_count,
            total_bytes = total_bytes_received,
            total_layers = layers.len(),
            "stream ended"
        );

        Ok(QuantizedModel::from_layers(layers))
    }
}

#[derive(Debug, Clone)]
enum ParseState {
    ReadingHeaderSize,
    ReadingHeader { header_size: u64 },
    ReadingTensors {
        tensors: Vec<TensorMetadata>,
        current_tensor_idx: usize,
        data_start: u64,
    },
}

struct ByteBuffer {
    data: Vec<u8>,
}

impl ByteBuffer {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn extend(&mut self, bytes: Bytes) {
        self.data.extend_from_slice(&bytes);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn read_u64_le(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[..8]);
        self.data.drain(..8);
        u64::from_le_bytes(buf)
    }

    fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes = self.data.drain(..n).collect();
        bytes
    }

    fn consume(&mut self, n: usize) {
        self.data.drain(..n);
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

