pub mod cmd;
pub mod db;
pub mod connection;
pub mod aof;
pub mod client;

use bytes::{Buf, Bytes};
use std::io::Cursor;

pub use cmd::Command;
pub use db::Db;
pub use connection::Connection;
pub use aof::Aof;
pub use client::Client;

// --- Frame and Error definitions ---
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
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

// --- Parsing Logic ---

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

    src.set_position((line_end + 2) as u64);
    Ok(num)
}

pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
     match get_u8(src)? {
        b'+' => {
            let len = get_line(src)?;
            src.set_position((len + 2) as u64);
            Ok(())
        }
        b'-' => {
            let len = get_line(src)?;
            src.set_position((len + 2) as u64);
            Ok(())
        }
        b':' => { get_decimal(src)?; Ok(()) }
        b'$' => {
            let len = get_decimal(src)?;
            if len == -1 { return Ok(()); }
            let len = len as usize;
            let n = len + 2;
            if src.remaining() < n { return Err(Error::Incomplete); }
            src.advance(n);
            Ok(())
        }
        b'*' => {
            let len = get_decimal(src)?;
            for _ in 0..len {
                check(src)?;
            }
            Ok(())
        }
        b => Err(Error::Other(format!("Unimplemented or Invalid byte: {}", b))),
    }
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
    if len == -1 { return Ok(Frame::Null); }
    let len = len as usize;
    let n = len + 2;
    if src.remaining() < n { return Err(Error::Incomplete); }
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
    if !src.has_remaining() { return Err(Error::Incomplete); }
    match get_u8(src)? {
        b'+' => parse_simple(src),
        b':' => { let val = get_decimal(src)?; Ok(Frame::Integer(val)) },
        b'$' => parse_bulk(src),
        b'*' => parse_array(src),
        _ => Err(Error::Other("Unimplemented".into())),
    }
}
pub mod server;
