//-- #########################
//-- Task: Json handling in nickel
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 20 April 17
//-- #########################	

extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname:  String,
}

fn main() {
    let mut server = Nickel::new();

    server.post("/a/post/request", middleware! { |request, response|
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    });

    server.listen("127.0.0.1:6767");
}