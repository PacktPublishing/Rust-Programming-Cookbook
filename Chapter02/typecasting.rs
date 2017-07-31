// Task: Performing type casting in rust
// Date: 11 Feb 2016
// Version: 0.0.1
// Author: Vigneshwer

use std::{i32,f32};

// Sample function for assigning values to confusion matrix
fn main() {
 
// assigning random values to the confusion matrix
let(true_positive,true_negative,false_positive,false_negative)=(100,50,10,5);

// define a total closure
let total = true_positive + true_negative + false_positive + false_negative;

println!("The total predictions {}",total);
 
// Calculating the accuracy of the model
println!("Accuracy of the model {:.2}",percentage(accuracy(true_positive,true_negative,total)));

}

// Accuracy Measures the overall performance of the model
fn accuracy(tp:i32,tn:i32,total:i32) -> f32 {
 // if semi-colon is not put then that returns 
 // No automatic type cast in rust 
 (tp as f32 + tn as f32 )/(total as f32)
}

// Converting to percentage 
fn percentage(value:f32) -> f32 {
 value as f32 *100.0
}