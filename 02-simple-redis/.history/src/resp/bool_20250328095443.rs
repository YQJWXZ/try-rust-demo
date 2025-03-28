use super::RespEncode;

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let b = if self { "t" } else { "f" };
        format!("#{}\r\n", b).into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::frame::RespFrame;

    use super::*;

    #[test]
    fn test_boolean_encode() {
        let frame: RespFrame = true.into();
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame: RespFrame = false.into();
        assert_eq!(frame.encode(), b"#f\r\n");
    }
}
