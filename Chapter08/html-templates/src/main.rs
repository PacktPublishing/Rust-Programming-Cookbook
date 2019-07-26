#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpServer, Responder};
use chrono::prelude::*;
use std::env;
use yarte::Template;

const PLACEHOLDER_IMG: &str =
    "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
CCGEEEKIKvwA6uJYps1NR+4AAAAASUVORK5CYII=";

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexViewModel {
    user: String,
    bookmarks: Vec<BookmarkViewModel>,
}

struct BookmarkViewModel {
    timestamp: Date<Utc>,
    url: String,
    mime: String,
    base64_image: String,
}

#[get("/{name}")]
pub fn index(name: web::Path<(String)>) -> impl Responder {
    let user_name = name.as_str().into();

    if &user_name == "Claus" {
        IndexViewModel {
            user: user_name,
            bookmarks: vec![
                BookmarkViewModel {
                    timestamp: Utc.ymd(2019, 7, 20),
                    url: "https://blog.x5ff.xyz".into(),
                    mime: "image/png".into(),
                    base64_image: std::fs::read_to_string("static/x5ff.xyz.b64")
                        .unwrap_or(PLACEHOLDER_IMG.into()),
                },
                BookmarkViewModel {
                    timestamp: Utc.ymd(2017, 9, 1),
                    url: "https://microsoft.com".into(),
                    mime: "image/png".into(),
                    base64_image: std::fs::read_to_string("static/microsoft.com.b64")
                        .unwrap_or(PLACEHOLDER_IMG.into()),
                },
                BookmarkViewModel {
                    timestamp: Utc.ymd(2019, 2, 2),
                    url: "https://www.packtpub.com/".into(),
                    mime: "image/png".into(),
                    base64_image: std::fs::read_to_string("static/packtpub.com.b64")
                        .unwrap_or(PLACEHOLDER_IMG.into()),
                },
            ],
        }
    } else {
        IndexViewModel {
            user: user_name,
            bookmarks: vec![],
        }
    }
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/bookmarks").service(index))
    })
    .bind("127.0.0.1:8081")?
    .run()
}
