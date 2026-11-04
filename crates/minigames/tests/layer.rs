//! End-to-end game layers: ChunkEdge layers marked as games, with mock clients
//! entering them and disconnecting.

mod common;

use chunkedge::prelude::*;
use chunkedge::protocol::packets::play::{LevelChunkWithLightS2c, RespawnS2c};
use common::{BEDWARS, app, connect, game_layer, lobby, waiting};
use minigames::prelude::*;

#[test]
fn entering_a_game_respawns_the_client_in_its_layer() {
    let mut app = app();
    let lobby = lobby(&mut app);
    let bedwars = game_layer(&mut app, BEDWARS, 4);
    let (alice, mut helper) = connect(&mut app, lobby, "alice");

    app.world_mut().entity_mut(alice).insert(Playing(bedwars));
    app.update();

    // As in ChunkEdge's own layer switching: the respawn clears the client's
    // chunks, so it comes before the new layer's.
    let received = helper.collect_received();
    received.assert_count::<RespawnS2c>(1);
    received.assert_order::<(RespawnS2c, LevelChunkWithLightS2c)>();
    assert_eq!(
        app.world().get::<VisibleChunkLayer>(alice).unwrap().0,
        bedwars
    );
}

#[test]
fn a_disconnected_player_frees_its_slot() {
    let mut app = app();
    let lobby = lobby(&mut app);
    let bedwars = game_layer(&mut app, BEDWARS, 2);
    let (alice, _) = connect(&mut app, lobby, "alice");
    let (bob, _) = connect(&mut app, lobby, "bob");
    for player in [alice, bob] {
        app.world_mut().entity_mut(player).insert(Playing(bedwars));
    }
    app.update();
    assert_eq!(waiting(&mut app, bedwars), (2, 0));

    app.world_mut().entity_mut(alice).remove::<Client>();
    app.update();

    assert!(app.world().get_entity(alice).is_err());
    assert_eq!(waiting(&mut app, bedwars), (1, 1));
}
