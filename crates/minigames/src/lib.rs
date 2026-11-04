//! Mini game server management for ChunkEdge: layers that hold games, and
//! queues that move players into a game with room.
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
//! /// Queues a squad; the queue moves it into a waiting game with room.
//! fn queue_squad(mut joins: MessageWriter<QueueMessage>, squad: Vec<Entity>) {
//!     joins.write(QueueMessage::Join { game: BEDWARS, players: squad });
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

pub use minigames_game::*;
pub use minigames_queue as queue;

/// The most used items of `minigames`. Holds only this library's items:
/// glob import [`chunkedge::prelude`] beside it.
///
/// This is usually glob imported:
///
/// ```
/// use minigames::prelude::*;
/// ```
pub mod prelude {
    pub use minigames_game::GamePhaseCommands as _;
    pub use minigames_game::{
        Ending, Game, GameItem, GameLayer, GameLayerPlugin, GamePhase, GamePlayers, GameSlug,
        InvalidGameSlug, Next, Playing, Running, Waiting,
    };
    pub use minigames_queue::{QueueMessage, QueuePlugin, Queued, Squad, Squadmates};

    pub use super::MinigamesPlugin;
}

/// Adds mini game management to a ChunkEdge app: game layers and queues.
pub struct MinigamesPlugin;

impl Plugin for MinigamesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameLayerPlugin, minigames_queue::QueuePlugin));
    }
}
