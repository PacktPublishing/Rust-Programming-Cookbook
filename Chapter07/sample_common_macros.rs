//-- #########################
//-- Task: Implementing Commonn Macros
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 March 17
//-- #########################

fn main() {

    // Creating a vector 
    let v = vec![1, 2, 3, 4, 5];
    print!("Vector :- {:?}", v);
    
    // Macros used for testing
    assert!(true);
	assert_eq!(5, 3 + 2);

	// assert!(5 < 3);
	// assert_eq!(5, 3);

	// Gives a message to panic
    // panic!("oh no!");
}