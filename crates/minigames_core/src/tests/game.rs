use super::{free_slots, game_layer, players, transition};
use crate::phase::Running;
use crate::players::Playing;
use crate::*;
use chunkedge::prelude::*;

#[test]
fn a_new_game_waits_for_players() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 2);
    assert_eq!(free_slots(&mut world, arena), Some(2));
    assert_eq!(players::<Running>(&mut world, arena), None);
}

#[test]
fn free_slots_follow_players() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 2);
    world.spawn(Playing(arena));
    assert_eq!(free_slots(&mut world, arena), Some(1));
    // Over capacity, since nothing enforces it: no slot left, not an
    // underflow.
    world.spawn_batch([Playing(arena), Playing(arena)]);
    assert_eq!(players::<Waiting>(&mut world, arena), Some(3));
    assert_eq!(free_slots(&mut world, arena), Some(0));
}

#[test]
fn a_started_game_has_no_free_slots() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 4);
    world.spawn(Playing(arena));

    transition::<Waiting, Running>(&mut world, arena);
    assert_eq!(free_slots(&mut world, arena), None);
    assert_eq!(players::<Running>(&mut world, arena), Some(1));
}
