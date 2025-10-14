use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use ndarray::Array2;
use std::pin::Pin;
use tracing::{debug, instrument};

use novaq_core::{QuantizationConfig, QuantizedModel, Quantizer};

use crate::artifact::ArtifactWriter;
use crate::progress::ProgressTracker;

const GGUF_MAGIC: &[u8; 4] = b"GGUF";

#[derive(Debug, Clone)]
struct TensorDescriptor {
    name: String,
    dims: Vec<usize>,
    type_id: u32,
    offset: u64,
}

pub struct StreamingGgufParser {
    quantizer: Quantizer,
    progress: Option<ProgressTracker>,
}

impl StreamingGgufParser {
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
        let mut state = ParseState::ReadingMagic;
        let mut version = 0u32;
        let mut tensor_count = 0u64;
        let mut kv_count = 0u64;
        let mut descriptors: Vec<TensorDescriptor> = Vec::new();
        let mut data_start = 0u64;
        let mut current_offset = 0u64;
        let mut layers = Vec::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            buffer.extend(chunk);

            match state {
                ParseState::ReadingMagic => {
                    if buffer.len() >= 4 {
                        let magic = buffer.read_bytes(4);
                        if &magic[..] != GGUF_MAGIC {
                            return Err(anyhow!("invalid GGUF magic bytes"));
                        }
                        current_offset = 4;
                        state = ParseState::ReadingVersion;
                        debug!("validated GGUF magic");
                    }
                }
                ParseState::ReadingVersion => {
                    if buffer.len() >= 4 {
                        version = buffer.read_u32_le();
                        if !(1..=3).contains(&version) {
                            return Err(anyhow!("unsupported GGUF version: {}", version));
                        }
                        current_offset += 4;
                        state = ParseState::ReadingCounts;
                        debug!(version, "read GGUF version");
                    }
                }
                ParseState::ReadingCounts => {
                    if buffer.len() >= 16 {
                        tensor_count = buffer.read_u64_le();
                        kv_count = buffer.read_u64_le();
                        current_offset += 16;
                        state = ParseState::SkippingKV {
                            remaining: kv_count,
                        };
                        debug!(tensor_count, kv_count, "read counts");
                    }
                }
                ParseState::SkippingKV { remaining } => {
                    if remaining == 0 {
                        state = ParseState::ReadingTensorHeaders {
                            remaining: tensor_count,
                        };
                    } else {
                        if let Some(bytes_consumed) = try_skip_kv(&mut buffer)? {
                            current_offset += bytes_consumed as u64;
                            state = ParseState::SkippingKV {
                                remaining: remaining - 1,
                            };
                        } else {
                            break;
                        }
                    }
                }
                ParseState::ReadingTensorHeaders { remaining } => {
                    if remaining == 0 {
                        data_start = current_offset;
                        state = ParseState::ReadingTensors {
                            descriptors: descriptors.clone(),
                            current_idx: 0,
                            data_start,
                        };

                        if let Some(ref progress) = self.progress {
                            let bar =
                                progress.add_file_processing_bar("gguf", descriptors.len() as u64);
                            bar.set_position(0);
                        }
                    } else {
                        if let Some((desc, bytes_consumed)) = try_read_tensor_header(&mut buffer)? {
                            current_offset += bytes_consumed as u64;
                            descriptors.push(desc);
                            state = ParseState::ReadingTensorHeaders {
                                remaining: remaining - 1,
                            };
                        } else {
                            break;
                        }
                    }
                }
                ParseState::ReadingTensors {
                    ref descriptors,
                    current_idx,
                    data_start,
                } => {
                    if current_idx >= descriptors.len() {
                        break;
                    }

                    let desc = &descriptors[current_idx];
                    if desc.dims.len() != 2 {
                        debug!(
                            tensor = desc.name,
                            dims = ?desc.dims,
                            "skipping non-matrix tensor"
                        );
                        state = ParseState::ReadingTensors {
                            descriptors: descriptors.clone(),
                            current_idx: current_idx + 1,
                            data_start,
                        };
                        continue;
                    }

                    if desc.type_id != 0 {
                        debug!(
                            tensor = desc.name,
                            type_id = desc.type_id,
                            "skipping non-F32 tensor"
                        );
                        state = ParseState::ReadingTensors {
                            descriptors: descriptors.clone(),
                            current_idx: current_idx + 1,
                            data_start,
                        };
                        continue;
                    }

                    let tensor_offset = data_start + desc.offset;
                    let tensor_size = desc.dims[0] * desc.dims[1] * 4;

                    if current_offset < tensor_offset {
                        let skip_bytes = (tensor_offset - current_offset) as usize;
                        if buffer.len() >= skip_bytes {
                            buffer.consume(skip_bytes);
                            current_offset = tensor_offset;
                        } else {
                            break;
                        }
                    }

                    if buffer.len() >= tensor_size {
                        let tensor_bytes = buffer.read_bytes(tensor_size);
                        current_offset += tensor_size as u64;

                        let matrix = decode_f32_tensor(&tensor_bytes, &desc.dims)?;
                        let layer_idx = layers.len();
                        let quantized =
                            self.quantizer
                                .quantize_layer(&desc.name, layer_idx, &matrix)?;

                        let serialized = serde_json::to_vec(&quantized)
                            .context("failed to serialize quantized layer")?;
                        writer.write_chunk(&serialized)?;
                        layers.push(quantized);

                        debug!(
                            tensor = desc.name,
                            shape = ?desc.dims,
                            "quantized tensor"
                        );

                        if let Some(ref progress) = self.progress {
                            let bar = progress
                                .add_file_processing_bar("gguf", descriptors.len() as u64);
                            bar.set_position((current_idx + 1) as u64);
                        }

                        state = ParseState::ReadingTensors {
                            descriptors: descriptors.clone(),
                            current_idx: current_idx + 1,
                            data_start,
                        };
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(QuantizedModel::from_layers(layers))
    }
}

#[derive(Debug, Clone)]
enum ParseState {
    ReadingMagic,
    ReadingVersion,
    ReadingCounts,
    SkippingKV { remaining: u64 },
    ReadingTensorHeaders { remaining: u64 },
    ReadingTensors {
        descriptors: Vec<TensorDescriptor>,
        current_idx: usize,
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

    fn read_u32_le(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.data[..4]);
        self.data.drain(..4);
        u32::from_le_bytes(buf)
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

    fn peek_u32_le(&self) -> Option<u32> {
        if self.data.len() >= 4 {
            Some(u32::from_le_bytes([
                self.data[0],
                self.data[1],
                self.data[2],
                self.data[3],
            ]))
        } else {
            None
        }
    }
}

fn try_skip_kv(buffer: &mut ByteBuffer) -> Result<Option<usize>> {
    let initial_len = buffer.len();

    if buffer.len() < 4 {
        return Ok(None);
    }

    let key_len = buffer.peek_u32_le().unwrap() as usize;
    if buffer.len() < 4 + key_len {
        return Ok(None);
    }

    buffer.consume(4);
    buffer.consume(key_len);

    if buffer.len() < 4 {
        return Ok(None);
    }

    let value_type = buffer.read_u32_le();
    let value_size = estimate_value_size(buffer, value_type)?;

    if let Some(size) = value_size {
        if buffer.len() >= size {
            buffer.consume(size);
            let consumed = initial_len - buffer.len();
            Ok(Some(consumed))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn estimate_value_size(buffer: &ByteBuffer, value_type: u32) -> Result<Option<usize>> {
    match value_type {
        0 | 1 | 7 => Ok(Some(1)),
        2 | 3 => Ok(Some(2)),
        4 | 5 | 6 => Ok(Some(4)),
        10 | 11 | 12 => Ok(Some(8)),
        8 => {
            if buffer.len() < 4 {
                return Ok(None);
            }
            let str_len = u32::from_le_bytes([
                buffer.data[0],
                buffer.data[1],
                buffer.data[2],
                buffer.data[3],
            ]) as usize;
            Ok(Some(4 + str_len))
        }
        9 => {
            if buffer.len() < 12 {
                return Ok(None);
            }
            Ok(None)
        }
        _ => Err(anyhow!("unsupported GGUF value type: {}", value_type)),
    }
}

fn try_read_tensor_header(buffer: &mut ByteBuffer) -> Result<Option<(TensorDescriptor, usize)>> {
    let initial_len = buffer.len();

    if buffer.len() < 4 {
        return Ok(None);
    }

    let name_len = buffer.peek_u32_le().unwrap() as usize;
    if buffer.len() < 4 + name_len {
        return Ok(None);
    }

    buffer.consume(4);
    let name_bytes = buffer.read_bytes(name_len);
    let name = String::from_utf8(name_bytes).context("invalid tensor name")?;

    if buffer.len() < 4 {
        return Ok(None);
    }

    let n_dims = buffer.read_u32_le() as usize;
    if buffer.len() < n_dims * 8 + 8 + 8 {
        return Ok(None);
    }

    let mut dims = Vec::with_capacity(n_dims);
    for _ in 0..n_dims {
        dims.push(buffer.read_u64_le() as usize);
    }

    let type_id = buffer.read_u32_le();
    let offset = buffer.read_u64_le();

    let consumed = initial_len - buffer.len();
    Ok(Some((
        TensorDescriptor {
            name,
            dims,
            type_id,
            offset,
        },
        consumed,
    )))
}

fn decode_f32_tensor(bytes: &[u8], dims: &[usize]) -> Result<Array2<f32>> {
    if dims.len() != 2 {
        return Err(anyhow!("expected 2D tensor, got {:?} dimensions", dims.len()));
    }

    let rows = dims[0];
    let cols = dims[1];
    let expected_size = rows * cols * 4;

    if bytes.len() != expected_size {
        return Err(anyhow!(
            "tensor size mismatch: expected {} bytes, got {}",
            expected_size,
            bytes.len()
        ));
    }

    let mut matrix = Array2::<f32>::zeros((rows, cols));
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

    Ok(matrix)
}

