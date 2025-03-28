use std::{ collections::BTreeMap, ops::{ Deref, DerefMut } };

use super::{ frame::RespFrame, simple_string::SimpleString, RespEncode, BUF_CAP };

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
// we only support string key which encode to SimpleString
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (k, v) in self.0 {
            buf.extend_from_slice(&SimpleString::new(k).encode());
            buf.extend_from_slice(&v.encode());
        }

        buf
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}
impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_encode() {
        let mut map = RespMap::new();
        map.insert("hello".to_string(), BulkString::new("world".to_string()).into());
        map.insert("foo".to_string(), (-123456.789).into());

        let frame: RespFrame = map.into();

        assert_eq!(
            // String::from_utf8_lossy(&frame.encode()),
            frame.encode(),
            b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n"
        )
    }
}
