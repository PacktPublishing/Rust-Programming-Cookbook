// Using standard libraries
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use rand::prelude::*;
use std::time::Duration;

enum ChartValue {
    Star(usize),
    Pipe(usize)
}

fn main() {
    let (tx, rx): (Sender<ChartValue>, Receiver<ChartValue>) = mpsc::channel();

    let pipe_sender = tx.clone();

    thread::spawn(move || {
        loop {
            pipe_sender.send(ChartValue::Pipe(random::<usize>() % 80)).unwrap();
            thread::sleep(Duration::from_millis(random::<u64>() % 800));
        }
    });

    let star_sender = tx.clone();
    thread::spawn(move || {
        loop {
            star_sender.send(ChartValue::Star(random::<usize>() % 80)).unwrap();
            thread::sleep(Duration::from_millis(random::<u64>() % 800));
        }
    });
    
    while let Ok(val) = rx.recv_timeout(Duration::from_secs(3)) {

        println!("{}", match val {
            ChartValue::Pipe(v) => "|".repeat(v + 1),
            ChartValue::Star(v) => "*".repeat(v + 1)
        });
    }
}