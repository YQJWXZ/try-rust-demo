use tokio::net::TcpStream;
use anyhow::Result;
use tokio_util::codec::{ Framed, Encoder, Decoder };
use crate::{ Backend, RespFrame };

#[derive(Debug)]
pub struct RespFrameCodec;
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

async fn request_hadler(request: RedisRequest) -> Result<RedisResponse> {
    todo!()
}

impl<T> Encoder<T> for RespFrameCodec where T: Into<RespFrame> {}
