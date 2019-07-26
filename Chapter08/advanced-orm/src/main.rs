#[macro_use]
extern crate diesel;
mod models;
mod schema;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use std::env;

use crate::schema::{date, julianday};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use futures::Future;
use serde_derive::{Deserialize, Serialize};
// Helpers
const SQLITE_DB_URL: &str = "db/bookmarks.sqlite";

#[derive(Debug, Serialize, Deserialize)]
struct WebBookmark {
    url: String,
    comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebBookmarkResponse {
    id: String,
    added: String,
    url: String,
    comment: Option<String>,
}

fn connect(db_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&SQLITE_DB_URL).expect(&format!("Error connecting to {}", db_url))
}

// Handlers

fn bookmarks_as_julian_by_date(
    at: web::Path<(String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;
        let conn = connect(&SQLITE_DB_URL);
        bookmarks
            .select((id, url, julianday(added)))
            .filter(date(added).eq(at.as_str()))
            .load::<models::JulianBookmark>(&conn)
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
        use self::schema::comments::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        let new_id = format!("{}", uuid::Uuid::new_v4());
        let now = Utc::now().to_rfc3339();
        let new_bookmark = models::NewBookmark {
            id: &new_id,
            url: &bookmark.url,
            added: &now,
        };

        if let Some(comment_) = &bookmark.comment {
            let new_comment_id = format!("{}", uuid::Uuid::new_v4());
            let new_comment = models::NewComment {
                comment_id: &new_comment_id,
                bookmark_id: &new_id,
                comment: &comment_,
            };
            let _ = diesel::insert_into(comments)
                .values(&new_comment)
                .execute(&conn);
        }

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
        use self::schema::comments::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        diesel::delete(bookmarks.filter(id.eq(req_id.as_str())))
            .execute(&conn)
            .and_then(|_| {
                diesel::delete(comments.filter(bookmark_id.eq(req_id.as_str()))).execute(&conn)
            })
    })
    .then(|res| match res {
        Ok(obj) => Ok(HttpResponse::Ok().json(obj)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn all_bookmarks() -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        use self::schema::bookmarks::dsl::*;
        use self::schema::comments::dsl::*;

        let conn = connect(&SQLITE_DB_URL);
        bookmarks
            .left_outer_join(comments)
            .load::<(models::Bookmark, Option<models::Comment>)>(&conn)
            .map(
                |bookmarks_: Vec<(models::Bookmark, Option<models::Comment>)>| {
                    let responses: Vec<WebBookmarkResponse> = bookmarks_
                        .into_iter()
                        .map(|(b, c)| WebBookmarkResponse {
                            id: b.id,
                            url: b.url,
                            added: b.added,
                            comment: c.map(|c| c.comment),
                        })
                        .collect();
                    responses
                },
            )
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
                    .service(web::resource("/all").route(web::get().to_async(all_bookmarks)))
                    .service(
                        web::resource("added_on/{at}/julian")
                            .route(web::get().to_async(bookmarks_as_julian_by_date)),
                    )
                    .service(
                        web::resource("/")
                            .data(web::JsonConfig::default())
                            .route(web::post().to_async(bookmarks_add)),
                    )
                    .service(
                        web::resource("by-id/{id}").route(web::delete().to_async(bookmarks_delete)),
                    ),
            ),
        )
    })
    .bind("127.0.0.1:8081")?
    .run()
}
