//! Moving queued squads into waiting games with room.

use std::collections::HashMap;

use chunkedge::prelude::*;
use minigames_game::{Game, GameSlug, Playing, Waiting};

use crate::squad::{Queued, Squad, Squadmates};

/// The squads to take, by index, for a game with `free` slots, given the
/// squads' sizes in queue order: each squad that still fits, whole. A squad
/// too big for the slots left waits; smaller squads behind it go first.
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

/// Fills each waiting game with queued squads of its game. Only
/// [`Game<Waiting>`] is read, so a started game takes nobody.
/// A game's queue: squads as (ticket, squad, squadmates).
type Line<'a> = Vec<(u64, Entity, &'a [Entity])>;

pub(crate) fn assign_players(
    squads: Query<(Entity, &Squad, &Squadmates)>,
    games: Query<(Entity, Game<Waiting>)>,
    mut commands: Commands,
) {
    let mut queues: HashMap<&GameSlug, Line> = HashMap::new();
    for (entity, squad, squadmates) in &squads {
        queues
            .entry(&squad.game)
            .or_default()
            .push((squad.ticket(), entity, squadmates.players()));
    }
    for queue in queues.values_mut() {
        queue.sort_unstable_by_key(|&(ticket, ..)| ticket);
    }

    for (layer, game) in &games {
        let Some(queue) = queues.get_mut(&game.layer.game) else {
            continue;
        };
        let sizes: Vec<_> = queue
            .iter()
            .map(|(_, _, squadmates)| squadmates.len())
            .collect();
        // Back to front, so the indices left to remove stay valid.
        for index in first_fit(game.free_slots(), &sizes).into_iter().rev() {
            let (_, squad, squadmates) = queue.remove(index);
            for &player in squadmates {
                commands
                    .entity(player)
                    .try_remove::<Queued>()
                    .try_insert(Playing(layer));
            }
            commands.entity(squad).try_despawn();
        }
    }
}
