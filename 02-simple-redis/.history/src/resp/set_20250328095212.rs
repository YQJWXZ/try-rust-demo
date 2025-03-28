use std::ops::Deref;

use super::{ frame::RespFrame, RespEncode, BUF_CAP };

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}

// - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for v in self.0 {
            buf.extend_from_slice(&v.encode());
        }

        buf
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet::new([
            RespArray::new([(1234).into(), true.into()]).into(),
            BulkString::new("world".to_string()).into(),
        ]).into();
        assert_eq!(frame.encode(), b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n")
    }
}
