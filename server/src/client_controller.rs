use actix::prelude::*;
use actix_web::ws::{Message as WebsocketMessage, ProtocolError, WebsocketContext};
use crate::game::*;
use crate::game_controller::*;
use futures::Future;
use log::*;
use serde_derive::*;

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
                let message = match serde_json::from_str(&text) {
                    Ok(message) => message,
                    Err(error) => {
                        warn!("Error parsing message from client: {:?}", error);
                        return;
                    }
                };

                match message {
                    ClientCommand::MoveTo { pos } => {
                        let send_future = self.game.send(InputMoveAction {
                            client: ctx.address(),
                            pos,
                        }).then(|result| {
                            result.expect("Failed to send move command to game controller");
                            Ok(())
                        });
                        Arbiter::spawn(send_future);
                    }
                }
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

/// The valid commands the client can send.
#[derive(Debug, Deserialize)]
#[serde(tag = "message")]
enum ClientCommand {
    /// Requests that the player controlled by the client move to the specified position.
    MoveTo { pos: GridPos },
}

/// Address of a [`ClientController`] actor.
///
/// The address is used to uniquely identify connected clients, no other unique
/// identifier is generated. As such, the `ClientAddr` is often sent with
/// messages to the [`GameController`] to identify which client is submitting
/// a command.
///
/// [`ClientController`]: ./struct.ClientController.html
/// [`GameController`]: ../game_controller/struct.GameController.html
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

/// Message sent to the [`GameController`] when a client inputs a move action.
///
/// [`GameController`]: ../game_controller/struct.GameController.html
#[derive(Debug, Message)]
pub struct InputMoveAction {
    pub client: ClientAddr,
    pub pos: GridPos,
}
