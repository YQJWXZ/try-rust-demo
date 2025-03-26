use crate::{ cmd::{ CommandError, Get, Set }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor, HGet };

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) =>
                Ok(Set {
                    key: String::from_utf8(key.0)?,
                    val: value,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key or value".to_string())),
        }
    }
}
