#[macro_use]
extern crate actix_web;

use actix_web::{web, App, middleware, HttpServer, Responder, Result};
use std::{env};
use actix_files::NamedFile;


async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn path_parser(info: web::Path<(String, i32)>) -> impl Responder {
    format!("You tried to reach '{}/{}'", info.0, info.1)
}

async fn rust_cookbook() -> impl Responder {
    format!("Welcome to the Rust Cookbook")
}

#[get("/foxes")]
async fn foxes() -> Result<NamedFile> {
    Ok(NamedFile::open("static/foxes.jpg")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .route("/", web::get().to(index))
        .route("/welcome", web::get().to(rust_cookbook))
        .route("/{path}/{id}", web::get().to(path_parser))
        .service(foxes)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}