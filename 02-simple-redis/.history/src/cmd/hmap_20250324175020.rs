use crate::{ cmd::{ CommandError, HGet, HSet, HGetAll }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor };

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) =>
                Ok(HGet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key or field".to_string())),
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) =>
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                    val: value,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key, field or value".to_string())),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) =>
                Ok(HGetAll {
                    key: String::from_utf8(key.0)?,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
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
    fn test_hget_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: HGet = frame.try_into()?;

        assert_eq!(result.key, "map");
        assert_eq!(result.field, "hello");
        Ok(())
    }

    #[test]
    fn test_hset_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: Set = frame.try_into()?;
        assert_eq!(result.key, "hello");
        assert_eq!(result.val, RespFrame::BulkString(b"world".into()));
        Ok(())
    }
}
