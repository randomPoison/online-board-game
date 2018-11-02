use actix::prelude::*;
use actix_web::ws;
use crate::game_controller::GameController;
use futures::Future;
use log::*;

#[derive(Debug)]
pub struct ClientController {
    game: Addr<GameController>,
    client_id: Option<usize>,
}

impl ClientController {
    pub fn new(game: Addr<GameController>) -> Self {
        ClientController {
            game,
            client_id: None,
        }
    }
}

impl Actor for ClientController {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ClientController {
    fn handle(&mut self, message: ws::Message, ctx: &mut Self::Context) {
        let self_addr = ctx.address();
        match message {
            ws::Message::Text(text) => {
                if self.client_id.is_some() {
                    unimplemented!("TODO: Implement stuff for after the client connects");
                }

                let send_future = self
                    .game
                    .send(ClientConnected {
                        addr: ctx.address(),
                    }).and_then(move |client_id| {
                        debug!("New client was assigned ID {}", client_id);

                        // TODO: Somehow save the client ID in the client controller?
                        self_addr.send(ClientRegistered { client_id })
                    }).then(|result| {
                        result.expect("Error sending client ID to client controller");
                        Ok(())
                    });

                Arbiter::spawn(send_future);
            }

            _ => unimplemented!("Unsupported websocket message: {:?}", message),
        }
    }
}

impl Handler<ClientRegistered> for ClientController {
    type Result = ();

    fn handle(&mut self, message: ClientRegistered, _ctx: &mut Self::Context) -> Self::Result {
        assert!(
            self.client_id.is_none(),
            "Received {:?} but already had client ID {:?}",
            message,
            self.client_id
        );
        self.client_id = Some(message.client_id);
    }
}

#[derive(Debug)]
pub struct ClientConnected {
    pub addr: Addr<ClientController>,
}

impl Message for ClientConnected {
    type Result = usize;
}

#[derive(Debug, Message)]
struct ClientRegistered {
    client_id: usize,
}
