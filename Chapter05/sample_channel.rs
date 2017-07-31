//-- #########################
//-- Task: Using channels to perform safe pass of data between threads
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

// Using standard libraries
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// Declaring number of threads
static NO_THREADS: i32 = 3;

// Main thread starts
fn main() {
    // Creating endpoints of the channel
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for thread_id in 0..NO_THREADS {
        // Cloing the Sender
        let thread_tx = tx.clone();

        // Sending threads via the channel
        thread::spawn(move || {
            // thread sends the message to the channel
            thread_tx.send(thread_id).unwrap();
            println!("thread {} finished", thread_id);
        });
    }

    // Collecting all the threads
    let mut thread_holder = Vec::with_capacity(NO_THREADS as usize);
    for _ in 0..NO_THREADS {
        // Get the message from channel
        thread_holder.push(rx.recv());
    }

    // Print the execution order
    println!("{:?}", thread_holder);
}