use crate::{
    cmd::{CommandError, Get},
    RespArray, RespFrame,
};

use super::{extract_args, validate_command, CommandExceutor, Set};

// impl CommandExceutor for Get {
//     fn execute(&self, _db: &mut crate::db::Database) -> Result<(), CommandError> {
//         // TODO: implement
//         Ok(())
//     }
// }

// impl CommandExceutor for Set {
//     fn execute(&self, _db: &mut crate::db::Database) -> Result<(), CommandError> {
//         // TODO: implement
//         Ok(())
//     }
// }

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Get {
                key: String::from_utf8(key.0)?,
            }),

            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["set"], 2)?;

        let mut args = extract_args(value, 2)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some()) => Ok(Set {}),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::RespDecode;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_get_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");
        let frame = RespArray::decode(&mut buf)?;

        let result: Get = frame.try_into()?;
        assert_eq!(result.key, "hello");

        Ok(())
    }
}
