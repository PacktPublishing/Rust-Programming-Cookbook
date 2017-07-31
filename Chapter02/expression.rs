// Task : To explain constants in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 19 Feb 2017

// main point of execution 
fn main() {

    // expression
    let x_val = 5u32;

    // y block 
    let y_val = {
        let x_squared = x_val * x_val;
        let x_cube = x_squared * x_val;

        // This expression will be assigned to `y_val`
        x_cube + x_squared + x_val
    };

    // z block
    let z_val = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x_val;
    };

    // printing the final outcomes
    println!("x is {:?}", x_val);
    println!("y is {:?}", y_val);
    println!("z is {:?}", z_val);
}