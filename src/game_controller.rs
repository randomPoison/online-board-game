use actix::prelude::*;
use crate::client_controller::*;
use crate::game::*;
use futures::Future;
use log::*;
use serde_derive::*;
use std::collections::HashSet;

/// Main actor in charge of managing and updating the game state.
///
/// The game controller owns and manages all state data for the game, updating
/// the game in response to incoming inputs from the clients and then
/// broadcasting updated information in turn.
#[derive(Debug, Default)]
pub struct GameController {
    clients: HashSet<ClientAddr>,

    // Game state objects.
    players: Vec<Player>,
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
        debug!(
            "Client connected: {:#x}",
            crate::default_hash(&message.addr)
        );

        // Track the connected clients so that we can broadcast state updates
        // out to them.
        self.clients.insert(message.addr);

        // Add a player corresponding to the new client.
        //
        // TODO: Handle a new client taking control of an existing player. This is something
        // we'll want to do if a client disconnects and then reconnects, rather than creating
        // a new player each time a new client connects.
        //
        // TODO: Use a better system for setting the initial position of players. Currently
        // we start them at (num_players, 0), which doesn't account for the total size of
        // the grid or give us any control over initial player positions.
        let pos = GridPos {
            x: self.players.len(),
            y: 0,
        };
        self.players.push(Player {
            pos,
            health: Health {
                max: 10,
                current: 10,
            },
        });

        // Broadcast the updated game state to all connected clients.
        for client in &self.clients {
            let send_future = client
                .send(StateUpdate {
                    players: self.players.clone(),
                }).then(|result| {
                    result.expect("Failed to send state update to client");
                    Ok(())
                });

            Arbiter::spawn(send_future);
        }
    }
}

impl Handler<ClientDisconnected> for GameController {
    type Result = ();

    fn handle(&mut self, message: ClientDisconnected, _ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Client disconnected: {:#x}",
            crate::default_hash(&message.addr)
        );
        self.clients.remove(&message.addr);
    }
}

/// Game state update being broadcast to connected clients.
///
/// This message is sent whenever the game's state has changed in order to
/// notify connected clients of the new state.
#[derive(Debug, Message, Serialize)]
pub struct StateUpdate {
    players: Vec<Player>,
}
