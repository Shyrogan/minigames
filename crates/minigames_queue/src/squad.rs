//! Squads: one entity per squad waiting for a game, and its squadmates.

use chunkedge::prelude::*;
use minigames_game::GameSlug;

/// A squad waiting for a game of `game`, on its own entity. Squads are
/// served in ascending [`ticket`](Self::ticket) order.
#[derive(Component, Clone, Debug)]
pub struct Squad {
    pub game: GameSlug,
    ticket: u64,
}

impl Squad {
    pub(crate) fn new(game: GameSlug, tickets: &mut QueueTickets) -> Self {
        tickets.0 += 1;
        Self {
            game,
            ticket: tickets.0,
        }
    }

    /// The squad's place in line: lower tickets are served first.
    pub fn ticket(&self) -> u64 {
        self.ticket
    }
}

/// The squad a player waits in. A player waits in at most one squad.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[relationship(relationship_target = Squadmates)]
pub struct Queued(pub Entity);

/// The players of a squad, kept in sync by Bevy from [`Queued`].
#[derive(Component, Debug)]
#[relationship_target(relationship = Queued)]
pub struct Squadmates(Vec<Entity>);

impl Squadmates {
    pub fn players(&self) -> &[Entity] {
        &self.0
    }
}

/// Hands out queue tickets, in increasing order.
#[derive(Resource, Default, Debug)]
pub(crate) struct QueueTickets(u64);

/// Despawns a squad once its last player left: Bevy removes an emptied
/// `Squadmates`.
pub(crate) fn despawn_empty_squad(remove: On<Remove, Squadmates>, mut commands: Commands) {
    commands.entity(remove.entity).try_despawn();
}
