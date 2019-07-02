use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

use ring::digest;

extern "C" {
    fn pre_digest() -> c_void;
}

#[no_mangle]
pub extern "C" fn digest(data: *mut c_char) -> *mut c_char {
    unsafe {
        pre_digest();

        let data = CStr::from_ptr(data);
        let signature = digest::digest(&digest::SHA256, data.to_bytes());

        let hex_digest = signature
            .as_ref()
            .iter()
            .map(|b| format!("{:X}", b))
            .collect::<String>();

        CString::new(hex_digest).unwrap().into_raw()
    }
}

// No tests :( 