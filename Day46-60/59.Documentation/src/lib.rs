//! # Mini-Redis-Doc
//!
//! `mini-redis-doc` is a simplified implementation of a Redis client and server,
//! designed for educational purposes to demonstrate Rust's asynchronous networking capabilities.
//!
//! ## Overview
//!
//! This crate provides a library for building Redis-compatible applications.
//! It includes:
//!
//! - A low-level **RESP Protocol** parser and serializer.
//! - An asynchronous **Client** for interacting with Redis servers.
//! - A **Connection** abstraction for managing TCP streams.
//! - A simple **Db** storage engine with basic command support (GET, SET, PUBLISH, SUBSCRIBE).
//!
//! ## Getting Started
//!
//! To use the client, add this crate to your dependencies and establish a connection:
//!
//! ```rust,no_run
//! use mini_redis_doc::Client;
//! use bytes::Bytes;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Connect to the mini-redis server
//!     let mut client = Client::connect("127.0.0.1:6379").await?;
//!
//!     // Set the value of "hello" to "world"
//!     client.set("hello", Bytes::from("world")).await?;
//!
//!     // Get the value of "hello"
//!     let val = client.get("hello").await?;
//!     println!("Got: {:?}", val);
//!
//!     Ok(())
//! }
//! ```
//!

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

/// Represents a Redis Serialization Protocol (RESP) Data Type.
///
/// See [Redis Protocol](https://redis.io/topics/protocol) for more details.
#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    /// A Simple String, e.g., `+OK\r\n`
    Simple(String),
    /// An Error, e.g., `-Error message\r\n`
    Error(String),
    /// An Integer, e.g., `:1000\r\n`
    Integer(i64),
    /// A Bulk String, e.g., `$6\r\nfoobar\r\n`
    Bulk(Bytes),
    /// A Null Bulk String, e.g., `$-1\r\n`
    Null,
    /// An Array of frames, e.g., `*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n`
    Array(Vec<Frame>),
}

/// Errors that can occur during protocol parsing or I/O.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Not enough data is available to parse a message.
    #[error("Incomplete frame")]
    Incomplete,
    /// Invalid protocol format.
    #[error("Invalid protocol: {0}")]
    Other(String),
    /// Underlying I/O error.
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

/// Checks if a complete frame can be decoded from `src`.
///
/// This function advances the `src` cursor to the end of the frame if successful.
/// If the frame is incomplete, `Error::Incomplete` is returned and the cursor is reset.
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

/// Parses a frame from `src`.
///
/// This function expects `check` to have been successfully called before.
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
