
#[macro_use]
extern crate actix_web;
use failure::Fail;

use actix_web::{ http, middleware, web, App, HttpResponse, HttpServer, error
};
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Fail, Debug)]
enum WebError {
    #[fail(display = "Invalid id '{}'", id)]
    InvalidIdError { id: i32 },
    #[fail(display = "Invalid request, please try again later")]
    RandomInternalError,
}

impl error::ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            WebError::InvalidIdError { .. } => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            WebError::RandomInternalError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bookmark {
    id: i32,
    url: String,
}

#[get("by-id/{id}")]
fn bookmarks_by_id(id: web::Path<(i32)>) -> Result<HttpResponse, WebError> {
    if *id < 10 {
        Ok(HttpResponse::Ok().json(Bookmark {
            id: *id,
            url: "https://blog.x5ff.xyz".into(),
        }))
    }
    else {
        Err(WebError::InvalidIdError { id: *id })
    }
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/bookmarks")
                    .service(bookmarks_by_id)
            )
            .route(
                "/underconstruction",
                web::get().to(|| Result::<HttpResponse, WebError>::Err(WebError::RandomInternalError)),
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
}
