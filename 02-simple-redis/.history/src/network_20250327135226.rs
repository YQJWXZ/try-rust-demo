use tokio::net::TcpStream;
use anyhow::Result;
use tokio_util::codec::{ Framed, Encoder, Decoder };
use crate::{ Backend, RespEncode, RespFrame };

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

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> Result<()> {
        let encoded = item.encode();
        dst.extend_from_slice(&excoded);
    }
}
