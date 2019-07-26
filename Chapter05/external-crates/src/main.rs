#[macro_use] extern crate quick_error;

use std::convert::From;
use std::io;
use std::error::Error;

quick_error! {
    #[derive(Debug)]
    pub enum ErrorWrapper {
        ///
        /// 
        /// 
        InvalidDeviceIdError(device_id: usize) {
            from(device_id: usize) -> (device_id)
            description("No device present with this id, check formatting.")
        }

        DeviceNotPresentError(device_id: usize) {
            display("Device with id \"{}\" not found", device_id)
        }

        UnexpectedDeviceStateError {}

        Io(err: io::Error) {
            from(kind: io::ErrorKind) -> (io::Error::from(kind))
            description(err.description())
            display("I/O Error: {}", err)
        }  
    }
}


fn main() {
    println!("(IOError) {}", ErrorWrapper::from(io::ErrorKind::InvalidData));
    println!("(InvalidDeviceIdError) {}", ErrorWrapper::InvalidDeviceIdError(42));
    println!("(DeviceNotPresentError) {}", ErrorWrapper::DeviceNotPresentError(42));
    println!("(UnexpectedDeviceStateError) {}", ErrorWrapper::UnexpectedDeviceStateError {});
}
