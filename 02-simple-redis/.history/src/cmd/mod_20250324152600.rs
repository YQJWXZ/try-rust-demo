// -Command
//    SET key val  "set hello world" =>  "*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
//    GET key      "get hello"       =>  "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
//  HSET key field val  "hset map hello world" =>   "*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
//    HGET key field      "hget map hello" => "*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"
//    HGETALL key          "hgetall hello" => "*2\r\n$7\r\nhgetall\r\n$5\r\nhello\r\n"
mod hmap;
mod map;

use crate::{RespArray, RespError, RespFrame};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("{0}")]
    RespError(#[from] RespError),
}

pub trait CommandExceutor {
    fn execute(&self) -> RespFrame;
}
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    val: RespFrame,
}
#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    val: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}

/**
 * validate command
 */
fn validate_command(value: &RespArray, names: &[&str], n_args: usize) -> Result<(), CommandError> {
    if value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }

    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.as_ref().to_ascii_lowercase() != *name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command name: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd.as_ref())
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first element".to_string(),
                ))
            }
        }
    }
    Ok(())
}

fn extract_args(value: RespArray, n_args: usize) -> Result<RespArray, CommandError> {}
