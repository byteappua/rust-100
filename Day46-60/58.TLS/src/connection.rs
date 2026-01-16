use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use crate::{Frame, Error};
use std::io::Cursor;
use crate::server::AsyncStream;

/// Send and receive `Frame` values from a remote peer.
pub struct Connection {
    stream: BufWriter<Box<dyn AsyncStream>>,
    buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new<S: AsyncStream>(socket: S) -> Connection {
        Connection {
            stream: BufWriter::new(Box::new(socket)),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Read a single `Frame` value from the underlying stream.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            if 0 == self.stream.read_buf(&mut self.buffer).await.map_err(|e| Error::Other(e.to_string()))? {
                // The remote closed the connection.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(Error::Other("connection reset by peer".into()));
                }
            }
        }
    }

    /// Tries to parse a frame from the buffer.
    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match crate::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = crate::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            },
            Err(Error::Incomplete) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Write a single `Frame` value to the underlying stream.
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val.as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val.as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val.to_string().as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
            }
            Frame::Bulk(val) => {
                self.stream.write_u8(b'$').await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val.len().to_string().as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
            }
            Frame::Array(val) => {
                self.stream.write_u8(b'*').await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(val.len().to_string().as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
                for entry in val {
                    // Simple recursive write
                    match entry {
                        Frame::Bulk(v) => {
                             self.stream.write_u8(b'$').await.map_err(|e| Error::Other(e.to_string()))?;
                             self.stream.write_all(v.len().to_string().as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                             self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
                             self.stream.write_all(v).await.map_err(|e| Error::Other(e.to_string()))?;
                             self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
                        }
                         Frame::Integer(v) => {
                            self.stream.write_u8(b':').await.map_err(|e| Error::Other(e.to_string()))?;
                            self.stream.write_all(v.to_string().as_bytes()).await.map_err(|e| Error::Other(e.to_string()))?;
                            self.stream.write_all(b"\r\n").await.map_err(|e| Error::Other(e.to_string()))?;
                         }
                        _ => return Err(Error::Other("Nested arrays or other types not supported in Array yet".into())),
                    }
                }
            }
        }

        self.stream.flush().await.map_err(|e| Error::Other(e.to_string()))?;

        Ok(())
    }
}
