
mod sensor;

use sensor::{Bmx42Device, RawI2CDeviceMock, Thermometer};
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let mut device = Bmx42Device::new(RawI2CDeviceMock::new("/dev/i2c-1".into(), 0x5f)).unwrap();
    let pause = Duration::from_secs(1);
    loop {
        println!("Current temperature {} °C", device.temp_celsius().unwrap());
        sleep(pause);
    }
}