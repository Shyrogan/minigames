//! Moving queued parties into waiting games with room.

use std::collections::HashMap;

use chunkedge::prelude::*;
use minigames_core::{Game, GameSlug, InGame, Waiting};

use crate::entry::{EntryMembers, QueueEntry, Queued};

/// The entries to take, by index, for a game with `free` slots, given the
/// entries' sizes in queue order: each entry that still fits, whole. A party
/// too big for the slots left waits; smaller entries behind it go first.
pub(crate) fn first_fit(mut free: usize, sizes: &[usize]) -> Vec<usize> {
    let mut taken = Vec::new();
    for (index, &size) in sizes.iter().enumerate() {
        if size <= free {
            free -= size;
            taken.push(index);
        }
    }
    taken
}

/// Fills each waiting game with queued parties of its game. Only
/// [`Game<Waiting>`] is read, so a started game takes nobody.
/// A game's queue: entries as (ticket, entry, members).
type Line<'a> = Vec<(u64, Entity, &'a [Entity])>;

pub(crate) fn assign_players(
    entries: Query<(Entity, &QueueEntry, &EntryMembers)>,
    games: Query<(Entity, Game<Waiting>)>,
    mut commands: Commands,
) {
    let mut queues: HashMap<&GameSlug, Line> = HashMap::new();
    for (entry, queued, members) in &entries {
        queues
            .entry(&queued.game)
            .or_default()
            .push((queued.ticket(), entry, members.players()));
    }
    for queue in queues.values_mut() {
        queue.sort_unstable_by_key(|&(ticket, ..)| ticket);
    }

    for (layer, game) in &games {
        let Some(queue) = queues.get_mut(&game.layer.game) else {
            continue;
        };
        let sizes: Vec<_> = queue.iter().map(|(_, _, members)| members.len()).collect();
        // Back to front, so the indices left to remove stay valid.
        for index in first_fit(game.free_slots(), &sizes).into_iter().rev() {
            let (_, entry, members) = queue.remove(index);
            for &player in members {
                commands
                    .entity(player)
                    .try_remove::<Queued>()
                    .try_insert(InGame(layer));
            }
            commands.entity(entry).try_despawn();
        }
    }
}
