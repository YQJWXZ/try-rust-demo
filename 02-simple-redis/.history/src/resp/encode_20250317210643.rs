use super::RespEncode;

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(b':');
        buf.extend(self.to_string().as_bytes());
        buf.push(b'\r');
        buf.push(b'\n');
        buf
    }
}
