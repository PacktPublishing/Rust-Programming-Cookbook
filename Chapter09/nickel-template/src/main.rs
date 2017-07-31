//-- #########################
//-- Task: Templating in nickel
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 20 April 17
//-- #########################	

#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "viki");
        return response.render("examples/assets/template.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}