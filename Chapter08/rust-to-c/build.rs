//-- #########################
//-- Task: Build script
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 14 April 17
//-- #########################

extern crate gcc;

fn main() {
    gcc::Config::new().file("src/double.c").compile("libdouble.a");
}
