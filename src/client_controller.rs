use actix::prelude::*;
use actix_web::ws;
use crate::game_controller::GameController;
use futures::Future;
use log::*;

#[derive(Debug)]
pub struct ClientController {
    game: Addr<GameController>,
}

impl ClientController {
    pub fn new(game: Addr<GameController>) -> Self {
        ClientController { game }
    }
}

impl Actor for ClientController {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ClientController {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),

            ws::Message::Text(text) => {
                let send_future = self
                    .game
                    .send(ClientConnected { addr: ctx.address() });

                Arbiter::spawn(send_future.then(|result| {
                    let client_id = result.expect("Failed to send player connected message");
                    debug!("New client was assigned ID {}", client_id);

                    // TODO: Somehow save the client ID in the client controller?

                    Ok(())
                }));
            }

            ws::Message::Binary(bin) => unimplemented!("Received binary frame: {:?}", bin),

            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct ClientConnected {
    pub addr: Addr<ClientController>,
}

impl Message for ClientConnected {
    type Result = usize;
}
