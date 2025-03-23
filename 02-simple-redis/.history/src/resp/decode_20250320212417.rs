/*
- 如何 serialize/deserialize Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - big number: "([+|-]<number>\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
*/
use crate::{RespDecode, RespError};
use anyhow::Result;
use bytes::BytesMut;

use super::{RespFrame, SimpleString};

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 1 {
            return Err(RespError::NotComplete);
        }

        let prefix = buf[0];
        let frame = match prefix {
            b'+' => RespFrame::SimpleString(SimpleString::decode(buf)?),
            b'-' => RespFrame::Error(Error::decode(buf)?),
            b':' => RespFrame::Integer(Integer::decode(buf)?),
            b'$' => RespFrame::BulkString(BulkString::decode(buf)?),
            b'*' => RespFrame::Array(Array::decode(buf)?),
            b'%' => RespFrame::Map(Map::decode(buf)?),
            b'~' => RespFrame::Set(Set::decode(buf)?),
            b'_' => RespFrame::Null(Null::decode(buf)?),
            b'#' => RespFrame::Boolean(Boolean::decode(buf)?),
            b',' => RespFrame::Double(Double::decode(buf)?),
            b'!' => RespFrame::BigNumber(BigNumber::decode(buf)?),
            _ => return Err(RespError::InvalidFrameType(format!("invalid prefix: {}", prefix))),
        };
        Ok(frame)
}

impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        // split the buffer
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[1..end]);
        Ok(SimpleString::new(s.to_string()))
    }
}

fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(b"+") {
        return Err(RespError::InvalidFrameType(format!(
            "expect: SimpleString(+), got: {:?}",
            buf
        )));
    }

    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;
    Ok(end)
}

// find nth CRLF in the buffer
fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;
    for (i, &b) in buf.iter().enumerate() {
        if b == b'\r' {
            count += 1;
            if count == nth {
                return Some(i);
            }
        }
    }
    None
}

fn parse_lenght(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_simple_frame_data(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);

    Ok((end, s.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_simple_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");
        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("OK".to_string()));

        Ok(())
    }
}
