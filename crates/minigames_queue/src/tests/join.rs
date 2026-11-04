use super::{BEDWARS, SKYWARS, app, join};
use crate::squad::Squadmates;
use crate::*;
use chunkedge::prelude::*;
use minigames_core::*;

fn squad(app: &App, player: Entity) -> Option<Entity> {
    app.world().get::<Queued>(player).map(|queued| queued.0)
}

#[test]
fn players_joining_together_form_one_squad() {
    let mut app = app();
    let players = [
        app.world_mut().spawn_empty().id(),
        app.world_mut().spawn_empty().id(),
    ];
    join(&mut app, BEDWARS, &players);

    let squad = squad(&app, players[0]).unwrap();
    assert_eq!(app.world().get::<Squad>(squad).unwrap().game, BEDWARS);
    assert_eq!(
        app.world().get::<Squadmates>(squad).unwrap().players(),
        players
    );
}

#[test]
fn joining_another_queue_leaves_the_first() {
    let mut app = app();
    let alice = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &[alice]);
    let first = squad(&app, alice).unwrap();

    join(&mut app, SKYWARS, &[alice]);

    let second = squad(&app, alice).unwrap();
    assert_eq!(app.world().get::<Squad>(second).unwrap().game, SKYWARS);
    assert!(app.world().get_entity(first).is_err());
}

#[test]
fn a_player_in_a_game_cant_queue() {
    let mut app = app();
    let layer = app.world_mut().spawn_empty().id();
    let alice = app.world_mut().spawn(Playing(layer)).id();
    let bob = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &[bob, alice]);

    assert_eq!(squad(&app, bob), None);
}
