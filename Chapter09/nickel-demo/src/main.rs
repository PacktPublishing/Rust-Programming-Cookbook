//-- #########################
//-- Task: Starting a simple hello world nickel web app
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 20 April 17
//-- #########################	

#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}