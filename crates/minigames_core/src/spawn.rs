//! Where players arrive in a layer.

use chunkedge::prelude::*;

/// Where players arrive when they enter a layer, and where they respawn while
/// in it. Sits on the layer entity, next to its `LayerBundle`; any layer can
/// have one, game or not.
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct LayerSpawn {
    pub pos: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}
