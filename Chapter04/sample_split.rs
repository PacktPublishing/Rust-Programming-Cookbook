//-- #########################
//-- Task: To create a sample file structure
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Using the contents of sample_module
mod sample_module;

// Defining a local sample_function
fn sample_function() {
    println!("called `sample_function()`");
}

// Execution starts here
fn main() {
    sample_module::sample_function();

    sample_function();

    sample_module::indirect_access();

    sample_module::nested_mod::sample_function();
}