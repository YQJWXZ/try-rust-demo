use crate::{
    cmd::{extract_args, validate_command, CommandError, Get},
    RespArray, RespFrame,
};
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
