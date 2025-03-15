use anyhow::Result;
use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let mut metrics = Metrics::new();

    // start N workers and M requesters
    println!("{:?}", metrics::snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    println!("{:?}", metrics.snapshot());
    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || {})
}
