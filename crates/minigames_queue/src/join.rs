//! Putting parties in a queue.

use chunkedge::prelude::*;
use minigames_core::{GameSlug, InGame};

use crate::entry::{QueueEntry, QueueTickets, Queued};

/// Puts `players` in line for `game`, as one party: they enter the same game
/// together. Players queued elsewhere switch to this queue. Ignored if the
/// party is empty or one of its players is in a game already.
#[derive(Message, Clone, Debug)]
pub struct JoinQueue {
    pub game: GameSlug,
    pub players: Vec<Entity>,
}

pub(crate) fn join_queues(
    mut joins: MessageReader<JoinQueue>,
    mut tickets: ResMut<QueueTickets>,
    in_game: Query<(), With<InGame>>,
    mut commands: Commands,
) {
    for JoinQueue { game, players } in joins.read() {
        if players.is_empty() || players.iter().any(|&player| in_game.contains(player)) {
            continue;
        }
        let entry = commands
            .spawn(QueueEntry::new(game.clone(), &mut tickets))
            .id();
        for &player in players {
            // Replacing `Queued` takes the player out of its previous entry.
            commands.entity(player).try_insert(Queued(entry));
        }
    }
}
