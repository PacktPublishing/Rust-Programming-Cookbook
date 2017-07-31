//-- #########################
//-- Task: Creating a simple REST API
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 April 17
//-- ######################### 

#[macro_use]
extern crate nickel;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! { |request, response|

        format!("Hello from GET /users")

    });

    router.post("/users/new", middleware! { |request, response|

        format!("Hello from POST /users/new")

    });

    router.delete("/users/:id", middleware! { |request, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}