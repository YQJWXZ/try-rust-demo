use super::RespEncode;

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let b = if self { "t" } else { "f" };
        format!("#{}\r\n", b).into_bytes()
    }
}
