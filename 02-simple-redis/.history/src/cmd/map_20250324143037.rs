use crate::{
    cmd::{CommandError, Get},
    RespArray, RespFrame,
};

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        // test if array have 2 elements
        if value.len() != 2 {
            return Err(CommandError::InvalidArgument(
                "GET command must have exactly 1 arguments".to_string(),
            ));
        }
        // test if first element is a bulk string
        let key = match value[1] {
          RespFrame::BulkString(key) => match  String::from_utf8(key.0) {
            Ok(key) => key,
            Err(_) => return Err(CommandError::InvalidArgument(
              "GET command must have a key as the first argument".to_string())),
          },
          _ => return Err(CommandError::InvalidArgument("Invalid key".to_string())),
              
          }
        }
    }
}
