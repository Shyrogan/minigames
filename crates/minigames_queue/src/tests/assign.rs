use crate::*;
use chunkedge::prelude::*;
use minigames_core::*;

use super::{BEDWARS, SKYWARS, app, game_layer, join};
use crate::assign::first_fit;
use minigames_core::{GamePhaseCommands, Running};

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
    let arena = game_layer(&mut app, BEDWARS, 2);
    let trio: Vec<_> = (0..3).map(|_| app.world_mut().spawn_empty().id()).collect();
    let alice = app.world_mut().spawn_empty().id();
    let bob = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &trio);
    join(&mut app, BEDWARS, &[alice]);
    join(&mut app, BEDWARS, &[bob]);

    assert_eq!(playing(&app, alice), Some(arena));
    assert_eq!(playing(&app, bob), Some(arena));
    assert!(app.world().get::<Queued>(alice).is_none());
    assert_eq!(playing(&app, trio[0]), None);
}

#[test]
fn players_only_enter_waiting_games_of_their_queue() {
    let mut app = app();
    let started = game_layer(&mut app, BEDWARS, 4);
    app.world_mut()
        .commands()
        .entity(started)
        .transition::<Waiting, Running>();
    game_layer(&mut app, SKYWARS, 4);
    let alice = app.world_mut().spawn_empty().id();
    join(&mut app, BEDWARS, &[alice]);

    assert_eq!(playing(&app, alice), None);
}
