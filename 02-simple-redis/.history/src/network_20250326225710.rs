use tokio::net::TcpStream;
use anyhow::Result;

use crate::{ Backend, RespFrame };

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}
pub async fn stream_handler(stream: TcpStream) -> Result<()> {
    todo!()
}

async fn request_hadler(request: RespFrame) -> Result<RespFrame> {
    todo!()
}
