//-- #########################
//-- Task: Routing using nickel
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 20 April 17
//-- #########################	

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:userid", middleware! { |request|
        format!("This is user: {:?}", request.param("userid"))
    });
    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));
    server.get("/a/**/d", middleware!("This matches /a/b/d and also /a/b/c/d"));

    server.listen("127.0.0.1:6767");
}