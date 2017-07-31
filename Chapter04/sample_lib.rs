//-- #########################
//-- Task: To create a sample library in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

pub fn public_function() {
    println!("called sample_lib `public_function()`");
}

fn private_function() {
    println!("called sample_lib `private_function()`");
}

pub fn indirect_access() {
    print!("called sample_lib `indirect_access()`, that\n> ");

    private_function();
}