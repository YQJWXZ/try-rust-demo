use anyhow::Result;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for _ in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(tx);
        })
    }

    Ok(())
}

fn producer(tx: mpsc::Sender<String>) {
  loop {
    let val
  }
}
