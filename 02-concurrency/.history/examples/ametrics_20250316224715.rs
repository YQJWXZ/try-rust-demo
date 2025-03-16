use anyhow::Result;
use concurrency::CmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new();

    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        // println!("{:?}", metrics.snapshot());
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
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

fn request_worker(metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // process request
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
