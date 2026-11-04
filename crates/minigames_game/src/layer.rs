//! The marker that turns a ChunkEdge layer into a game layer.

use chunkedge::prelude::*;

use crate::mini_game::MiniGame;
use crate::phase::Waiting;
use crate::slug::GameSlug;

/// Marks a layer entity (a [`LayerBundle`]) as the layer of one game of a
/// [`MiniGame`], holding at most `capacity` players. A new game starts in
/// [`Waiting`].
///
/// Nothing enforces `capacity`: check the free slots of a
/// [`Game<Waiting>`](crate::Game) before adding a player.
#[derive(Component, Clone, Debug)]
#[require(Waiting)]
pub struct GameLayer {
    game: GameSlug,
    pub capacity: usize,
}

impl GameLayer {
    /// The layer of a game of `G`.
    pub fn new<G: MiniGame>(capacity: usize) -> Self {
        Self {
            game: G::SLUG,
            capacity,
        }
    }

    /// The slug of the mini game played here.
    pub fn game(&self) -> &GameSlug {
        &self.game
    }

    /// Whether the game played here is a game of `G`.
    pub fn is<G: MiniGame>(&self) -> bool {
        self.game == G::SLUG
    }
}
