use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

const BUF_SIZE: usize = 4096;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis listening on {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);
        tokio::spawn(async move {
            process_redis_connection(stream).await.unwrap();
        });
    }
    Ok(())
}

async fn process_redis_connection(stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock` error
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,

            Ok(_) => {
                let cmd = String::from_utf8_lossy(&buf);
                info!("Received command: {}", cmd);
                // process command
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // This is a false positive, we'll wait for the next event
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
