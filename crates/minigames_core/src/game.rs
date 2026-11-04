//! Reading the games of one phase: `Query<Game<Waiting>>`.

use chunkedge::prelude::bevy_ecs::query::QueryData;
use chunkedge::prelude::*;

use crate::layer::GameLayer;
use crate::phase::{GamePhase, Waiting};
use crate::players::GamePlayers;

/// A layer's game in phase `P`: what it plays and its players. Only matches
/// layers in `P`, so `Query<Game<Waiting>>` only sees games that take players.
#[derive(QueryData)]
pub struct Game<P: GamePhase> {
    pub layer: &'static GameLayer,
    pub players: Option<&'static GamePlayers>,
    /// Restricts the query to phase `P`. A filter can't be a field of a
    /// `QueryData`, and reading the zero-sized marker costs nothing.
    phase: &'static P,
}

impl<P: GamePhase> GameItem<'_, '_, P> {
    pub fn player_count(&self) -> usize {
        self.players.map_or(0, |players| players.players().len())
    }
}

impl GameItem<'_, '_, Waiting> {
    /// How many more players the game takes. Only a waiting game takes
    /// players, so only it has free slots.
    pub fn free_slots(&self) -> usize {
        self.layer.capacity.saturating_sub(self.player_count())
    }
}
