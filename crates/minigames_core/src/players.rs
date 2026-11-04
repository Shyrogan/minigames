//! Which players are in which game layer, as a Bevy relationship.

use chunkedge::prelude::*;

/// The [`GameLayer`](crate::GameLayer) a player is in.
///
/// Inserting it moves the player into that layer, at its `LayerSpawn` (see
/// [`GameLayerPlugin`](crate::GameLayerPlugin)). Removing it leaves the
/// player where it is.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[relationship(relationship_target = GamePlayers)]
pub struct InGame(pub Entity);

/// The players in a game layer, kept in sync by Bevy from [`InGame`]. Absent
/// while the layer is empty.
///
/// Despawning the layer removes `InGame` from its players but keeps them.
#[derive(Component, Debug)]
#[relationship_target(relationship = InGame)]
pub struct GamePlayers(Vec<Entity>);

impl GamePlayers {
    pub fn players(&self) -> &[Entity] {
        &self.0
    }
}
