use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    Ok(())
}
