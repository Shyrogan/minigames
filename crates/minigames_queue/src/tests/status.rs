use super::{Bedwars, app, game_layer, has, player};
use crate::*;
use minigames_game::*;

#[test]
fn a_player_is_busy_exactly_while_playing() {
    let mut app = app();
    let arena = game_layer::<Bedwars>(&mut app, 4);
    let alice = player(&mut app);

    app.world_mut().entity_mut(alice).insert(Playing(arena));
    app.update();
    assert!(has::<Busy>(&app, alice) && !has::<Free>(&app, alice));

    app.world_mut().entity_mut(alice).remove::<Playing>();
    app.update();
    assert!(has::<Free>(&app, alice) && !has::<Busy>(&app, alice));
}
