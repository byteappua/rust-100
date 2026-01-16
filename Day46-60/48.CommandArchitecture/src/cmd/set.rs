use super::Parse;
use crate::Error;
use bytes::Bytes;

#[derive(Debug)]
pub struct Set {
    pub key: String,
    pub value: Bytes,
}

impl Set {
    pub fn parse_frames(parse: &mut Parse) -> Result<Set, Error> {
        let key = parse.next_string()?;
        let value = parse.next_bytes()?;

        Ok(Set { key, value })
    }
}
