use chunkedge::prelude::*;
use std::collections::BTreeSet;

use super::game_layer;
use crate::enter::enter_game;
use crate::*;

#[test]
fn entering_a_game_swaps_only_the_players_layer() {
    let mut world = World::new();
    world.add_observer(enter_game);
    let (lobby, scoreboard) = (world.spawn_empty().id(), world.spawn_empty().id());
    let (first, second) = (game_layer(&mut world, 4), game_layer(&mut world, 4));
    let player = world
        .spawn((
            EntityLayerId(lobby),
            VisibleChunkLayer(lobby),
            VisibleEntityLayers(BTreeSet::from([lobby, scoreboard])),
        ))
        .id();

    for layer in [first, second] {
        world.entity_mut(player).insert(Playing(layer));

        assert_eq!(world.get::<EntityLayerId>(player).unwrap().0, layer);
        assert_eq!(world.get::<VisibleChunkLayer>(player).unwrap().0, layer);
        let visible = &world.get::<VisibleEntityLayers>(player).unwrap().0;
        assert_eq!(*visible, BTreeSet::from([scoreboard, layer]));
    }
}
