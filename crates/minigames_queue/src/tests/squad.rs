use super::{Bedwars, app, game_layer, has, join, player};
use crate::*;
use minigames_game::*;

#[test]
fn a_despawned_player_frees_its_squad() {
    let mut app = app();
    let alice = player(&mut app);
    let bob = player(&mut app);
    join::<Bedwars>(&mut app, &[alice, bob]);
    let squad = app.world().get::<Queued<Bedwars>>(bob).unwrap().squad();

    app.world_mut().despawn(alice);
    app.update();

    assert!(has::<Free>(&app, bob));
    assert!(app.world().get_entity(squad).is_err());
}

#[test]
fn a_game_taking_a_queued_player_frees_its_squad() {
    let mut app = app();
    let arena = game_layer::<Bedwars>(&mut app, 1);
    // The arena is full, so the queue places nobody.
    let carol = player(&mut app);
    app.world_mut().entity_mut(carol).insert(Playing(arena));
    let alice = player(&mut app);
    let bob = player(&mut app);
    join::<Bedwars>(&mut app, &[alice, bob]);

    app.world_mut().entity_mut(alice).insert(Playing(arena));
    app.update();

    assert!(has::<Busy>(&app, alice));
    assert!(has::<Free>(&app, bob));
}
