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
        let (socket, raddr) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket).await;
        });
    }
    Ok(())
}
