use crate::{ cmd::{ CommandError, Get, Set }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor, HGet };

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        let args = extract_args(value)?;
        validate_command(&args, 2)?;

        Ok(Self {
            key: args[0].try_into()?,
            field: args[1].try_into()?,
        })
    }
}
