use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use crate::{Frame, Error};
use std::io::Cursor;

/// Send and receive `Frame` values from a remote peer.
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
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
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await.map_err(|e| Error::Other(e.to_string()))? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(Error::Other("connection reset by peer".into()));
                }
            }
        }
    }

    /// Tries to parse a frame from the buffer. If the buffer contains enough
    /// data, the frame is returned and the data removed from the buffer. If not
    /// enough data has been buffered yet, `None` is returned.
    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        let mut buf = Cursor::new(&self.buffer[..]);

        // check if enough data is available
        match crate::check(&mut buf) {
            Ok(_) => {
                // The check logic advanced the cursor, but we want to reset it
                // to parse the data again into a Frame.
                // In a real implementation, `check` and `parse` might be combined or optimized.
                // Here we just use the length to know how much to parse.
                let len = buf.position() as usize;

                buf.set_position(0);

                // Parse the frame
                let frame = crate::parse(&mut buf)?;

                // Discard the parsed data from the read buffer
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
            Frame::Array(_val) => {
                return Err(Error::Other("Array encoding not implemented for simplicity".into()));
            }
        }

        // Ensure the data is flushed to the socket
        self.stream.flush().await.map_err(|e| Error::Other(e.to_string()))?;

        Ok(())
    }
}
