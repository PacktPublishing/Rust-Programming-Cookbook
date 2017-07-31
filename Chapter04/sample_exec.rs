//-- #########################
//-- Task: To create a sample executor of sample_lib in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Imports all items under sample_lib
extern crate sample_lib;

fn main() {
	// Calling public_function
    sample_lib::public_function();
    // Calling indirect_access to private_function
    sample_lib::indirect_access();
}