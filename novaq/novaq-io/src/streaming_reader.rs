use anyhow::{Context, Result};
use bytes::Bytes;
use futures::stream::Stream;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use tokio_util::io::StreamReader;
use tracing::debug;

pub struct BufferedStreamReader<S> {
    inner: Pin<Box<StreamReader<S, Bytes>>>,
    buffer: VecDeque<u8>,
    total_read: u64,
    chunk_count: u64,
}

impl<S> BufferedStreamReader<S>
where
    S: Stream<Item = Result<Bytes, std::io::Error>> + Unpin,
{
    pub fn new(stream: S) -> Self {
        let stream_reader = StreamReader::new(stream);
        Self {
            inner: Box::pin(stream_reader),
            buffer: VecDeque::with_capacity(1024 * 1024),
            total_read: 0,
            chunk_count: 0,
        }
    }

    pub fn total_read(&self) -> u64 {
        self.total_read
    }

    pub fn chunk_count(&self) -> u64 {
        self.chunk_count
    }
}

impl<S> AsyncRead for BufferedStreamReader<S>
where
    S: Stream<Item = Result<Bytes, std::io::Error>> + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let available = buf.remaining();
        
        if self.buffer.len() < available && self.buffer.len() < 65536 {
            let mut temp_buf = vec![0u8; 65536];
            let mut read_buf = ReadBuf::new(&mut temp_buf);
            
            match self.inner.as_mut().poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(())) => {
                    let filled = read_buf.filled();
                    if !filled.is_empty() {
                        self.buffer.extend(filled.iter().copied());
                        self.total_read += filled.len() as u64;
                        self.chunk_count += 1;
                        
                        if self.chunk_count % 1000 == 0 {
                            debug!(
                                chunks = self.chunk_count,
                                bytes = self.total_read,
                                buffer_size = self.buffer.len(),
                                "streaming progress"
                            );
                        }
                    }
                }
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => {
                    if self.buffer.is_empty() {
                        return Poll::Pending;
                    }
                }
            }
        }

        let to_read = available.min(self.buffer.len());
        if to_read > 0 {
            for _ in 0..to_read {
                if let Some(byte) = self.buffer.pop_front() {
                    buf.put_slice(&[byte]);
                }
            }
            Poll::Ready(Ok(()))
        } else if self.buffer.is_empty() {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

pub async fn read_exact_async<R: AsyncRead + Unpin>(
    reader: &mut R,
    buf: &mut [u8],
) -> Result<()> {
    let mut total_read = 0;
    while total_read < buf.len() {
        let mut read_buf = ReadBuf::new(&mut buf[total_read..]);
        let before = read_buf.filled().len();
        
        tokio::io::AsyncReadExt::read_buf(reader, &mut read_buf)
            .await
            .context("failed to read from stream")?;
        
        let after = read_buf.filled().len();
        if after == before {
            return Err(anyhow::anyhow!(
                "unexpected EOF: read {} bytes, expected {}",
                total_read,
                buf.len()
            ));
        }
        
        total_read += after - before;
    }
    Ok(())
}

