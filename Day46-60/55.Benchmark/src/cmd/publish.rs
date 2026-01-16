use super::Parse;
use crate::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Publish {
    pub channel: String,
    pub message: Bytes,
}

impl Publish {
    pub fn parse_frames(parse: &mut Parse) -> Result<Publish, Error> {
        let channel = parse.next_string()?;
        let message = parse.next_bytes()?;

        Ok(Publish { channel, message })
    }

    pub fn into_frame(self) -> crate::Frame {
        let mut frames = Vec::new();
        frames.push(crate::Frame::Bulk(Bytes::from("PUBLISH")));
        frames.push(crate::Frame::Bulk(Bytes::from(self.channel)));
        frames.push(crate::Frame::Bulk(self.message));
        crate::Frame::Array(frames)
    }
}
