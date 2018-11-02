extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate log;
extern crate log4rs;

use actix_web::actix::*;
use actix_web::fs::*;
use actix_web::middleware::*;
use actix_web::{server, ws, App, HttpRequest, Result};
use futures::future::Future;
use log::*;
use std::collections::HashMap;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();

    let system = System::new("online-board-game");

    let game_addr = GameActor::new().start();

    server::new(move || {
        let game_addr = game_addr.clone();
        App::new()
            .middleware(Logger::default())
            .resource("/ws/", move |resource| {
                let game_addr = game_addr.clone();
                resource.f(move |req| ws::start(req, WebsocketActor::new(game_addr.clone())))
            }).resource("/", |resource| resource.f(index))
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
    players: HashMap<usize, Addr<WebsocketActor>>,
}

impl GameActor {
    fn new() -> Self {
        Default::default()
    }
}

impl Actor for GameActor {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for GameActor {
    type Result = ();

    fn handle(&mut self, message: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        debug!("Incoming client message: {:?}", message);

        match message {
            ClientMessage::PlayerConnected(client_addr) => {
                // TODO: Come up with a better system for generating player IDs.
                let client_id = self.players.len();
            }
        }
    }
}

#[derive(Debug, Message)]
enum ClientMessage {
    PlayerConnected(Addr<WebsocketActor>),
}

#[derive(Debug)]
struct WebsocketActor {
    game: Addr<GameActor>,
}

impl WebsocketActor {
    fn new(game: Addr<GameActor>) -> Self {
        WebsocketActor { game }
    }
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WebsocketActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),

            ws::Message::Text(text) => {
                let send_future = self
                    .game
                    .send(ClientMessage::PlayerConnected(ctx.address()));

                Arbiter::spawn(send_future.then(|result| {
                    result.expect("Failed to send player connected message");
                    Ok(())
                }));
            }

            ws::Message::Binary(bin) => println!("Received binary frame: {:?}", bin),

            _ => (),
        }
    }
}
