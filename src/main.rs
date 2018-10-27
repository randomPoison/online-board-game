extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}
