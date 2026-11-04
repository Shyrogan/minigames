use super::{Bedwars, Skywars, app, game_layer, has, join, player, send};
use crate::*;
use chunkedge::prelude::*;
use minigames_game::*;

fn squad(app: &App, player: Entity) -> Option<Entity> {
    app.world()
        .get::<Queued<Bedwars>>(player)
        .map(Queued::squad)
}

#[test]
fn players_joining_together_form_one_squad() {
    let mut app = app();
    let players = [player(&mut app), player(&mut app)];
    join::<Bedwars>(&mut app, &players);

    let squad = squad(&app, players[0]).unwrap();
    assert_eq!(
        app.world()
            .get::<Squadmates<Bedwars>>(squad)
            .unwrap()
            .players(),
        players
    );
    assert!(!has::<Free>(&app, players[0]));
}

#[test]
fn only_free_players_can_queue() {
    let mut app = app();
    let arena = game_layer::<Skywars>(&mut app, 4);
    let alice = player(&mut app);
    let bob = player(&mut app);
    app.world_mut().entity_mut(alice).insert(Playing(arena));
    app.update();
    join::<Bedwars>(&mut app, &[bob, alice]);
    assert_eq!(squad(&app, bob), None);

    // Queued isn't free either: joining another queue is ignored.
    join::<Bedwars>(&mut app, &[bob]);
    join::<Skywars>(&mut app, &[bob]);
    assert!(squad(&app, bob).is_some());
    assert!(!has::<Queued<Skywars>>(&app, bob));
}

#[test]
fn leaving_frees_the_whole_squad() {
    let mut app = app();
    let alice = player(&mut app);
    let bob = player(&mut app);
    join::<Bedwars>(&mut app, &[alice, bob]);
    let squad = squad(&app, alice).unwrap();

    send(&mut app, QueueMessage::Leave { player: alice });

    for player in [alice, bob] {
        assert!(has::<Free>(&app, player));
        assert!(!has::<Queued<Bedwars>>(&app, player));
    }
    assert!(app.world().get_entity(squad).is_err());
}
