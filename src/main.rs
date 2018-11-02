extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate log;
extern crate log4rs;

use actix_web::actix::*;
use actix_web::fs::*;
use actix_web::middleware::*;
use actix_web::{server, ws, App, HttpRequest, Result};
use crate::client_controller::ClientController;
use crate::game_controller::GameController;

mod client_controller;
mod game_controller;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

    let system = System::new("online-board-game");

    let game_addr = GameController::new().start();

    server::new(move || {
        let game_addr = game_addr.clone();
        App::new()
            // Capture log output coming from actix_web.
            .middleware(Logger::default())

            // Listen for websocket connections from clients and spawn a new `ClientController`
            // for each new connection.
            .resource("/ws/", move |resource| {
                let game_addr = game_addr.clone();
                resource.f(move |req| ws::start(req, ClientController::new(game_addr.clone())))
            })

            // Serve index.html if someone browses to the root.
            .resource("/", |resource| resource.f(index))

            // Server any other file requests out of the static files directory.
            .handler(
                "/",
                StaticFiles::new("./static").unwrap().show_files_listing(),
            )
    }).bind("127.0.0.1:8088")
    .unwrap()
    .start();

    system.run();
}

fn index(_req: &HttpRequest) -> Result<NamedFile> {
    NamedFile::open("static/index.html").map_err(Into::into)
}
