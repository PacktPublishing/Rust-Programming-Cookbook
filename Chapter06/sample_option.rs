//-- #########################
//-- Task: Implementing Options 
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 26 March 17
//-- #########################

// All arguments are handled explicitly using `match`.
fn compare_stmt_match(input: Option<&str>) {
    // Specify a course of action for each case.
    match input {
        Some("Rust CookBook") => println!("Rust CookBook was selected"),
        Some(inner)   => println!("Rust CookBook not selected"),
        None          => println!("No input provided"),
    }
}

// All arguments are handled implicitly using `unwrap`.
fn compare_stmt_unwrap(input: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None` value
    let inside_val = input.unwrap();
    if inside_val == "Another Book" { panic!("Rust CookBook is not selected"); }

    println!("I love {}s!!!!!", inside_val);
}

// main execution starts here
fn main() {
    let Desired_Book  = Some("Rust CookBook");
    let Another_Book = Some("Another Book");
    let Empty_value  = None;

    compare_stmt_match(Desired_Book);
    compare_stmt_match(Another_Book);
    compare_stmt_match(Empty_value);

    println!("*********************");

    let Rand_Book = Some("Random Book");
    let No_val = None;

    compare_stmt_unwrap(Rand_Book);
    compare_stmt_unwrap(No_val);
}