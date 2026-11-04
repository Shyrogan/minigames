//! Queues for mini games.
//!
//! A [`JoinQueue`] puts a party (one or more players) in line for a game,
//! named by its [`GameSlug`](minigames_core::GameSlug): it becomes a
//! [`QueueEntry`] whose [`EntryMembers`] carry [`Queued`]. Every tick,
//! entries move, first come first served and never split, into waiting games
//! of their game with enough free slots: they gain
//! [`InGame`](minigames_core::InGame), which moves them into the game layer.
//!
//! A player leaves the queue by losing `Queued`, or by despawning.

mod assign;
mod entry;
mod join;
mod plugin;
#[cfg(test)]
mod tests;

pub use entry::{EntryMembers, QueueEntry, Queued};
pub use join::JoinQueue;
pub use plugin::QueuePlugin;
