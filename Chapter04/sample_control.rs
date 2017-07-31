//-- #########################
//-- Task: To create a sample module to illustrating `use`
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::sample_function as other_function;

// Defined a nested 
mod deeply {
    pub mod nested {
        pub fn sample_function() {
            println!("called `deeply::nested::function()` \n")
        }
    }
}

fn sample_function() {
    println!("called `function()` \n");
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering a block \n");
    {
        // This is equivalent to `use deeply::nested::sample_function as sample_function`.
        // This `sample_function()` will shadow the outer one.
        use deeply::nested::sample_function;
        sample_function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        println!("Leaving the block \n");
    }

    sample_function();
}