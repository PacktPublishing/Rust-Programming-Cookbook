#![feature(await_macro, async_await)]

use tokio::await;
use tokio::prelude::*;
use hyper::{Client, StatusCode};
use std::time::Duration;
use std::str;

async fn response_code(url: &str) -> StatusCode {
    let uri = url.parse().unwrap();
    let client = Client::new();
    let response = await!(client.get(uri)
                .timeout(Duration::from_secs(30))).unwrap();
    response.status()
}

fn main() {
    tokio::run_async(async {
        let status = await!(response_code("http://blog.x5ff.xyz"));
        println!("Got the status: {}", status);
    });
}