//! Queue statuses: where each player stands with the queue.
//!
//! A player is [`Free`], [`Queued`](crate::Queued) or [`Busy`], exactly one at
//! a time. The queue moves players between them; callers read them.

use chunkedge::prelude::bevy_ecs::lifecycle::HookContext;
use chunkedge::prelude::bevy_ecs::world::DeferredWorld;
use chunkedge::prelude::*;
use minigames_game::Playing;

/// The queue status of a player neither queued nor in a game. Every client
/// starts free.
#[derive(Component, Clone, Copy, Default, Debug)]
#[component(on_insert = keep_only::<Free>)]
pub struct Free;

/// The queue status of a player in a game: a player is busy exactly while it
/// has [`Playing`].
#[derive(Component, Clone, Copy, Default, Debug)]
#[component(on_insert = keep_only::<Busy>)]
pub struct Busy;

/// [`Free`] or [`Busy`]. `Queued` is generic over its mini game, so each
/// [`GameQueuePlugin`](crate::GameQueuePlugin) keeps it apart from these.
trait Status: Component {
    /// The other status, removed when this one is inserted.
    type Other: Component;
}

impl Status for Free {
    type Other = Busy;
}

impl Status for Busy {
    type Other = Free;
}

fn keep_only<S: Status>(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .try_remove::<S::Other>();
}

pub(crate) fn free_new_clients(add: On<Add, Client>, mut commands: Commands) {
    commands.entity(add.entity).try_insert(Free);
}

pub(crate) fn busy_while_playing(insert: On<Insert, Playing>, mut commands: Commands) {
    commands.entity(insert.entity).try_insert(Busy);
}

pub(crate) fn free_after_playing(remove: On<Remove, Playing>, mut commands: Commands) {
    commands.entity(remove.entity).try_insert(Free);
}
