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

use super::SimpleString;

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end: usize = 0;
        if buf.len() < 3 {
            return Err(RespError::NotComplete);
        }

        if !buf.starts_with(b"+") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: SimpleString(+), got: {:?}",
                buf
            )));
        }

        if end == 0 {
            return Err(RespError::NotComplete);
        }

        // split the buffer
        let data = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&data[1..end]);
        Ok(SimpleString::new(s.to_string()))
    }
}

fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {}

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
