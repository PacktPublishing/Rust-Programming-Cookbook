//-- #########################
//-- Task: Making sequential code parallel
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

// Calling the rayon crate
extern crate rayon;
use rayon::prelude::*;

// Sum of squares fucntion
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}

// Main execution of code
fn main() {
	// Declaring a random variable of 10
	let rand_val = 10;
	// Calling the method to get sum_of_squares
	let sum_sq = sum_of_squares(&[rand_val]);
	// Printing the result
	println!("Sum of squares of {0} is {1}",rand_val,sum_sq);
}
