use anyhow::{anyhow, Result};
use std::time::Duration;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    // 创建 producer
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    // 创建 consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("consumer join error: {:?}", e))?;
    println!("secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let rand_sleep = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(rand_sleep));
        if rand::random::<u8>() % 5 == 0 {
            break;
        }
    }
    println!("producer {} exit\n", idx);
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
