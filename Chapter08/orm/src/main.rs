#[macro_use]
extern crate diesel;
mod models;
mod schema;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use futures::Future;
use models::{Bookmark, NewBookmark};
use serde_derive::{Deserialize, Serialize};

// Helpers
const SQLITE_DB_URL: &str = "db/bookmarks.sqlite";

#[derive(Debug, Serialize, Deserialize)]
struct WebBookmark {
    url: String,
}

fn connect(db_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&SQLITE_DB_URL).expect(&format!("Error connecting to {}", db_url))
}

// Handlers
fn bookmarks_by_id(req_id: web::Path<(String)>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        bookmarks
            .filter(id.eq(req_id.as_str()))
            .limit(1)
            .load::<Bookmark>(&conn)
    })
    .then(|res| match res {
        Ok(obj) => Ok(HttpResponse::Ok().json(obj)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn all_bookmarks() -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        bookmarks.load::<Bookmark>(&conn)
    })
    .then(|res| match res {
        Ok(obj) => Ok(HttpResponse::Ok().json(obj)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn bookmarks_add(
    bookmark: web::Json<WebBookmark>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        let new_id = format!("{}", uuid::Uuid::new_v4());
        let new_bookmark = NewBookmark {
            id: &new_id,
            url: &bookmark.url,
        };
        diesel::insert_into(bookmarks)
            .values(&new_bookmark)
            .execute(&conn)
            .map(|_| new_id)
    })
    .then(|res| match res {
        Ok(obj) => Ok(HttpResponse::Ok().json(obj)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn bookmarks_delete(
    req_id: web::Path<(String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        diesel::delete(bookmarks.filter(id.eq(req_id.as_str()))).execute(&conn)
    })
    .then(|res| match res {
        Ok(obj) => Ok(HttpResponse::Ok().json(obj)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api").service(
                web::scope("/bookmarks")
                    .service(web::resource("/all")
                        .route(web::get().to_async(all_bookmarks))
                        )
                    .service(
                        web::resource("by-id/{id}")
                        .route(web::get().to_async(bookmarks_by_id))
                        .route(web::delete().to_async(bookmarks_delete))
                    )
                    .service(
                        web::resource("/")
                            .data(web::JsonConfig::default())
                            .route(web::post().to_async(bookmarks_add)),
                    )
            ),
        )
    })
    .bind("127.0.0.1:8081")?
    .run()
}
