use actix::prelude::*;
use actix_web::ws::{Message as WebsocketMessage, ProtocolError, WebsocketContext};
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
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let send_future = self
            .game
            .send(ClientConnected {
                addr: ctx.address(),
            }).then(|result| {
                result.expect("Error sending client ID to client controller");
                Ok(())
            });

        Arbiter::spawn(send_future);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let send_future = self
            .game
            .send(ClientDisconnected {
                addr: ctx.address(),
            }).then(|result| {
                result.expect("Error sending closed message to game");
                Ok(())
            });
        Arbiter::spawn(send_future);
    }
}

impl StreamHandler<WebsocketMessage, ProtocolError> for ClientController {
    fn handle(&mut self, message: WebsocketMessage, ctx: &mut Self::Context) {
        match message {
            WebsocketMessage::Text(text) => {
                unimplemented!("TODO: Handle incoming messages: {:?}", text);
            }

            WebsocketMessage::Close(reason) => {
                debug!("Client disconnected: {:?}", reason);
                ctx.stop();
            }

            _ => unimplemented!("Unsupported websocket message: {:?}", message),
        }
    }
}

/// Address of a [`ClientController`] actor.
///
/// [`ClientController`]: ./struct.ClientController.html
pub type ClientAddr = Addr<ClientController>;

#[derive(Debug, Message)]
pub struct ClientConnected {
    pub addr: Addr<ClientController>,
}

#[derive(Debug, Message)]
pub struct ClientDisconnected {
    pub addr: Addr<ClientController>,
}
