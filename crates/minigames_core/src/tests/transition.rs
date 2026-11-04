use super::{game_layer, players, transition};
use crate::*;
use chunkedge::prelude::*;

#[test]
fn a_game_is_in_one_phase_at_a_time() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 2);

    transition::<Waiting, Running>(&mut world, arena);
    assert_eq!(players::<Waiting>(&mut world, arena), None);
    assert_eq!(players::<Running>(&mut world, arena), Some(0));

    transition::<Running, Ending>(&mut world, arena);
    assert_eq!(players::<Running>(&mut world, arena), None);
    assert_eq!(players::<Ending>(&mut world, arena), Some(0));

    // Inserting a phase by hand still drops the current one.
    world.entity_mut(arena).insert(Waiting);
    world.flush();
    assert_eq!(players::<Ending>(&mut world, arena), None);
    assert_eq!(players::<Waiting>(&mut world, arena), Some(0));
}

#[test]
fn a_transition_from_another_phase_does_nothing() {
    let mut world = World::new();
    let arena = game_layer(&mut world, 2);
    transition::<Running, Ending>(&mut world, arena);
    assert_eq!(players::<Waiting>(&mut world, arena), Some(0));
    assert_eq!(players::<Ending>(&mut world, arena), None);
}
