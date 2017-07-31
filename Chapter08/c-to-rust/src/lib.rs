//-- #########################
//-- Task: Rust Function for C
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 14 April 17
//-- #########################

#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}
