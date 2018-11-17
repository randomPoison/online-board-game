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
    clients: HashMap<ClientAddr, PlayerId>,

    /// List of players that are not currently assigned to a connected client.
    ///
    /// When a new client connects to the game, they will be given control of
    /// an existing player before a new player is created.
    unassigned_players: Vec<PlayerId>,

    /// State data for all the players in the game.
    players: HashMap<PlayerId, Player>,
}

impl GameController {
    pub fn new() -> Self {
        Default::default()
    }

    /// Broadcasts a message to all connected clients.
    fn broadcast<M>(&self, message: M)
    where
        M: 'static + Message + Clone + Send,
        M::Result: Send,
        ClientController: Handler<M>,
    {
        for client in self.clients.keys() {
            let send_future = client.send(message.clone()).then(|result| {
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
    type Result = WorldState;

    fn handle(&mut self, message: ClientConnected, _ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Client connected: {:#x}",
            crate::default_hash(&message.addr)
        );

        // Check if there are any players that are not currently being controlled by
        // a client. If so, assign the new client to control one of the available
        // players. Otherwise, create a new player for the client.
        //
        // TODO: Set a cap on how many players can be in the game at a time.
        let id = match self.unassigned_players.pop() {
            Some(id) => {
                info!("Assigning existing player {:?} to connected client", id,);
                id
            }

            None => {
                let id = PlayerId(self.players.len());
                info!("Creating player {:?} for connected client", id);

                // Add a player corresponding to the new client.
                //
                // TODO: Use a better system for setting the initial position of players. Currently
                // we start them at (num_players, 0), which doesn't account for the total size of
                // the grid or give us any control over initial player positions.
                let player = Player {
                    pos: GridPos {
                        x: self.players.len(),
                        y: 0,
                    },

                    health: Health {
                        max: 10,
                        current: 10,
                    },

                    pending_turn: Default::default(),
                };

                // Add the new player to the board.
                self.players.insert(id, player.clone());

                // Notify existing clients that a new player was added to the board.
                self.broadcast(Update::PlayerAdded {
                    id: id,
                    data: player,
                });

                id
            }
        };

        // Add the new client, tracking which player it controls.
        self.clients.insert(message.addr, id);

        // Return the current world state to the new client.
        WorldState {
            players: self.players.clone(),
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
        let player_index = match self.clients.remove(&message.addr) {
            Some(index) => index,
            None => {
                warn!("Disconnected client was not assigned to a player");
                return;
            }
        };
        self.unassigned_players.push(player_index);
    }
}

impl Handler<InputMoveAction> for GameController {
    type Result = ();

    fn handle(&mut self, message: InputMoveAction, _ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Moving client {:#x} to {:?}",
            crate::default_hash(&message.client),
            message.pos,
        );

        // Update the move action for the player controlled by the client that sent
        // the message.
        let &id = self
            .clients
            .get(&message.client)
            .expect("No such client found");
        self.players.get_mut(&id).unwrap().pending_turn.movement = Some(message.pos);

        // Broadcast the updated game state to all connected clients.
        self.broadcast(Update::SetMovement {
            id,
            pos: message.pos,
        });
    }
}

/// Unique identifier for a player in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(usize);

/// Game state update being broadcast to connected clients.
///
/// This message is sent whenever the game's state has changed in order to
/// notify connected clients of the new state.
#[derive(Debug, Clone, Serialize)]
pub struct WorldState {
    players: HashMap<PlayerId, Player>,
}

impl<A, M> actix::dev::MessageResponse<A, M> for WorldState
where
    A: Actor,
    M: Message<Result = WorldState>,
{
    fn handle<R: actix::dev::ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

/// An update to the game state.
///
/// Updates to the game state are delta-encoded, sending only the changes
/// resulting from each action and user input. This minimizes the amount of
/// data that needs to be sent to the clients when the game state changes.
#[derive(Debug, Clone, Message, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Update {
    /// A new player was added to the board.
    PlayerAdded { id: PlayerId, data: Player },

    /// A player set their move action for the current turn.
    SetMovement { id: PlayerId, pos: GridPos },
}
