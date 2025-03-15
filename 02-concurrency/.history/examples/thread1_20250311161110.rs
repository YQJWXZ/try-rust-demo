use anyhow::Result;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });

    Ok(())
}
