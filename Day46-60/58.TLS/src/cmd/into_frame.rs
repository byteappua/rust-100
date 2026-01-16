use crate::cmd::{Get, Set, Unknown, Command};
use crate::Frame;
use bytes::Bytes;

impl Command {
    pub fn into_frame(self) -> Frame {
        match self {
            Command::Get(cmd) => cmd.into_frame(),
            Command::Set(cmd) => cmd.into_frame(),
            Command::Publish(cmd) => cmd.into_frame(),
            Command::Subscribe(cmd) => cmd.into_frame(),
            Command::Ping(cmd) => cmd.into_frame(),
            Command::Unknown(cmd) => cmd.into_frame(),
        }
    }
}

impl Get {
    pub fn into_frame(self) -> Frame {
        let mut frames = Vec::new();
        frames.push(Frame::Bulk(Bytes::from("GET")));
        frames.push(Frame::Bulk(Bytes::from(self.key)));
        Frame::Array(frames)
    }
}

impl Set {
    pub fn into_frame(self) -> Frame {
        let mut frames = Vec::new();
        frames.push(Frame::Bulk(Bytes::from("SET")));
        frames.push(Frame::Bulk(Bytes::from(self.key)));
        frames.push(Frame::Bulk(self.value));
        Frame::Array(frames)
    }
}

impl Unknown {
    pub fn into_frame(self) -> Frame {
        Frame::Simple("Unknown".to_string())
    }
}
