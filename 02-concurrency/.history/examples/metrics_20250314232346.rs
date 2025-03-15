use anyhow::Result;
use concurrency::Metrics;

fn main() -> Result<()> {
    let mut metrics = Metrics::new();
    metrics.inc("req.page.1");
}
