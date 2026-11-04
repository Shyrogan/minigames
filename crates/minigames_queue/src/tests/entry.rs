use crate::*;
use chunkedge::prelude::*;

use super::BEDWARS;
use crate::entry::{QueueTickets, despawn_empty_entry};

#[test]
fn an_entry_goes_away_with_its_last_member() {
    let mut world = World::new();
    world.add_observer(despawn_empty_entry);
    let entry = world
        .spawn(QueueEntry::new(BEDWARS, &mut QueueTickets::default()))
        .id();
    let alice = world.spawn(Queued(entry)).id();
    let bob = world.spawn(Queued(entry)).id();

    world.entity_mut(alice).remove::<Queued>();
    world.flush();
    assert_eq!(world.get::<EntryMembers>(entry).unwrap().players(), [bob]);

    world.despawn(bob);
    world.flush();
    assert!(world.get_entity(entry).is_err());
}
