use anyhow::{ Result };

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379".parse()?;

    Ok(())
}
