use serde_derive::*;

/// Grid position of an entity in the world.
///
/// Grid position starts at the lower-left of the world and increases moving
/// up and to the right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPos {
    pub x: usize,
    pub y: usize,
}

/// Health information for an entity that can be damaged and destroyed.
///
/// Any entity in the world that can be damaged tracks that damage using the
/// `Health` component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Health {
    pub max: usize,
    pub current: usize,
}

/// Entity representing a player in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Player {
    pub pos: GridPos,
    pub health: Health,
}
