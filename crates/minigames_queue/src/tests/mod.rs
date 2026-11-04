//! Unit tests of the crate, one file per module, and the fixtures they share:
//! a bare `App` with the queue, and plain
//! entities as players.

use chunkedge::prelude::*;
use minigames_game::{GameLayer, GameSlug};

use crate::message::QueueMessage;
use crate::plugin::QueuePlugin;

pub const BEDWARS: GameSlug = GameSlug::from_static("bedwars");
pub const SKYWARS: GameSlug = GameSlug::from_static("skywars");

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(QueuePlugin);
    app
}

pub fn game_layer(app: &mut App, game: GameSlug, capacity: usize) -> Entity {
    app.world_mut().spawn(GameLayer { game, capacity }).id()
}

/// Sends a [`QueueMessage::Join`] and runs a tick.
pub fn join(app: &mut App, game: GameSlug, players: &[Entity]) {
    let players = players.to_vec();
    app.world_mut()
        .resource_mut::<Messages<QueueMessage>>()
        .write(QueueMessage::Join { game, players });
    app.update();
}

mod assign;
mod join;
mod squad;
