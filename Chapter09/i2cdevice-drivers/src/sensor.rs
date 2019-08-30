use std::io;
use rand::prelude::*;

pub trait Thermometer {
    fn temp_celsius(&mut self) -> Result<f32>;
}

type Result<T> = std::result::Result<T, io::Error>;

enum Register {
    Calib0 = 0x00,
    Data = 0x01,
}

#[allow(dead_code)]
pub struct RawI2CDeviceMock {
    path: String,
    device_id: u8,
}


impl RawI2CDeviceMock {
    pub fn new(path: String, device_id: u8) -> RawI2CDeviceMock {
        RawI2CDeviceMock {
            path: path,
            device_id: device_id,
        }
    }

    pub fn read(&self, register: u8) -> Result<u8> {
        let register = register as usize;
        if register == Register::Calib0 as usize {
            Ok(1_u8)
        } else { // register is the data register
            Ok(random::<u8>())
        }
    }
}

pub struct Bmx42Device {
    raw: RawI2CDeviceMock,
    calibration: u8,
}

impl Bmx42Device {
    pub fn new(device: RawI2CDeviceMock) -> Result<Bmx42Device> {
        let calib = device.read(Register::Calib0 as u8)?;
        Ok(Bmx42Device {
            raw: device,
            calibration: calib
        })
    }
}


impl Thermometer for Bmx42Device {
    fn temp_celsius(&mut self) -> Result<f32> {
        let raw_temp = self.raw.read(Register::Data as u8)?;
        // converts the result into something usable; from the specification
        Ok(((raw_temp as i8) << (self.calibration as i8)) as f32 / 10.0)
    }
}