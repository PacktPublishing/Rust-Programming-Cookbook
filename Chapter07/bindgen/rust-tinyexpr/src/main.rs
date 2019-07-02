#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


fn main() {
    let expr = "sqrt(5^2+7^2+11^2+(8-2)^2)".to_owned();
    let result = unsafe {
        te_interp(CString::new(expr.clone()).unwrap().into_raw(), 0 as *mut i32)
    };
    println!("{} = {}", expr, result);
}