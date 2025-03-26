use crate::{ cmd::{ CommandError, HGet, HSet }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor, HGetAll };

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
