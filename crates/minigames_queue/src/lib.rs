//! Queues for mini games.
//!
//! Every player has one queue status: [`Free`], [`Queued`] or [`Busy`]. A
//! [`QueueMessage::Join`] puts a squad (1..n free players) in line for a
//! [`MiniGame`](minigames_game::MiniGame): it becomes a [`Squad`] whose
//! [`Squadmates`] are queued. Every tick, squads move, first come first
//! served and never split, into waiting games of their mini game with enough
//! free slots: they gain [`Playing`](minigames_game::Playing), which moves
//! them into the game layer and makes them busy.
//!
//! A squad is dissolved, all its players free, when one of them leaves with
//! [`QueueMessage::Leave`], despawns, or is taken into a game by other means.

mod assign;
mod message;
mod plugin;
mod squad;
mod status;
#[cfg(test)]
mod tests;

pub use message::QueueMessage;
pub use plugin::{GameQueuePlugin, QueuePlugin};
pub use squad::{Queued, Squad, Squadmates};
pub use status::{Busy, Free};
