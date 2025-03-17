use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

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

async fn process_redis_connection(stream: TcpStream) -> Result<()> {}
