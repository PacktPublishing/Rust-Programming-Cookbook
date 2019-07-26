use std::fmt::Debug;

///
/// An interface that can be used for quick and easy logging
/// 
pub trait Loggable: Debug + Sized {
    fn log(self) {
        println!("{:?}", &self)
    }
}
///
/// A simple print function for printing debug formatted variables
/// 
fn log_debug<T: Debug>(t: T) {
    println!("{:?}", t);
}

#[derive(Debug)]
struct ArbitraryType {
    v: Vec<i32>
}

impl ArbitraryType {
    pub fn new() -> ArbitraryType {
        ArbitraryType {
            v: vec![1,2,3,4]
        }
    }
}
impl Loggable for ArbitraryType {}

#[derive(Debug)]
struct AnotherType(usize);

fn main() {
    let a = ArbitraryType::new();
    a.log();
    let b = AnotherType(2);
    log_debug(b);
}
