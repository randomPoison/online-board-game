use actix::prelude::*;
use crate::client_controller::*;
use crate::game::*;
use futures::Future;
use log::*;
use serde_derive::*;
use std::collections::HashMap;

/// Main actor in charge of managing and updating the game state.
///
/// The game controller owns and manages all state data for the game, updating
/// the game in response to incoming inputs from the clients and then
/// broadcasting updated information in turn.
#[derive(Debug, Default)]
pub struct GameController {
    /// Maps the client to the index of the player it controls.
    clients: HashMap<ClientAddr, usize>,

    /// State data for all the players in the game.
    players: Vec<Player>,
}

impl GameController {
    pub fn new() -> Self {
        Default::default()
    }

    /// Broadcasts the full game state to all connected clients.
    fn broadcast_game_state(&self) {
        for client in self.clients.keys() {
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
        self.clients.insert(message.addr, self.players.len());

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
            pending_turn: Default::default(),
        });

        // Broadcast the updated game state to all connected clients.
        self.broadcast_game_state();
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

impl Handler<InputMoveAction> for GameController {
    type Result = ();

    fn handle(&mut self, message: InputMoveAction, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Moving client {:#x} to {:?}",
            crate::default_hash(&message.client),
            message.pos,
        );

        // TODO: Lookup the player controlled by the client and update them?
        let &player_index = self
            .clients
            .get(&message.client)
            .expect("No such client found");

        // HACK: Create an explicit scope for the borrow on `self.players` since we also
        // end up borrowing `self` when broadcasting the game state update. This can
        // be fixed once NLLs are stable.
        {
            let player = &mut self.players[player_index];
            player.pending_turn.movement = Some(message.pos);
        }

        // Broadcast the updated game state to all connected clients.
        self.broadcast_game_state();
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
