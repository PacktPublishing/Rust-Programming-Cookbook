//-- #########################
//-- Task: To create a sample nested_mod module
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Defined module named `sample_mod`
mod sample_mod {

    // Defined public Nested module named `nested_mod`
    pub mod nested_mod {
        pub fn function() {
            println!("called `sample_mod::nested_mod::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `sample_mod::nested_mod::private_function()`");
        }
    }

    // Nested modules follow the same rules for visibility
    mod private_nested_mod {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `sample_mod::private_nested_mod::function()`");
        }
    }
}

// Execution starts from main function
fn main() {

    sample_mod::nested_mod::function();

    // Private items of a module cannot be directly accessed, even if nested_mod in a public module

    // Error! `private_function` is private
    //sample_mod::nested_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested_mod` is a private module
    //sample_mod::private_nested_mod::function();
    // TODO ^ Try uncommenting this line
}