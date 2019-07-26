mod macros {
    // Exporting macros to other modules
    #[macro_export]
    macro_rules! two_plus_two {
        () => { 2 + 2 };
    }
}

// A simple macro without arguments
macro_rules! one_plus_one {
    () => { 1 + 1 };
}


// A simple pattern matching argument
macro_rules! one_and_one {
    (plus) => { 1 + 1 };
    (minus) => { 1 - 1 };
    (mult) => { 1 * 1 };
}

fn main() {

    println!("Did the pattern match? '{}'", strange_patterns!(The pattern must match precisely));

    make_fn!(awesome_fn, { println!("Called awesome_fn")});
    awesome_fn();
    
    println!("1 + 1 = {}", one_plus_one!());
    println!("1 + 1 = {}", one_and_one!(plus));
    println!("1 - 1 = {}", one_and_one!(minus));
    println!("1 * 1 = {}", one_and_one!(mult));

    println!("2 + 2 = {}", two_plus_two!());

    n_times!({10}, || println!("Hello World!"));
    // n_times!(-10, || println!("negative?"));
    // n_times!(2.2, || println!("floats?"));
}
