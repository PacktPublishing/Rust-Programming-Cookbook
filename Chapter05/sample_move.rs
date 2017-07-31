//-- #########################
//-- Task: Passing values to a thread in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

use std::thread;

fn main() {
    let x = 1;
    
    let handle = thread::spawn(move || {
       	(x)
    });

    println!("{:?}", handle.join().unwrap());
}