//-- #########################
//-- Task: Implement matching
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 March 17
//-- #########################

macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

fn main() {
    foo!(y => 3);
}