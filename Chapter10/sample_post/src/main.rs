//-- #########################
//-- Task: Perform POST action
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 April 17
//-- ######################### 

#[macro_use]
extern crate nickel;

extern crate rustc_serialize;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// Nickel
use nickel::{Nickel, JsonBody, HttpRouter};
use nickel::status::StatusCode::{self};

// MongoDB
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

// bson
use bson::{Bson, Document};
use bson::oid::ObjectId;

// rustc_serialize
use rustc_serialize::json::{Json, ToJson};

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    firstname: String,
    lastname: String,
    email: String
}


fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! { |request, response|

        format!("Hello from GET /users")

    });

	router.post("/users/new", middleware! { |request, response|

	    // Accept a JSON string that corresponds to the User struct
	    let user = request.json_as::<User>().unwrap();

	    let firstname = user.firstname.to_string();
	    let lastname = user.lastname.to_string();
	    let email = user.email.to_string();

	    // Connect to the database
	    let client = Client::connect("localhost", 27017)
	        .ok().expect("Error establishing connection.");

	    // The users collection
	    let coll = client.db("rust-cookbook").collection("users");

	    // Insert one user
	    match coll.insert_one(doc! {
	        "firstname" => firstname,
	        "lastname" => lastname,
	        "email" => email
	    }, None) {
	        Ok(_) => (StatusCode::Ok, "Item saved!"),
	        Err(e) => return response.send(format!("{}", e))
	    }

	});

	    router.delete("/users/:id", middleware! { |request, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}