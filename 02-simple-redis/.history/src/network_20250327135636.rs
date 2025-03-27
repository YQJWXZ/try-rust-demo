use tokio::net::TcpStream;
use anyhow::Result;
use tokio_util::codec::{ Framed, Encoder, Decoder };
use crate::{ Backend, RespDecode, RespEncode, RespError, RespFrame };

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
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

impl Decoder<RespFrame> for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<RespFrame>> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
