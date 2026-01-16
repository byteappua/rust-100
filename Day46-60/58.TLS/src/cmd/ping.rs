use super::Parse;
use crate::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Ping {
    pub msg: Option<Bytes>,
}

impl Ping {
    pub fn parse_frames(parse: &mut Parse) -> Result<Ping, Error> {
        match parse.next_bytes() {
            Ok(msg) => Ok(Ping { msg: Some(msg) }),
            Err(Error::Other(_)) => Ok(Ping { msg: None }), // End of frame, no argument
            Err(e) => Err(e),
        }
    }

    pub fn into_frame(self) -> crate::Frame {
        let mut frames = Vec::new();
        frames.push(crate::Frame::Bulk(Bytes::from("PING")));
        if let Some(msg) = self.msg {
            frames.push(crate::Frame::Bulk(msg));
        }
        crate::Frame::Array(frames)
    }
}
