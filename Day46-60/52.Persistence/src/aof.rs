use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use crate::{Command, Frame};
use std::path::Path;
use async_recursion::async_recursion;

pub struct Aof {
    writer: BufWriter<File>,
}

impl Aof {
    pub async fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .await?;

        Ok(Aof {
            writer: BufWriter::new(file),
        })
    }

    pub async fn append(&mut self, cmd: Command) -> anyhow::Result<()> {
        let frame = cmd.into_frame();
        self.write_frame(&frame).await?;
        self.writer.flush().await?;
        Ok(())
    }

    #[async_recursion]
    async fn write_frame(&mut self, frame: &Frame) -> anyhow::Result<()> {
        match frame {
            Frame::Array(val) => {
                self.writer.write_u8(b'*').await?;
                self.writer.write_all(val.len().to_string().as_bytes()).await?;
                self.writer.write_all(b"\r\n").await?;
                for entry in val {
                    self.write_frame(entry).await?;
                }
            }
            Frame::Bulk(val) => {
                self.writer.write_u8(b'$').await?;
                self.writer.write_all(val.len().to_string().as_bytes()).await?;
                self.writer.write_all(b"\r\n").await?;
                self.writer.write_all(val).await?;
                self.writer.write_all(b"\r\n").await?;
            }
            Frame::Simple(val) => {
                self.writer.write_u8(b'+').await?;
                self.writer.write_all(val.as_bytes()).await?;
                self.writer.write_all(b"\r\n").await?;
            }
            // Other types omitted for brevity
            _ => {}
        }
        Ok(())
    }
}
