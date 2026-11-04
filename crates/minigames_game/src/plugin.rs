//! The plugin that makes game layers work.

use chunkedge::prelude::*;

use crate::enter::enter_game;

/// Moves players into a game layer when they enter its game (gain
/// [`Playing`](crate::Playing)).
pub struct GameLayerPlugin;

impl Plugin for GameLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(enter_game);
    }
}
