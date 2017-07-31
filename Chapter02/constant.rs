// Task : To explain constants in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 19 Feb 2017

// Global variables are declared outside scopes of other function
const  UPPERLIMIT: i32 = 12;

// function to check if bunber 
fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > UPPERLIMIT
}

fn main() {
    let random_number = 15;

    // Access constant in the main thread
    println!("The threshold is {}", UPPERLIMIT);
    println!("{} is {}", random_number, if is_big(random_number) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // UPPERLIMIT = 5;

}
