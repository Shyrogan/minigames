//! Unit tests of the crate, one file per module, and the fixtures they share:
//! a bare `App` with the queues of two mini games, and plain entities as
//! players.

use chunkedge::prelude::*;
use minigames_game::{GameLayer, GameSlug, MiniGame};

use crate::message::QueueMessage;
use crate::plugin::{GameQueuePlugin, QueuePlugin};
use crate::status::Free;

pub struct Bedwars;

impl MiniGame for Bedwars {
    const SLUG: GameSlug = GameSlug::from_static("bedwars");
}

pub struct Skywars;

impl MiniGame for Skywars {
    const SLUG: GameSlug = GameSlug::from_static("skywars");
}

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        QueuePlugin,
        GameQueuePlugin::<Bedwars>::default(),
        GameQueuePlugin::<Skywars>::default(),
    ));
    app
}

pub fn game_layer<G: MiniGame>(app: &mut App, capacity: usize) -> Entity {
    app.world_mut().spawn(GameLayer::new::<G>(capacity)).id()
}

/// A plain entity standing for a client: it starts free.
pub fn player(app: &mut App) -> Entity {
    app.world_mut().spawn(Free).id()
}

pub fn has<C: Component>(app: &App, entity: Entity) -> bool {
    app.world().get::<C>(entity).is_some()
}

/// Sends a [`QueueMessage`] and runs a tick.
pub fn send(app: &mut App, message: QueueMessage) {
    app.world_mut()
        .resource_mut::<Messages<QueueMessage>>()
        .write(message);
    app.update();
}

/// Sends a [`QueueMessage::Join`] for `G` and runs a tick.
pub fn join<G: MiniGame>(app: &mut App, players: &[Entity]) {
    let players = players.to_vec();
    send(
        app,
        QueueMessage::Join {
            game: G::SLUG,
            players,
        },
    );
}

mod assign;
mod message;
mod squad;
mod status;
