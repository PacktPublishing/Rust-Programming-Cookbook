// Task : Basic mathematical model on 2 numbers
// Date : 26th Dec 2016
// Version : 1.0
// Author : Vigneshwer

// Libraries in rust
use std::io;
use std::{i32};

// Main Functions
fn main() {

 // Entering number 1 
 println!("Enter First number ? ");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1)
        .expect("Failed to read line");

    // Entering number 2 
    println!("Enter second number ? ");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2)
        .expect("Failed to read line");

    // Convert to int 
 let a_int: i32 = input_1.trim().parse()
 .ok()
 .expect("Please type a number!");
 
 let b_int: i32 = input_2.trim().parse()
 .ok()
 .expect("Please type a number!");

 // output of basic operations
 println!("sum is: {}", a_int + b_int);
 println!("difference is: {}", a_int - b_int);
 println!("mutlipy is: {}", a_int * b_int);
 println!("division is: {}", a_int / b_int);

}