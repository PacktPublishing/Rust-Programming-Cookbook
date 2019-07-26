use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct MyError {
    code: usize,
}

impl Error for MyError {
    fn description(&self) -> &str {
        "Occurs when someone makes a mistake"
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Error code {:#X}", self.code) 
    }
}


fn main() {
    println!("Display: {}", MyError{ code: 1535 });
    println!("Debug: {:?}", MyError{ code: 42 });
    println!("Description: {:?}", (MyError{ code: 42 }).description());
}