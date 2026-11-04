use super::{BEDWARS, SKYWARS, app, join};
use crate::entry::EntryMembers;
use crate::*;
use chunkedge::prelude::*;
use minigames_core::*;

fn entry(app: &App, player: Entity) -> Option<Entity> {
    app.world().get::<Queued>(player).map(|queued| queued.0)
}

#[test]
fn a_party_shares_one_entry() {
    let mut app = app();
    let party = [
        app.world_mut().spawn_empty().id(),
        app.world_mut().spawn_empty().id(),
    ];
    join(&mut app, BEDWARS, &party);

    let entry = entry(&app, party[0]).unwrap();
    assert_eq!(app.world().get::<QueueEntry>(entry).unwrap().game, BEDWARS);
    assert_eq!(
        app.world().get::<EntryMembers>(entry).unwrap().players(),
        party
    );
}

#[test]
fn joining_another_queue_leaves_the_first() {
    let mut app = app();
    let alice = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &[alice]);
    let first = entry(&app, alice).unwrap();

    join(&mut app, SKYWARS, &[alice]);

    let second = entry(&app, alice).unwrap();
    assert_eq!(app.world().get::<QueueEntry>(second).unwrap().game, SKYWARS);
    assert!(app.world().get_entity(first).is_err());
}

#[test]
fn a_player_in_a_game_cant_queue() {
    let mut app = app();
    let layer = app.world_mut().spawn_empty().id();
    let alice = app.world_mut().spawn(InGame(layer)).id();
    let bob = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &[bob, alice]);

    assert_eq!(entry(&app, bob), None);
}
