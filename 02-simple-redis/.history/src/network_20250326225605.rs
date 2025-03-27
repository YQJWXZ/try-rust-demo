use tokio::net::TcpStream;
use anyhow::Result;

use crate::RespFrame;

struct RedisRequest {}
pub async fn stream_handler(stream: TcpStream) -> Result<()> {
    todo!()
}

async fn request_hadler(request: RespFrame) -> Result<RespFrame> {
    todo!()
}
