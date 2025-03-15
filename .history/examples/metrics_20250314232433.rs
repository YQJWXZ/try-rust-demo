use anyhow::Result;
use concurrency::Metrics;

fn main() -> Result<()> {
    let mut metrics = Metrics::new();
    metrics.inc("req.page.1");
    metrics.inc("call.thread.worker.1");

    println!("{:?}", metrics.snapshot());
    Ok(())
}
