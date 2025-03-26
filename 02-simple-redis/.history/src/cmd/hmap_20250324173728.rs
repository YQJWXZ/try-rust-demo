use crate::{ cmd::{ CommandError, Get, Set }, RespArray, RespFrame };

use super::{ extract_args, validate_command, CommandExceutor, HGet };

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {}
}
