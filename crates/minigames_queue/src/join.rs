//! Putting squads in a queue.

use chunkedge::prelude::*;
use minigames_game::Playing;

use crate::message::QueueMessage;
use crate::squad::{QueueTickets, Queued, Squad};

pub(crate) fn join_queues(
    mut messages: MessageReader<QueueMessage>,
    mut tickets: ResMut<QueueTickets>,
    playing: Query<(), With<Playing>>,
    mut commands: Commands,
) {
    for QueueMessage::Join { game, players } in messages.read() {
        if players.is_empty() || players.iter().any(|&player| playing.contains(player)) {
            continue;
        }
        let squad = commands.spawn(Squad::new(game.clone(), &mut tickets)).id();
        for &player in players {
            // Replacing `Queued` takes the player out of its previous squad.
            commands.entity(player).try_insert(Queued(squad));
        }
    }
}
