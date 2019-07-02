use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use ring::digest;

#[no_mangle]
pub extern "C" fn digest(data: *mut c_char) -> *mut c_char {
    unsafe {

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
