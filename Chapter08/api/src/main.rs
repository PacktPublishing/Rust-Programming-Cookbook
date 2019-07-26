#[macro_use]
extern crate actix_web;

use actix_files as fs;
use actix_web::{ guard,
    http::header, http::Method, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use std::env;
use std::path::PathBuf;

#[get("by-id/{id}")]
fn bookmark_by_id(id: web::Path<(i32)>) -> impl Responder {
    format!("{{ \"id\": {}, \"url\": \"https://blog.x5ff.xyz\" }}", id)
}

fn echo_bookmark(req: HttpRequest) -> impl Responder {
    let id: i32 = req.match_info().query("id").parse().unwrap();
    format!("{:?}", id)
}

#[get("/captures/{tail:.*}")]
fn captures(req: HttpRequest) -> Result<fs::NamedFile> {
    let mut root = PathBuf::from("static/");
    let tail: PathBuf = req.match_info().query("tail").parse().unwrap();
    root.push(tail);

    Ok(fs::NamedFile::open(root)?)
}

#[get("from-bitly/{bitlyid}")]
fn bit_ly(req: HttpRequest) -> HttpResponse {
    let bitly_id = req.match_info().get("bitlyid").unwrap();
    let url = req.url_for("bitly", &[bitly_id]).unwrap();
    HttpResponse::Found()
        .header(header::LOCATION, url.into_string())
        .finish()
        .into_body()
}

#[get("/")]
fn bookmarks_index() -> impl Responder {
    format!("Welcome to your quick and easy bookmarking service!")
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
                    .service(captures)
                    .service(bookmark_by_id)
                    .service(bit_ly)
                    .service(web::resource("add/{id}")
                        .name("add") 
                        .guard(guard::Any(guard::Put()).or(guard::Post()))
                        .to(echo_bookmark))
            ))
            .service(
                web::scope("/bookmarks")
                    .service(bookmarks_index)
            )
            .external_resource("bitly", "https://bit.ly/{bitly}")
            
    })
    .bind("127.0.0.1:8081")?
    .run()
}
