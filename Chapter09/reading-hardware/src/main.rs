mod sensor;
use tokio::prelude::*;
use tokio::timer::Interval;

use sensor::{Bmx42Device, RawI2CDeviceMock, Thermometer};
use std::time::{Duration, UNIX_EPOCH, SystemTime, Instant};
use std::thread;

use std::sync::mpsc::channel;

fn current_timestamp_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

#[derive(Debug)]
struct Reading {
    timestamp: u64,
    value: f32
}


fn main() {
    let mut device = Bmx42Device::new(RawI2CDeviceMock::new("/dev/i2c-1".into(), 0x5f)).unwrap();

    let (sx, rx) = channel();

    thread::spawn(move || {
        while let Ok(reading) = rx.recv() {

            // or batch and save/send to somewhere
            println!("{:?}", reading);
        }
    });
    let task = Interval::new(Instant::now(), Duration::from_secs(1))
        .take(5)
        .for_each(move |_instant| {
            let sx = sx.clone();
            let temp = device.temp_celsius().unwrap();
            let _ = sx.send(Reading {
                timestamp: current_timestamp_ms(),
                value: temp
            });
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}
