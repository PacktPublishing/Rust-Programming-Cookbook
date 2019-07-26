#[macro_use]
extern crate actix_web;

use actix_web::{web, App, middleware, HttpServer, Responder, Result};
use std::{env};
use actix_files as fs;


fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

fn path_parser(info: web::Path<(String, i32)>) -> impl Responder {
    format!("You tried to reach '{}/{}'", info.0, info.1)
}

fn rust_cookbook() -> impl Responder {
    format!("Welcome to the Rust Cookbook")
}

#[get("/foxes")]
fn foxes() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/foxes.jpg")?)
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(
        || App::new()
            .wrap(middleware::Logger::default())
            .service(foxes)
            .service(web::resource("/").to(index))
            .service(web::resource("/welcome").to(rust_cookbook))
            .service(web::resource("/{path}/{id}").to(path_parser)))
        .bind("127.0.0.1:8081")?
        .run()
}
