//-- #########################
//-- Task: To create a sample file structure
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Similarly `mod sample_private` and `mod nested_mod` will locate the `nested_mod.rs`
// and `sample_private.rs` files and insert them here under their respective
// modules
mod sample_private;
pub mod nested_mod;

pub fn sample_function() {
    println!("called `sample_module::sample_function()`");
}

fn private_function() {
    println!("called `sample_module::private_function()`");
}

pub fn indirect_access() {
    print!("called `sample_module::indirect_access()`, that\n> ");

    private_function();
}
