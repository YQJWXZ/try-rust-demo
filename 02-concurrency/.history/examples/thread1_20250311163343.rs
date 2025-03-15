use anyhow::Result;
use rand::{distr::Uniform, Rng};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for _ in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(i, tx);
        })
    }

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<usize>) -> Result<()> {
    let range = Uniform::from(0..100);
    loop {
        let value = rand::thread_rng().gen_range(range.clone());
        tx.send(value)?;
        thread::sleep(Duration::from_millis(1000));
    }
}
