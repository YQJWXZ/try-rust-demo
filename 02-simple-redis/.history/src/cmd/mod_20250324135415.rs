// -Command
//    SET key val  "set hello world" =>  "*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
//    GET key      "get hello"       =>  "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
//  HSET key field val  "hset map hello world" =>   "*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
//    HGET key field      "hget map hello" => "*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"
//    HGETALL key          "hgetall hello" => "*2\r\n$7\r\nhgetall\r\n$5\r\nhello\r\n"
mod hmap;
mod map;
use crate::RespFrame;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {}

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

impl TryFrom<RespFrame> for Command {
    type Error = String;
    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        let mut cmd = value;
        let cmd_name = cmd.pop().unwrap();
        let cmd_name = cmd_name.as_str().unwrap();
        match cmd_name {
            "get" => Ok(Command::Get(Get::try_from(cmd)?)),
            "set" => Ok(Command::Set(Set::try_from(cmd)?)),
            "hget" => Ok(Command::HGet(HGet::try_from(cmd)?)),
            "hset" => Ok(Command::HSet(HSet::try_from(cmd)?)),
            "hgetall" => Ok(Command::HGetAll(HGetAll::try_from(cmd)?)),
            _ => Err("unknown command".to_string()),
        }
    }
}
