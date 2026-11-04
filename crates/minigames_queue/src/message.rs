//! The messages the queue reads.

use chunkedge::prelude::*;
use minigames_game::GameSlug;

/// A request to the queue.
#[derive(Message, Clone, Debug)]
pub enum QueueMessage {
    /// Puts `players` in line for `game`, as one squad: they enter the same
    /// game together. Players queued elsewhere switch to this queue. Ignored
    /// if the squad is empty or one of its players is in a game already.
    Join {
        game: GameSlug,
        players: Vec<Entity>,
    },
}
