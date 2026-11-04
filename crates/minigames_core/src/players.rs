//! Which players are in which game layer, as a Bevy relationship.

use chunkedge::prelude::*;

/// The [`GameLayer`](crate::GameLayer) a player is in.
///
/// Inserting it moves the player into that layer (see
/// [`GameLayerPlugin`](crate::GameLayerPlugin)). Removing it leaves the
/// player where it is.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[relationship(relationship_target = GamePlayers)]
pub struct Playing(pub Entity);

/// The players in a game layer, kept in sync by Bevy from [`Playing`]. Absent
/// while the layer is empty.
///
/// Despawning the layer removes `Playing` from its players but keeps them.
#[derive(Component, Debug)]
#[relationship_target(relationship = Playing)]
pub struct GamePlayers(Vec<Entity>);

impl GamePlayers {
    pub fn players(&self) -> &[Entity] {
        &self.0
    }
}
