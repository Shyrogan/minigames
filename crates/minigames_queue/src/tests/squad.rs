use crate::*;
use chunkedge::prelude::*;

use super::BEDWARS;
use crate::squad::{QueueTickets, despawn_empty_squad};

#[test]
fn a_squad_goes_away_with_its_last_player() {
    let mut world = World::new();
    world.add_observer(despawn_empty_squad);
    let squad = world
        .spawn(Squad::new(BEDWARS, &mut QueueTickets::default()))
        .id();
    let alice = world.spawn(Queued(squad)).id();
    let bob = world.spawn(Queued(squad)).id();

    world.entity_mut(alice).remove::<Queued>();
    world.flush();
    assert_eq!(world.get::<Squadmates>(squad).unwrap().players(), [bob]);

    world.despawn(bob);
    world.flush();
    assert!(world.get_entity(squad).is_err());
}
