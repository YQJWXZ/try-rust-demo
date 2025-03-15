use anyhow::Result;
use std::{sync::mpsc, thread};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });

    Ok(())
}
