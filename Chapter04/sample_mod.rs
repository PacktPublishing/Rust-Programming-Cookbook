//-- #########################
//-- Task: To create a sample module to illustrate how to use a module in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Defined module named `sample_mod`
mod sample_mod {
    // By default all the items in module have private visibility
    fn private_function() {
        println!("called `sample_mod::private_function()` \n");
    }

    // Using the `pub` keyword changes it visibility to public
    pub fn sample_function() {
        println!("called `sample_mod::sample_function()` \n");
    }

    // Public items of the module can access the private visible items 
    pub fn indirect_private_fn() {
        print!("called `sample_mod::indirect_access()`, that\n> ");
        private_function();
    }
}

// Created a sample function to illustrate calling of 
fn sample_function() {
    println!("Called the `sample_function()` which is not a part of mod `sample_mod` \n");
}

// Execution of the program starts from here
fn main() {
    // Calling the sample_function which is outside module
    sample_function();
    // Calling the public visible sample_mod's sample_function
    sample_mod::sample_function();

    // Accessing the private fucntion indirectly
    sample_mod::indirect_private_fn();

    // Error! `private_function` is private
    //sample_mod::private_function();
    // TODO ^ Try uncommenting this line
}