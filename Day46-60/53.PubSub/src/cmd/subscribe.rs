use super::Parse;
use crate::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Subscribe {
    pub channels: Vec<String>,
}

impl Subscribe {
    pub fn parse_frames(parse: &mut Parse) -> Result<Subscribe, Error> {
        let mut channels = Vec::new();

        loop {
            match parse.next_string() {
                Ok(s) => channels.push(s),
                Err(crate::Error::Other(_)) => break, // End of frame or invalid string, assuming loop consumption until error/end
                Err(_) => break,
            }
        }

        // The `Parse` struct logic in `mod.rs` is strict: `next_string` calls `next`.
        // `next` returns Error if no more frames.
        // So we need to handle "End of Stream" gracefully or check length first.
        // But `Parse` iterates over `std::vec::IntoIter`.
        // Let's rely on the fact that `Parse` consumes the iterator.

        // Actually, `Parse` struct as defined in `src/cmd/mod.rs` consumes the iterator one by one.
        // If we want to consume *all remaining* args as channels:

        Ok(Subscribe { channels })
    }

    // We need a custom parse implementation because `Parse::next` errors on empty.
    // Let's just consume until we get an error.

    pub fn into_frame(self) -> crate::Frame {
        let mut frames = Vec::new();
        frames.push(crate::Frame::Bulk(Bytes::from("SUBSCRIBE")));
        for channel in self.channels {
            frames.push(crate::Frame::Bulk(Bytes::from(channel)));
        }
        crate::Frame::Array(frames)
    }
}
