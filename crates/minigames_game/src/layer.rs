//! The marker that turns a ChunkEdge layer into a game layer.

use chunkedge::prelude::*;

use crate::phase::Waiting;
use crate::slug::GameSlug;

/// Marks a layer entity (a [`LayerBundle`]) as the layer of one game of
/// `game`, holding at most `capacity` players. A new game starts in
/// [`Waiting`].
///
/// Nothing enforces `capacity`: check the free slots of a
/// [`Game<Waiting>`](crate::Game) before adding a player.
#[derive(Component, Clone, Debug)]
#[require(Waiting)]
pub struct GameLayer {
    pub game: GameSlug,
    pub capacity: usize,
}
