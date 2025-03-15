use std::{os::windows::thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    // start N workers and M requesters
    println!("{:?}", metrics::snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics::snapshot());
    }
}

fn task_worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        // do long term stuff
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.workers.{}", idx)).unwrap();
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || loop {
        // process request
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
