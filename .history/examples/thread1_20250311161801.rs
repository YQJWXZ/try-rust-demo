use anyhow::Result;
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
    loop {
        let value = rand::random::<usize>();
        tx.send(value)?;
        thread::sleep(Duration::from_millis(1000));
    }
}
