//-- #########################
//-- Task: Building your first macro in Rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 March 17
//-- #########################

// This is a simple macro named `say_hello`.
macro_rules! Welcome_RustBook {
    // `()` indicates that the macro takes no argument.
    () => (
        // The macro will expand into the contents of this block.
        println!("Welcome to Rust Cookbook!");
    )
}

fn main() {
    // This call will expand into `println!("Hello");`
    Welcome_RustBook!()
}