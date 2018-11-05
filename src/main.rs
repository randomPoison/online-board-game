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

/// Hashes a value using [`DefaultHasher`].
///
/// This is a helper method used primarily to generate hash values from an
/// [`Addr`]. We use the address of an actor as a unique identifier in a
/// number of cases (e.g. connected clients are identified solely by their
/// [`Addr`]), but the default debug log for those addresses don't include
/// useful identifying information. Hashing the address provides a unique
/// value that will be the same for all [`Addr`] objects pointing to the same
/// actor.
///
/// At some point it may be worth it to provide this functionality in a more
/// "automatic" way, either by providing a wrapper type for [`Addr`] that
/// handles this automatically, or by changing the [`Debug`] impl for [`Addr`]
/// directly.
///
/// [`DefaultHasher`]: https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html
/// [`Addr`]: https://docs.rs/actix/0/actix/struct.Addr.html
/// [`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
pub(crate) fn default_hash<H>(value: &H) -> u64 where H: std::hash::Hash {
    use std::hash::Hasher;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
