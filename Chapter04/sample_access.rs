//-- #########################
//-- Task: To create a sample module to illustrating `self` and `super`
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

fn sample_function() {
    println!("called `sample_function()`");
}

// Defined a module names cool
mod cool {
    pub fn sample_function() {
        println!("called `cool::sample_function()` \n");
    }
}

mod sample_mod {
    fn sample_function() {
        println!("called `sample_mod::sample_function()` \n");
    }

    mod cool {
        pub fn sample_function() {
            println!("called `sample_mod::cool::sample_function()` \n");
        }
    }

    pub fn indirect_call() {
        // Let's access all the sample_functions named `sample_function` from this scope!
        print!("called `sample_mod::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `sample_mod`.
        // Calling `self::sample_function()` and calling `sample_function()` directly both give
        // the same result, because they refer to the same sample_function.
        self::sample_function();
        sample_function();

        // We can also use `self` to access another module inside `sample_mod`:
        self::cool::sample_function();

        // The `super` keyword refers to the parent scope (outside the `sample_mod` module).
        super::sample_function();

        // This will bind to the `cool::sample_function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use cool::sample_function as root_sample_function;
            root_sample_function();
        }
    }
}

// Execution starts here
fn main() {
    // Calling the sample_mod module's item 
    sample_mod::indirect_call();
}