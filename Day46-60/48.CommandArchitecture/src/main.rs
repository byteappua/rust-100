pub mod cmd;

use bytes::{Buf, Bytes};
use std::io::Cursor;
use tracing::info;

pub use cmd::Command;

#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(i64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Incomplete frame")]
    Incomplete,
    #[error("Invalid protocol: {0}")]
    Other(String),
}

// ... (Rest of parsing logic from Day 47, kept for completeness of the file)
fn get_line(src: &mut Cursor<&[u8]>) -> Result<usize, Error> {
    let start = src.position() as usize;
    let end = src.get_ref().len();

    for i in start..end - 1 {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            return Ok(i);
        }
    }

    Err(Error::Incomplete)
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.get_u8())
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<i64, Error> {
    let line_end = get_line(src)?;
    let start = src.position() as usize;
    let line = &src.get_ref()[start..line_end];

    let s = std::str::from_utf8(line).map_err(|_| Error::Other("Invalid UTF-8".into()))?;
    let num = s.parse::<i64>().map_err(|_| Error::Other("Invalid integer".into()))?;

    src.set_position((line_end + 2) as u64); // +2 for \r\n
    Ok(num)
}

fn parse_simple(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
    let line_end = get_line(src)?;
    let start = src.position() as usize;
    let line = &src.get_ref()[start..line_end];

    let s = std::str::from_utf8(line).map_err(|_| Error::Other("Invalid UTF-8".into()))?;
    let frame = Frame::Simple(s.to_string());

    src.set_position((line_end + 2) as u64);
    Ok(frame)
}

fn parse_bulk(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
    let len = get_decimal(src)?;

    if len == -1 {
        return Ok(Frame::Null);
    }

    let len = len as usize;
    let n = len + 2; // content + \r\n

    if src.remaining() < n {
        return Err(Error::Incomplete);
    }

    let start = src.position() as usize;
    let data = Bytes::copy_from_slice(&src.get_ref()[start..start + len]);

    src.advance(n);

    Ok(Frame::Bulk(data))
}

fn parse_array(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
    let len = get_decimal(src)?;
    let mut out = Vec::with_capacity(len as usize);

    for _ in 0..len {
        out.push(parse(src)?);
    }

    Ok(Frame::Array(out))
}

pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    match get_u8(src)? {
        b'+' => parse_simple(src),
        b':' => {
            let val = get_decimal(src)?;
            Ok(Frame::Integer(val))
        },
        b'$' => parse_bulk(src),
        b'*' => parse_array(src),
        _ => Err(Error::Other("Unimplemented".into())),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Command Architecture Demo");

    // Example 1: SET hello world
    // Array(3) -> [Bulk(SET), Bulk(hello), Bulk(world)]
    let frames = vec![
        Frame::Bulk(Bytes::from("SET")),
        Frame::Bulk(Bytes::from("hello")),
        Frame::Bulk(Bytes::from("world")),
    ];
    let cmd = Command::from_frame(Frame::Array(frames))?;
    info!("Parsed Command: {:?}", cmd);

    // Example 2: GET hello
    let frames = vec![
        Frame::Bulk(Bytes::from("GET")),
        Frame::Bulk(Bytes::from("hello")),
    ];
    let cmd = Command::from_frame(Frame::Array(frames))?;
    info!("Parsed Command: {:?}", cmd);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command() {
        let frames = vec![
            Frame::Bulk(Bytes::from("SET")),
            Frame::Bulk(Bytes::from("key")),
            Frame::Bulk(Bytes::from("value")),
        ];
        let frame = Frame::Array(frames);
        let cmd = Command::from_frame(frame).unwrap();
        match cmd {
            Command::Set(set) => {
                assert_eq!(set.key, "key");
                assert_eq!(set.value, Bytes::from("value"));
            },
            _ => panic!("Expected Set command"),
        }
    }

    #[test]
    fn test_get_command() {
        let frames = vec![
            Frame::Bulk(Bytes::from("GET")),
            Frame::Bulk(Bytes::from("key")),
        ];
        let frame = Frame::Array(frames);
        let cmd = Command::from_frame(frame).unwrap();
        match cmd {
            Command::Get(get) => {
                assert_eq!(get.key, "key");
            },
            _ => panic!("Expected Get command"),
        }
    }

    #[test]
    fn test_unknown_command() {
        let frames = vec![
            Frame::Bulk(Bytes::from("FOOBAR")),
        ];
        let frame = Frame::Array(frames);
        let cmd = Command::from_frame(frame).unwrap();
         match cmd {
            Command::Unknown(u) => {
                assert_eq!(u.command_name, "foobar");
            },
            _ => panic!("Expected Unknown command"),
        }
    }
}
