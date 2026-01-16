use bytes::{Buf, Bytes};
use std::io::Cursor;
use tracing::info;

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
    #[error("Invalid protocol")]
    Other(String),
}

/// Checks if a full line is available in the buffer.
/// Returns Ok(line_end_index) if found, Err(Incomplete) otherwise.
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
    info!("RESP Parser Demo");

    // Example Usage
    let data = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
    let mut cursor = Cursor::new(&data[..]);

    match parse(&mut cursor) {
        Ok(frame) => info!("Parsed frame: {:?}", frame),
        Err(e) => info!("Error parsing: {:?}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string() {
        let data = b"+OK\r\n";
        let mut cursor = Cursor::new(&data[..]);
        let frame = parse(&mut cursor).unwrap();
        assert_eq!(frame, Frame::Simple("OK".to_string()));
    }

    #[test]
    fn test_integer() {
        let data = b":1000\r\n";
        let mut cursor = Cursor::new(&data[..]);
        let frame = parse(&mut cursor).unwrap();
        assert_eq!(frame, Frame::Integer(1000));
    }

    #[test]
    fn test_bulk_string() {
        let data = b"$6\r\nfoobar\r\n";
        let mut cursor = Cursor::new(&data[..]);
        let frame = parse(&mut cursor).unwrap();
        if let Frame::Bulk(b) = frame {
            assert_eq!(&b[..], b"foobar");
        } else {
            panic!("Expected bulk string");
        }
    }

    #[test]
    fn test_array() {
        let data = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let mut cursor = Cursor::new(&data[..]);
        let frame = parse(&mut cursor).unwrap();
        if let Frame::Array(vec) = frame {
            assert_eq!(vec.len(), 2);
             if let Frame::Bulk(b) = &vec[0] { assert_eq!(&b[..], b"foo"); }
             if let Frame::Bulk(b) = &vec[1] { assert_eq!(&b[..], b"bar"); }
        } else {
            panic!("Expected array");
        }
    }
}
