//! Queues for mini games.
//!
//! A [`QueueMessage::Join`] puts a squad (1..n players) in line for a game, named
//! by its [`GameSlug`](minigames_core::GameSlug): it becomes a
//! [`Squad`] whose [`Squadmates`] carry [`Queued`]. Every tick, squads
//! move, first come first served and never split, into waiting games
//! of their game with enough free slots: they gain
//! [`Playing`](minigames_core::Playing), which moves them into the game layer.
//!
//! A player leaves the queue by losing `Queued`, or by despawning.

mod assign;
mod join;
mod message;
mod plugin;
mod squad;
#[cfg(test)]
mod tests;

pub use message::QueueMessage;
pub use plugin::QueuePlugin;
pub use squad::{Queued, Squad, Squadmates};
