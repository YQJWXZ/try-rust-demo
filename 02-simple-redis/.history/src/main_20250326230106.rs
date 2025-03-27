use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{ info, warn };

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple Redis server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = crate::stream_handler(stream).await {
                warn!("")
            }
        });
    }
    Ok(())
}
