#[macro_use]
extern crate actix_web;
mod middlewares;
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, Header};
use middlewares::Claims;
use serde_derive::{Deserialize, Serialize};
use std::env;

const PASSWORD: &str = "swordfish";
pub const TOKEN_SECRET: &str = "0fd2af6f";

#[derive(Debug, Serialize, Deserialize)]
struct Login {
    password: String,
}

#[get("/secret")]
fn authed() -> impl Responder {
    format!("Congrats, you are authenticated")
}

fn login(login: web::Json<Login>) -> HttpResponse {
    // TODO: have a proper security concept
    if &login.password == PASSWORD {
        let claims = Claims {
            user_id: "1".into(),
        };
        encode(&Header::default(), &claims, TOKEN_SECRET.as_ref())
            .map(|token| {
                HttpResponse::Ok()
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .finish()
            })
            .unwrap_or(HttpResponse::InternalServerError().into())
    } else {
        HttpResponse::Unauthorized().into()
    }
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middlewares::JwtLogin)
            .service(authed)
            .service(web::resource("/login").route(web::post().to(login)))
    })
    .bind("127.0.0.1:8081")?
    .run()
}
