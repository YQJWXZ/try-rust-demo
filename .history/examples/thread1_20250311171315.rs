use anyhow::Result;
use rand::{distr::Uniform, Rng};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

struct Msg {
    idx: usize,
    value: usize,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for _ in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    let range = Uniform::new(0, 100);
    loop {
        let value = rand::thread_rng().gen_range(range.clone());
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }

    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
