 //-- #########################
 //-- Task: To generate a random number between the range of 1 to 10
 //-- Author: Vigneshwer.D
 //-- Version: 1.0.0
 //-- Date: 28 April 17
 //-- ######################### 

extern crate rand;
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(0, 10));
}