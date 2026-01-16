use super::Parse;
use crate::Error;

#[derive(Debug)]
pub struct Get {
    pub key: String,
}

impl Get {
    pub fn parse_frames(parse: &mut Parse) -> Result<Get, Error> {
        let key = parse.next_string()?;

        Ok(Get { key })
    }
}
