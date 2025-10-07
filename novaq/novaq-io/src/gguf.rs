use anyhow::{anyhow, Context, Result};
use ndarray::Array2;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use tracing::{debug, instrument};

use novaq_core::{QuantizationConfig, QuantizedModel, Quantizer};

const GGUF_MAGIC: &[u8; 4] = b"GGUF";

#[derive(Debug)]
struct TensorDescriptor {
    name: String,
    dims: Vec<usize>,
    offset: u64,
    type_id: u32,
}

pub struct GgufLoader {
    quantizer: Quantizer,
}

impl GgufLoader {
    pub fn new(config: QuantizationConfig) -> Result<Self> {
        Ok(Self {
            quantizer: Quantizer::new(config)?,
        })
    }

    #[instrument(skip(self, reader, writer))]
    pub async fn load_from_reader<R>(
        &self,
        mut reader: R,
        writer: &mut crate::artifact::ArtifactWriter,
    ) -> Result<QuantizedModel>
    where
        R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin + Send,
    {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).await?;
        if &magic != GGUF_MAGIC {
            return Err(anyhow!("invalid GGUF magic"));
        }
        let version = reader.read_u32_le().await?;
        if !(1..=3).contains(&version) {
            return Err(anyhow!("unsupported GGUF version {version}"));
        }

        let tensor_count = reader.read_u64_le().await?;
        let kv_count = reader.read_u64_le().await?;

        for _ in 0..kv_count {
            skip_kv(&mut reader).await?;
        }

        let mut descriptors = Vec::with_capacity(tensor_count as usize);
        for _ in 0..tensor_count {
            let name = read_string(&mut reader).await?;
            let n_dims = reader.read_u32_le().await? as usize;
            let mut dims = Vec::with_capacity(n_dims);
            for _ in 0..n_dims {
                dims.push(reader.read_u64_le().await? as usize);
            }
            let type_id = reader.read_u32_le().await?;
            let offset = reader.read_u64_le().await?;
            descriptors.push(TensorDescriptor {
                name,
                dims,
                offset,
                type_id,
            });
        }

        let data_start = reader.seek(SeekFrom::Current(0)).await?;
        let mut layers = Vec::with_capacity(descriptors.len());

        for (index, desc) in descriptors.into_iter().enumerate() {
            if desc.dims.len() != 2 {
                debug!(tensor = %desc.name, dims = ?desc.dims, "skipping non-matrix tensor");
                continue;
            }
            if desc.type_id != 0 {
                return Err(anyhow!(
                    "unsupported GGUF tensor type {} for {}",
                    desc.type_id,
                    desc.name
                ));
            }

            let matrix = read_tensor_matrix(&mut reader, &desc, data_start).await?;
            let quantized = self.quantizer.quantize_layer(&desc.name, index, &matrix)?;
            debug!(tensor = %desc.name, subspaces = quantized.subspaces.len(), "tensor quantized");
            let serialized = serde_json::to_vec(&quantized).context("serialize quantized layer")?;
            writer.write_chunk(&serialized)?;
            layers.push(quantized);
        }

        Ok(QuantizedModel::from_layers(layers))
    }
}

async fn read_tensor_matrix<R>(
    reader: &mut R,
    desc: &TensorDescriptor,
    data_start: u64,
) -> Result<Array2<f32>>
where
    R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin,
{
    let rows = desc.dims[0];
    let cols = desc.dims[1];
    let _expected_bytes = rows
        .checked_mul(cols)
        .ok_or_else(|| anyhow!("tensor shape overflow"))?
        * std::mem::size_of::<f32>();
    reader
        .seek(SeekFrom::Start(data_start + desc.offset))
        .await?;
    let mut matrix = Array2::<f32>::zeros((rows, cols));
    let mut row_bytes = vec![0u8; cols * std::mem::size_of::<f32>()];
    for row in 0..rows {
        reader.read_exact(&mut row_bytes).await?;
        for (col, chunk) in row_bytes.chunks_exact(4).enumerate() {
            let value = f32::from_le_bytes(chunk.try_into().unwrap());
            matrix[(row, col)] = value;
        }
    }
    debug!(tensor_offset = desc.offset, "loaded gguf tensor data");
    Ok(matrix)
}

async fn skip_kv<R>(reader: &mut R) -> Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let key = read_string(reader).await?;
    let value_type = reader.read_u32_le().await?;
    skip_value(reader, value_type)
        .await
        .with_context(|| format!("failed to skip kv value for key {key}"))
}

async fn skip_value<R>(reader: &mut R, value_type: u32) -> Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut stack = vec![value_type];
    while let Some(vt) = stack.pop() {
        match vt {
            0 => {
                reader.read_u8().await?;
            }
            1 => {
                reader.read_i8().await?;
            }
            2 => {
                reader.read_u16_le().await?;
            }
            3 => {
                reader.read_i16_le().await?;
            }
            4 => {
                reader.read_u32_le().await?;
            }
            5 => {
                reader.read_i32_le().await?;
            }
            6 => {
                reader.read_f32_le().await?;
            }
            7 => {
                reader.read_u8().await?;
            }
            8 => {
                let _ = read_string(reader).await?;
            }
            9 => {
                let elem_type = reader.read_u32_le().await?;
                let len = reader.read_u64_le().await?;
                for _ in 0..len {
                    stack.push(elem_type);
                }
            }
            10 => {
                reader.read_u64_le().await?;
            }
            11 => {
                reader.read_i64_le().await?;
            }
            12 => {
                reader.read_f64_le().await?;
            }
            _ => return Err(anyhow!("unsupported gguf value type {vt}")),
        }
    }
    Ok(())
}

async fn read_string<R>(reader: &mut R) -> Result<String>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let len = reader.read_u32_le().await? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}
