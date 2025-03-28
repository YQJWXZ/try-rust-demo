use super::RespEncode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNull;

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}
