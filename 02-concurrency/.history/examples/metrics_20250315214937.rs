use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    // start N workers and M requesters
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
            metrics.inc(format!("call.thread.workers.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // process request
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..256);
            metrics.inc(format!("req.page.{}", page)).unwrap();
        }

        #[allow(unreachable_code)]
        OK::<_, anyhow::Error>(())
    });

    Ok(())
}
