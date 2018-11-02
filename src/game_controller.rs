use actix::prelude::*;
use crate::client_controller::*;
use log::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GameController {
    players: HashMap<usize, Addr<ClientController>>,
}

impl GameController {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Actor for GameController {
    type Context = Context<Self>;
}

impl Handler<ClientConnected> for GameController {
    type Result = usize;

    fn handle(&mut self, message: ClientConnected, ctx: &mut Self::Context) -> Self::Result {
        debug!("Incoming client message: {:?}", message);

        // TODO: Come up with a better system for generating player IDs.
        let client_id = self.players.len();

        debug!("Registering new client with ID {}", client_id);
        self.players.insert(client_id, message.addr);

        client_id
    }
}
