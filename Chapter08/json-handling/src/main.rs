#[macro_use]
extern crate actix_web;

use actix_web::{
    guard, http::Method, middleware, web, App, HttpResponse, HttpServer,
};
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bookmark {
    id: i32,
    url: String,
}

#[get("by-id/{id}")]
fn bookmarks_by_id(id: web::Path<(i32)>) -> HttpResponse {
    let bookmark = Bookmark {
        id: *id,
        url: "https://blog.x5ff.xyz".into(),
    };
    HttpResponse::Ok().json(bookmark)
}

fn echo_bookmark(bookmark: web::Json<Bookmark>) -> HttpResponse {
    HttpResponse::Ok().json(bookmark.clone())
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                .service(
                    web::scope("/bookmarks")
                    .service(bookmarks_by_id)
                    .service(
                        web::resource("add/{id}")
                            .name("add")
                            .guard(guard::Any(guard::Put()).or(guard::Post()))
                            .to(echo_bookmark),
                    )
                    .default_service(web::route().method(Method::GET)),
            ))
    })
    .bind("127.0.0.1:8081")?
    .run()
}
