//-- #########################
//-- Task: Rust-python module
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 14 April 17
//-- #########################

#[macro_use] extern crate cpython;

use cpython::{Python, PyResult};

// Fibonacci implementation in Rust
fn fibo(py: Python, n : u64) -> PyResult<u64> {
    if n < 2 {
        return Ok(1)
    }
    let mut prev1 = 1;
    let mut prev2 = 1;
    for _ in 1..n {
        let new = prev1 + prev2;
        prev2 = prev1;
        prev1 = new;
    }
    Ok(prev1) 
}

// To build a Python compatible module we need an intialiser which expose the public interface
py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    // Expose the function fibo as `extern "C"`
    try!(m.add(py, "fibo", py_fn!(py, fibo(rand_int: u64))));

    // Initialiser's macro needs a Result<> as return value
    Ok(())
});
