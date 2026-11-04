//! The plugin that runs the queues.

use chunkedge::prelude::*;

use crate::assign::assign_players;
use crate::entry::{QueueTickets, despawn_empty_entry};
use crate::join::{JoinQueue, join_queues};

/// Handles [`JoinQueue`], then moves queued parties into waiting games with
/// room, in `Update`.
pub struct QueuePlugin;

impl Plugin for QueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueueTickets>()
            .add_message::<JoinQueue>()
            .add_observer(despawn_empty_entry)
            .add_systems(Update, (join_queues, assign_players).chain());
    }
}
