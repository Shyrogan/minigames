//! End-to-end queue: mock clients queue in a lobby and move into a waiting
//! game with room.

mod common;

use chunkedge::prelude::*;
use chunkedge::protocol::packets::play::RespawnS2c;
use common::{BEDWARS, app, connect, game_layer, lobby};
use minigames::prelude::*;

#[test]
fn a_queued_squad_moves_into_a_waiting_game() {
    let mut app = app();
    let lobby = lobby(&mut app);
    let bedwars = game_layer(&mut app, BEDWARS, 4);
    let (alice, mut alice_helper) = connect(&mut app, lobby, "alice");
    let (bob, mut bob_helper) = connect(&mut app, lobby, "bob");

    app.world_mut()
        .resource_mut::<Messages<QueueMessage>>()
        .write(QueueMessage::Join {
            game: BEDWARS,
            players: vec![alice, bob],
        });
    app.update();

    for (player, helper) in [(alice, &mut alice_helper), (bob, &mut bob_helper)] {
        assert_eq!(app.world().get::<Playing>(player).unwrap().0, bedwars);
        helper.collect_received().assert_count::<RespawnS2c>(1);
    }
}
