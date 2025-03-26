use crate::{
    cmd::{CommandError, Get},
    RespArray,
};

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        // test if array have 2 elements

        // test if first element is a bulk string
    }
}
