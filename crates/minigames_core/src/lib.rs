//! Shared types of the `minigames` crates.
//!
//! A *game* is one match of a mini game, held by a world: a ChunkEdge layer
//! entity marked with [`GameWorld`]. The kind of mini game it plays, such as
//! `bedwars`, is named by a [`GameSlug`]. Players in a game carry [`InGame`],
//! and the world lists them in [`GamePlayers`]. A game is in one phase at a
//! time ([`Waiting`], [`Running`] or [`Ending`]), and systems read the games
//! of a phase through the [`Game`] query: `Query<Game<Waiting>>`.

mod game;
mod phase;
mod players;
mod slug;
#[cfg(test)]
mod test_support;
mod transition;
mod world;

pub use game::{Game, GameItem};
pub use phase::{Ending, GamePhase, Running, Waiting};
pub use players::{GamePlayers, InGame};
pub use slug::{GameSlug, InvalidGameSlug};
pub use transition::{GamePhaseCommands, Next};
pub use world::GameWorld;
