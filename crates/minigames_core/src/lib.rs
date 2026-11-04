//! Shared types of the `minigames` crates.
//!
//! A *game* is one match of a mini game, held by a ChunkEdge layer entity
//! marked with [`GameLayer`]. The kind of mini game it plays, such as
//! `bedwars`, is named by a [`GameSlug`]. Players in a game carry [`Playing`],
//! and the layer lists them in [`GamePlayers`]; inserting `Playing` moves the
//! player into the layer ([`GameLayerPlugin`]). A game
//! is in one phase at a time ([`Waiting`], [`Running`] or [`Ending`]), and
//! systems read the games of a phase through the [`Game`] query:
//! `Query<Game<Waiting>>`.

mod enter;
mod game;
mod layer;
mod phase;
mod players;
mod plugin;
mod slug;
#[cfg(test)]
mod tests;
mod transition;

pub use game::{Game, GameItem};
pub use layer::GameLayer;
pub use phase::{Ending, GamePhase, Running, Waiting};
pub use players::{GamePlayers, Playing};
pub use plugin::GameLayerPlugin;
pub use slug::{GameSlug, InvalidGameSlug};
pub use transition::{GamePhaseCommands, Next};
