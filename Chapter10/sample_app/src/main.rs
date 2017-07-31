//-- #########################
//-- Task: Perform DELETE action
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
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
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

	    // Connect to the database
	    let client = Client::connect("localhost", 27017)
	      .ok().expect("Error establishing connection.");

	    // The users collection
	    let coll = client.db("rust-users").collection("users");

	    // Create cursor that finds all documents
	    let mut cursor = coll.find(None, None).unwrap();

	    // Opening for the JSON string to be returned
	    let mut data_result = "{\"data\":[".to_owned();

	    for (i, result) in cursor.enumerate() {
	        
	    	if let Ok(item) = result {
	    	        if let Some(&Bson::String(ref firstname)) = item.get("firstname") {
	    	           
	    	            let string_data = if i == 0 {
    	                    format!("{},", firstname)
    	                } else {
    	                    format!("{},", firstname)
    	                };
    	                data_result.push_str(&string_data);
	    	        }

	    	        
	    	    }
	    }

	    // Close the JSON string
	    data_result.push_str("]}");

	    // Send back the result
	    format!("{}", data_result)

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
	    let coll = client.db("rust-users").collection("users");

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

	    let client = Client::connect("localhost", 27017)
	        .ok().expect("Failed to initialize standalone client.");

	    // The users collection
	    let coll = client.db("rust-users").collection("users");

	    // Get the objectId from the request params
	    let object_id = request.param("id").unwrap();

	    // Match the user id to an bson ObjectId
	    let id = match ObjectId::with_string(object_id) {
	        Ok(oid) => oid,
	        Err(e) => return response.send(format!("{}", e))
	    };

	    match coll.delete_one(doc! {"_id" => id}, None) {
	        Ok(_) => (StatusCode::Ok, "Item deleted!"),
	        Err(e) => return response.send(format!("{}", e))
	    }

	});

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}