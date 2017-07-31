// Task : To explain variable binding in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 19 Feb 2017

fn main() {
	// Simplest variable binding
    let a = 5;
    // pattern
    let (b, c) = (1, 2);
    // type annotation
    let x_val: i32 = 5;
    // shadow example
    let y_val: i32 = 8;
    {
        println!("Value assigned when entering the scope : {}", y_val); // Prints "8".
        let y_val = 12;
        println!("Value modified within scope :{}", y_val); // Prints "12".
    }
    println!("Value which was assigned first : {}", y_val); // Prints "8".
    let y_val =  42;
    println!("New value assigned : {}", y_val); // Prints "42".

}