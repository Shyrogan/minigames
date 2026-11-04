use chunkedge::prelude::*;
use std::collections::BTreeSet;

use super::{BEDWARS, SPAWN};
use crate::enter::enter_game;
use crate::spawn::LayerSpawn;
use crate::*;

const ELSEWHERE: LayerSpawn = LayerSpawn {
    pos: DVec3::new(100.5, 70.0, -20.5),
    yaw: 90.0,
    pitch: 10.0,
};

fn world() -> World {
    let mut world = World::new();
    world.add_observer(enter_game);
    world
}

fn layer(world: &mut World, spawn: LayerSpawn) -> Entity {
    let game = GameLayer {
        game: BEDWARS,
        capacity: 4,
    };
    world.spawn((game, spawn)).id()
}

/// A client-like player living in `lobby`, which also sees `extra`.
fn player(world: &mut World, lobby: Entity, extra: Entity) -> Entity {
    world
        .spawn((
            EntityLayerId(lobby),
            VisibleChunkLayer(lobby),
            VisibleEntityLayers(BTreeSet::from([lobby, extra])),
            Position::default(),
            Look::default(),
            RespawnPosition::default(),
        ))
        .id()
}

fn visible(world: &World, player: Entity) -> Vec<Entity> {
    world
        .get::<VisibleEntityLayers>(player)
        .unwrap()
        .0
        .iter()
        .copied()
        .collect()
}

#[test]
fn entering_a_game_moves_the_player_to_its_spawn() {
    let mut world = world();
    let (lobby, scoreboard) = (world.spawn_empty().id(), world.spawn_empty().id());
    let arena = layer(&mut world, SPAWN);
    let player = player(&mut world, lobby, scoreboard);

    world.entity_mut(player).insert(InGame(arena));

    assert_eq!(world.get::<EntityLayerId>(player).unwrap().0, arena);
    assert_eq!(world.get::<VisibleChunkLayer>(player).unwrap().0, arena);
    let mut expected = vec![scoreboard, arena];
    expected.sort();
    assert_eq!(visible(&world, player), expected);
    assert_eq!(world.get::<Position>(player).unwrap().0, SPAWN.pos);
    let look = world.get::<Look>(player).unwrap();
    assert_eq!((look.yaw, look.pitch), (SPAWN.yaw, SPAWN.pitch));
    let respawn = world.get::<RespawnPosition>(player).unwrap();
    assert_eq!(respawn.pos, SPAWN.pos.into());
    assert_eq!((respawn.yaw, respawn.pitch), (SPAWN.yaw, SPAWN.pitch));
}

#[test]
fn moving_to_another_game_swaps_the_game_layer() {
    let mut world = world();
    let (lobby, scoreboard) = (world.spawn_empty().id(), world.spawn_empty().id());
    let first = layer(&mut world, SPAWN);
    let second = layer(&mut world, ELSEWHERE);
    let player = player(&mut world, lobby, scoreboard);

    world.entity_mut(player).insert(InGame(first));
    world.entity_mut(player).insert(InGame(second));

    assert_eq!(world.get::<VisibleChunkLayer>(player).unwrap().0, second);
    let mut expected = vec![scoreboard, second];
    expected.sort();
    assert_eq!(visible(&world, player), expected);
    assert_eq!(world.get::<Position>(player).unwrap().0, ELSEWHERE.pos);
}
