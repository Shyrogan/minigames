//! Moving queued squads into waiting games with room.

use chunkedge::prelude::*;
use minigames_game::{Game, MiniGame, Playing, Waiting};

use crate::squad::{Placed, Squad, Squadmates};

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

/// Fills each waiting game of `G` with squads in line for `G`. Only
/// [`Game<Waiting>`] is read, so a started game takes nobody.
pub(crate) fn assign_players<G: MiniGame>(
    squads: Query<(Entity, &Squad<G>, &Squadmates<G>)>,
    games: Query<(Entity, Game<Waiting>)>,
    mut commands: Commands,
) {
    let mut line: Vec<_> = squads.iter().collect();
    line.sort_unstable_by_key(|(_, squad, _)| squad.ticket());

    for (layer, game) in &games {
        if !game.layer.is::<G>() {
            continue;
        }
        let sizes: Vec<_> = line
            .iter()
            .map(|(_, _, squadmates)| squadmates.players().len())
            .collect();
        // Back to front, so the indices left to remove stay valid.
        for index in first_fit(game.free_slots(), &sizes).into_iter().rev() {
            let (squad, _, squadmates) = line.remove(index);
            // Before its players enter the game, which takes them out of
            // the squad.
            commands.entity(squad).try_insert(Placed);
            for &player in squadmates.players() {
                commands.entity(player).try_insert(Playing(layer));
            }
        }
    }
}
