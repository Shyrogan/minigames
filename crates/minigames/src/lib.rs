//! Mini game server management for ChunkEdge: layers that hold games, and
//! later a player queue and moving queued players into a game with room.
//!
//! Glob import the [`prelude`] next to ChunkEdge's:
//!
//! ```
//! use chunkedge::prelude::*;
//! use minigames::prelude::*;
//!
//! const BEDWARS: GameSlug = GameSlug::from_static("bedwars");
//!
//! fn add_minigames(app: &mut App) {
//!     app.add_plugins(MinigamesPlugin);
//! }
//!
//! fn start_full_games(mut commands: Commands, games: Query<(Entity, Game<Waiting>)>) {
//!     for (layer, game) in &games {
//!         if game.free_slots() == 0 {
//!             commands.entity(layer).transition::<Waiting, Running>();
//!         }
//!     }
//! }
//! ```

use chunkedge::prelude::*;

pub use minigames_core::*;

/// The most used items of `minigames`. Holds only this library's items:
/// glob import [`chunkedge::prelude`] beside it.
///
/// This is usually glob imported:
///
/// ```
/// use minigames::prelude::*;
/// ```
pub mod prelude {
    pub use minigames_core::GamePhaseCommands as _;
    pub use minigames_core::{
        Ending, Game, GameItem, GameLayer, GameLayerPlugin, GamePhase, GamePlayers, GameSlug,
        InGame, InvalidGameSlug, LayerSpawn, Next, Running, Waiting,
    };

    pub use super::MinigamesPlugin;
}

/// Adds mini game management to a ChunkEdge app: game layers.
pub struct MinigamesPlugin;

impl Plugin for MinigamesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameLayerPlugin);
    }
}
