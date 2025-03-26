use crate::{
    cmd::{CommandError, Get},
    RespArray, RespFrame,
};

use super::validate_command;

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;
    }
}
