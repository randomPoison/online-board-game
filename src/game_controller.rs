use actix::prelude::*;
use crate::client_controller::*;
use log::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct GameController {
    clients: HashSet<ClientAddr>,
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
    type Result = ();

    fn handle(&mut self, message: ClientConnected, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Client connected: {:#x}", crate::default_hash(&message.addr));
        self.clients.insert(message.addr);
    }
}

impl Handler<ClientDisconnected> for GameController {
    type Result = ();

    fn handle(&mut self, message: ClientDisconnected, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Client disconnected: {:#x}", crate::default_hash(&message.addr));
        self.clients.remove(&message.addr);
    }
}
