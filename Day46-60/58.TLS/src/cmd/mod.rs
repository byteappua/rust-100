pub mod get;
pub mod set;
pub mod publish;
pub mod subscribe;
pub mod unknown;
pub mod ping;
pub mod into_frame;

use crate::{Frame, Error};
use self::get::Get;
use self::set::Set;
use self::publish::Publish;
use self::subscribe::Subscribe;
use self::unknown::Unknown;
use self::ping::Ping;

#[derive(Debug, Clone)]
pub enum Command {
    Get(Get),
    Set(Set),
    Publish(Publish),
    Subscribe(Subscribe),
    Ping(Ping),
    Unknown(Unknown),
}

impl Command {
    pub fn from_frame(frame: Frame) -> Result<Command, Error> {
        let mut parse = Parse::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match command_name.as_str() {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
            "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        parse.finish()?;

        Ok(command)
    }
}

pub struct Parse {
    parts: std::vec::IntoIter<Frame>,
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, Error> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(Error::Other(format!("protocol error; expected array, got {:?}", frame))),
        };

        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    pub fn next(&mut self) -> Result<Frame, Error> {
        self.parts.next().ok_or(Error::Other("protocol error; expected frame".into()))
    }

    pub fn next_string(&mut self) -> Result<String, Error> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => std::str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| Error::Other("protocol error; invalid string".into())),
            frame => Err(Error::Other(format!("protocol error; expected simple frame or bulk frame, got {:?}", frame))),
        }
    }

    pub fn next_bytes(&mut self) -> Result<bytes::Bytes, Error> {
        match self.next()? {
            Frame::Simple(s) => Ok(bytes::Bytes::from(s)),
            Frame::Bulk(data) => Ok(data),
            frame => Err(Error::Other(format!("protocol error; expected simple frame or bulk frame, got {:?}", frame))),
        }
    }

    pub fn finish(&mut self) -> Result<(), Error> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err(Error::Other("protocol error; expected end of frame".into()))
        }
    }
}
