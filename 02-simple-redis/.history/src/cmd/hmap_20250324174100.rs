use crate::{ cmd::{ CommandError, Get, Set }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor, HGet, HSet };

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
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) =>
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(value.to_string().into_bytes())?,
                    val: value,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key or field or value".to_string())),
        }
    }
}
