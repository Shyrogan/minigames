//! Queue entries: one entity per party waiting for a game, and its members.

use chunkedge::prelude::*;
use minigames_core::GameSlug;

/// A party waiting for a game of `game`, on its own entity. Entries are
/// served in ascending [`ticket`](Self::ticket) order.
#[derive(Component, Clone, Debug)]
pub struct QueueEntry {
    pub game: GameSlug,
    ticket: u64,
}

impl QueueEntry {
    pub(crate) fn new(game: GameSlug, tickets: &mut QueueTickets) -> Self {
        tickets.0 += 1;
        Self {
            game,
            ticket: tickets.0,
        }
    }

    /// The entry's place in line: lower tickets are served first.
    pub fn ticket(&self) -> u64 {
        self.ticket
    }
}

/// The queue entry a player waits in. A player waits in at most one queue.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[relationship(relationship_target = EntryMembers)]
pub struct Queued(pub Entity);

/// The players of a queue entry, kept in sync by Bevy from [`Queued`].
#[derive(Component, Debug)]
#[relationship_target(relationship = Queued)]
pub struct EntryMembers(Vec<Entity>);

impl EntryMembers {
    pub fn players(&self) -> &[Entity] {
        &self.0
    }
}

/// Hands out queue tickets, in increasing order.
#[derive(Resource, Default, Debug)]
pub(crate) struct QueueTickets(u64);

/// Despawns an entry once its last member left: Bevy removes an emptied
/// `EntryMembers`.
pub(crate) fn despawn_empty_entry(remove: On<Remove, EntryMembers>, mut commands: Commands) {
    commands.entity(remove.entity).try_despawn();
}
