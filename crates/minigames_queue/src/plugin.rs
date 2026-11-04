//! The plugins that run the queues.

use std::marker::PhantomData;

use chunkedge::prelude::*;
use minigames_game::MiniGame;

use crate::assign::assign_players;
use crate::message::{QueueMessage, Queues, handle_queue_messages};
use crate::squad::{
    QueueTickets, despawn_empty_squad, dissolve_squad, drop_queued, keep_only_queued, spawn_squad,
};
use crate::status::{busy_while_playing, free_after_playing, free_new_clients};

/// Keeps every player's queue status, and handles [`QueueMessage`] in
/// `Update`. Each mini game adds its own queue with [`GameQueuePlugin`].
pub struct QueuePlugin;

impl Plugin for QueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueueTickets>()
            .init_resource::<Queues>()
            .add_message::<QueueMessage>()
            .add_observer(free_new_clients)
            .add_observer(busy_while_playing)
            .add_observer(free_after_playing)
            .configure_sets(
                Update,
                (QueueSystems::Messages, QueueSystems::Assign).chain(),
            )
            .add_systems(Update, handle_queue_messages.in_set(QueueSystems::Messages));
    }
}

/// The queue of the mini game `G`: squads joining it, and moving into waiting
/// games of `G` with room, in `Update`. Needs [`QueuePlugin`].
pub struct GameQueuePlugin<G: MiniGame>(PhantomData<fn() -> G>);

impl<G: MiniGame> Default for GameQueuePlugin<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<G: MiniGame> Plugin for GameQueuePlugin<G> {
    fn build(&self, app: &mut App) {
        let previous = app
            .world_mut()
            .get_resource_or_init::<Queues>()
            .0
            .insert(G::SLUG, spawn_squad::<G>);
        assert!(previous.is_none(), "two mini games are named {}", G::SLUG);
        app.add_observer(keep_only_queued::<G>)
            .add_observer(drop_queued::<G>)
            .add_observer(dissolve_squad::<G>)
            .add_observer(despawn_empty_squad::<G>)
            .add_systems(Update, assign_players::<G>.in_set(QueueSystems::Assign));
    }
}

/// Messages first, so squads that join can be placed in the same tick.
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
enum QueueSystems {
    Messages,
    Assign,
}
