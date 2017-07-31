//-- #########################
//-- Task: To create a sample nested_mod module
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

// Sample module which has struct item
mod sample_struct {
    // A public struct with a public field of generic type `T`
    pub struct WhiteBox<T> {
        pub information: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        information: T,
    }

    impl<T> BlackBox<T> {
        // A public constructor method
        pub fn const_new(information: T) -> BlackBox<T> {
            BlackBox {
                information: information,
            }
        }
    }
}

// Execution starts here 
fn main() {
    // Public structs with public fields can be constructed as usual
    let white_box = sample_struct::WhiteBox { information: "public information \n" };

    // and their fields can be normally accessed.
    println!("The white box contains: {} \n", white_box.information);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `BlackBox` has private fields
    //let black_box = sample_struct::BlackBox { information: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _black_box = sample_struct::BlackBox::const_new("classified information \n");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `information` field is private
    //println!("The black box contains: {}", _black_box.information);
    // TODO ^ Try uncommenting this line
}