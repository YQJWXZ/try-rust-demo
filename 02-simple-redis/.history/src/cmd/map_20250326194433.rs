use crate::{ cmd::{ CommandError, Get, Set }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor };

impl CommandExceutor for Get {
    fn execute(&self, _db: &mut crate::db::Database) -> Result<(), CommandError> {
        // TODO: implement
        Ok(())
    }
}

impl CommandExceutor for Set {
    fn execute(&self, _db: &mut crate::db::Database) -> Result<(), CommandError> {
        // TODO: implement
        Ok(())
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) =>
                Ok(Get {
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

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) =>
                Ok(Set {
                    key: String::from_utf8(key.0)?,
                    val: value,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key or value".to_string())),
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

    #[test]
    fn test_set_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: Set = frame.try_into()?;
        assert_eq!(result.key, "hello");
        assert_eq!(result.val, RespFrame::BulkString(b"world".into()));
        Ok(())
    }
}
