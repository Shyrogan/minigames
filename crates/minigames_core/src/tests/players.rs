use super::{free_slots, game_layer};
use crate::*;
use chunkedge::prelude::*;

#[test]
fn moving_a_player_updates_both_games() {
    let mut world = World::new();
    let from = game_layer(&mut world, 4);
    let to = game_layer(&mut world, 4);
    let player = world.spawn(InGame(from)).id();

    world.entity_mut(player).insert(InGame(to));
    assert_eq!(free_slots(&mut world, from), Some(4));
    assert_eq!(world.get::<GamePlayers>(to).unwrap().players(), [player]);
}

#[test]
fn despawning_a_layer_keeps_its_players() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 4);
    let player = world.spawn(InGame(arena)).id();

    world.despawn(arena);
    assert!(world.get_entity(player).is_ok());
    assert!(world.get::<InGame>(player).is_none());
}

#[test]
fn a_despawned_player_leaves_its_game() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 4);
    let player = world.spawn(InGame(arena)).id();

    world.despawn(player);
    assert_eq!(free_slots(&mut world, arena), Some(4));
}
