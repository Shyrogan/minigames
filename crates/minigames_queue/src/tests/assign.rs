use super::{Bedwars, Skywars, app, game_layer, has, join, player};
use crate::assign::first_fit;
use crate::*;
use chunkedge::prelude::*;
use minigames_game::*;

#[test]
fn first_fit_takes_what_fits_in_order_without_splitting() {
    assert_eq!(first_fit(4, &[1, 2, 1]), [0, 1, 2]);
    assert_eq!(first_fit(2, &[3, 1, 1]), [1, 2]);
    assert_eq!(first_fit(0, &[1]), [] as [usize; 0]);
}

fn playing(app: &App, player: Entity) -> Option<Entity> {
    app.world().get::<Playing>(player).map(|playing| playing.0)
}

#[test]
fn queued_squads_enter_a_waiting_game_with_room() {
    let mut app = app();
    let arena = game_layer::<Bedwars>(&mut app, 2);
    let trio: Vec<_> = (0..3).map(|_| player(&mut app)).collect();
    let alice = player(&mut app);
    let bob = player(&mut app);
    join::<Bedwars>(&mut app, &trio);
    join::<Bedwars>(&mut app, &[alice]);
    join::<Bedwars>(&mut app, &[bob]);

    for player in [alice, bob] {
        assert_eq!(playing(&app, player), Some(arena));
        assert!(has::<Busy>(&app, player));
        assert!(!has::<Queued<Bedwars>>(&app, player));
    }
    assert_eq!(playing(&app, trio[0]), None);
    assert!(has::<Queued<Bedwars>>(&app, trio[0]));
}

#[test]
fn players_only_enter_waiting_games_of_their_mini_game() {
    let mut app = app();
    let started = game_layer::<Bedwars>(&mut app, 4);
    app.world_mut()
        .commands()
        .entity(started)
        .transition::<Waiting, Running>();
    game_layer::<Skywars>(&mut app, 4);
    let alice = player(&mut app);
    join::<Bedwars>(&mut app, &[alice]);

    assert_eq!(playing(&app, alice), None);
}
