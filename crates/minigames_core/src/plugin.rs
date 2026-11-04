//! The plugin that makes game layers work.

use chunkedge::prelude::*;

use crate::enter::enter_game;

/// Moves players into a game layer when they enter its game (gain
/// [`InGame`](crate::InGame)).
pub struct GameLayerPlugin;

impl Plugin for GameLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(enter_game);
    }
}
