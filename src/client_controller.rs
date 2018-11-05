use actix::prelude::*;
use actix_web::ws::{Message as WebsocketMessage, ProtocolError, WebsocketContext};
use crate::game_controller::*;
use futures::Future;
use log::*;

/// Actor in charge of communicating with a connected client.
///
/// The client controller doesn't control or interact with the game state
/// directly, rather it passes incoming inputs from the client to the
/// [`GameController`], and then forwards state updates from the game controller
/// back to the client.
///
/// On start it automatically notifes the game controller that a new client
/// has connected. When the client disconnects it will stop itself and notify
/// the game controller that the client has disconnected.
///
/// [`GameController`]: ../game_controller/struct.GameController.html
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

impl Handler<StateUpdate> for ClientController {
    type Result = ();

    fn handle(&mut self, message: StateUpdate, ctx: &mut Self::Context) -> Self::Result {
        let json = serde_json::to_string(&message).expect("Failed to serialize state update");
        ctx.text(json);
    }
}

/// Address of a [`ClientController`] actor.
///
/// [`ClientController`]: ./struct.ClientController.html
pub type ClientAddr = Addr<ClientController>;

/// Message sent to the [`GameController`] when a client first connects.
///
/// [`GameController`]: ../game_controller/struct.GameController.html
#[derive(Debug, Message)]
pub struct ClientConnected {
    pub addr: Addr<ClientController>,
}

/// Message sent to the [`GameController`] when a client disconnects.
///
/// [`GameController`]: ../game_controller/struct.GameController.html
#[derive(Debug, Message)]
pub struct ClientDisconnected {
    pub addr: Addr<ClientController>,
}
