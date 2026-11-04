//! The messages the queue reads, and how it handles them.

use std::collections::{HashMap, HashSet};

use chunkedge::prelude::*;
use minigames_game::GameSlug;

use crate::squad::QueueTickets;
use crate::status::{Busy, Free};

/// A request to the queue.
#[derive(Message, Clone, Debug)]
pub enum QueueMessage {
    /// Puts `players` in line for the mini game named `game`, as one squad:
    /// they enter the same game together. Ignored unless every player is
    /// free and `game` has a [`GameQueuePlugin`](crate::GameQueuePlugin).
    Join {
        game: GameSlug,
        players: Vec<Entity>,
    },
    /// Takes `player` out of the queue: its whole squad becomes free.
    /// Ignored unless the player is queued.
    Leave { player: Entity },
}

/// Spawns a squad of one mini game: `(commands, ticket, players)`.
pub(crate) type SpawnSquad = fn(&mut Commands, u64, &[Entity]);

/// How to spawn a squad, by mini game slug, filled by each
/// [`GameQueuePlugin`](crate::GameQueuePlugin).
#[derive(Resource, Default)]
pub(crate) struct Queues(pub(crate) HashMap<GameSlug, SpawnSquad>);

pub(crate) fn handle_queue_messages(
    mut messages: MessageReader<QueueMessage>,
    queues: Res<Queues>,
    mut tickets: ResMut<QueueTickets>,
    statuses: Query<(Has<Free>, Has<Busy>)>,
    mut commands: Commands,
) {
    // Statuses read as they were before this tick's joins, so remember who
    // joined a squad already.
    let mut joined = HashSet::new();
    for message in messages.read() {
        match message {
            QueueMessage::Join { game, players } => {
                let Some(spawn_squad) = queues.0.get(game) else {
                    continue;
                };
                let squad: HashSet<_> = players.iter().copied().collect();
                let all_free = squad
                    .iter()
                    .all(|&player| statuses.get(player).is_ok_and(|(free, _)| free));
                if squad.is_empty() || squad.len() != players.len() || !all_free {
                    continue;
                }
                if !squad.is_disjoint(&joined) {
                    continue;
                }
                joined.extend(squad);
                spawn_squad(&mut commands, tickets.next(), players);
            }
            &QueueMessage::Leave { player } => {
                // Exactly one status: neither free nor busy is queued.
                if statuses.get(player) == Ok((false, false)) {
                    commands.entity(player).try_insert(Free);
                }
            }
        }
    }
}
