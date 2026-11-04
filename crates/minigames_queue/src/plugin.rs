//! The plugin that runs the queues.

use chunkedge::prelude::*;

use crate::assign::assign_players;
use crate::join::join_queues;
use crate::message::QueueMessage;
use crate::squad::{QueueTickets, despawn_empty_squad};

/// Handles [`QueueMessage`], then moves queued squads into waiting games with
/// room, in `Update`.
pub struct QueuePlugin;

impl Plugin for QueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueueTickets>()
            .add_message::<QueueMessage>()
            .add_observer(despawn_empty_squad)
            .add_systems(Update, (join_queues, assign_players).chain());
    }
}
