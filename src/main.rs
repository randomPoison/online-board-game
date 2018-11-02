extern crate actix_web;
extern crate log;
extern crate log4rs;

use actix_web::actix::*;
use actix_web::fs::*;
use actix_web::{server, ws, App, HttpRequest, Result};
use actix_web::middleware::*;
use std::collections::HashMap;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

    let system = System::new("online-board-game");

    let game_addr = GameActor::new().start();

    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/ws/", |r| r.f(|req| ws::start(req, WebsocketActor)))
            .resource("/", |r| r.f(index))
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

#[derive(Debug, Default)]
struct GameActor {
    players: HashMap<usize, ()>,
}

impl GameActor {
    fn new() -> Self {
        Default::default()
    }
}

impl Actor for GameActor {
    type Context = Context<Self>;
}

enum ClientMessage {
    PlayerJoined(usize),
}

struct WebsocketActor;

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WebsocketActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => println!("Received text frame: {:?}", text),
            ws::Message::Binary(bin) => println!("Received binary frame: {:?}", bin),
            _ => (),
        }
    }
}
