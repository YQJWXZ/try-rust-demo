use anyhow::{ Result };
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379".parse()?;
    info!("Simple Redis server listening on {}", addr);
    Ok(())
}
