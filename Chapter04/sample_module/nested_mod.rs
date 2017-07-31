//-- #########################
//-- Task: Nested module
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// sample_mod/nested.rs
pub fn sample_function() {
    println!("called `sample_module::nested::sample_function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}