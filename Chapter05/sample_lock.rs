//-- #########################
//-- Task: Safe Mutuable access across threads for preventing data races
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

// Call the standard library
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Main thread
fn main() {
    // Declaring a Arc type data
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Creating 3 threads and implementing lock
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
            println!("Thread id : {:?}",i );
            println!("Data value :{:?}", data[0]);
        });
    }

    thread::sleep(Duration::from_millis(10));
}