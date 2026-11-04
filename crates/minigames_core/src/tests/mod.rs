//! Unit tests of the crate, one file per module, and the fixtures they share:
//! game layers on a bare Bevy `World`.

use chunkedge::prelude::*;

use crate::game::Game;
use crate::layer::GameLayer;
use crate::phase::{GamePhase, Waiting};
use crate::slug::GameSlug;
use crate::spawn::LayerSpawn;
use crate::transition::{GamePhaseCommands, Next};

pub const BEDWARS: GameSlug = GameSlug::from_static("bedwars");

pub const SPAWN: LayerSpawn = LayerSpawn {
    pos: DVec3::new(0.5, 64.0, 0.5),
    yaw: 180.0,
    pitch: 0.0,
};

pub fn game_layer(world: &mut World, capacity: usize) -> Entity {
    world
        .spawn(GameLayer {
            game: BEDWARS,
            capacity,
        })
        .id()
}

/// The player count of a game in phase `P`, or `None` if the game isn't in
/// `P`.
pub fn players<P: GamePhase>(world: &mut World, game: Entity) -> Option<usize> {
    let mut query = world.query::<Game<P>>();
    query.get(world, game).ok().map(|game| game.player_count())
}

/// The free slots of a waiting game, or `None` if it isn't waiting.
pub fn free_slots(world: &mut World, game: Entity) -> Option<usize> {
    let mut query = world.query::<Game<Waiting>>();
    query.get(world, game).ok().map(|game| game.free_slots())
}

pub fn transition<From: Next<To>, To: GamePhase>(world: &mut World, game: Entity) {
    world.commands().entity(game).transition::<From, To>();
    world.flush();
}

mod enter;
mod game;
mod players;
mod slug;
mod transition;
