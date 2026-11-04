//! End-to-end game worlds: ChunkEdge layers marked as games, with mock clients
//! joining them and disconnecting.

use std::time::Duration;

use chunkedge::ServerSettings;
use chunkedge::keepalive::KeepaliveSettings;
use chunkedge::prelude::*;
use chunkedge::testing::{self, create_mock_client};
use minigames::prelude::*;

const BEDWARS: GameSlug = GameSlug::from_static("bedwars");
const SKYWARS: GameSlug = GameSlug::from_static("skywars");

fn app() -> App {
    let mut app = App::new();
    app.insert_resource(KeepaliveSettings {
        period: Duration::MAX,
    })
    // Mock clients don't compress, so neither may the layers.
    .insert_resource(ServerSettings {
        compression_threshold: Default::default(),
        ..Default::default()
    });
    testing::add_plugins(&mut app);
    app.add_plugins(MinigamesPlugin)
        .add_systems(Update, despawn_disconnected_clients);
    app.update();
    app
}

/// A ChunkEdge world holding a game of `game`.
fn game_world(app: &mut App, game: GameSlug, capacity: usize) -> Entity {
    let layer = LayerBundle::new(
        ident!("overworld"),
        app.world().resource::<DimensionTypeRegistry>(),
        app.world().resource::<BiomeRegistry>(),
        app.world().resource::<Server>(),
    );
    app.world_mut()
        .spawn((layer, GameWorld { game, capacity }))
        .id()
}

/// A mock client that lives in `world` and is in its game.
fn join(app: &mut App, world: Entity, name: &str) -> Entity {
    let (mut bundle, _helper) =
        create_mock_client(name, &mut app.world_mut().resource_mut::<EntityManager>());
    bundle.layer.0 = world;
    bundle.visible_chunk_layer.0 = world;
    bundle.visible_entity_layers.0.insert(world);
    app.world_mut().spawn((bundle, InGame(world))).id()
}

fn waiting(app: &mut App, world: Entity) -> (usize, usize) {
    let mut query = app.world_mut().query::<Game<Waiting>>();
    let game = query.get(app.world(), world).unwrap();
    (game.player_count(), game.free_slots())
}

#[test]
fn each_world_counts_its_own_players() {
    let mut app = app();
    let bedwars = game_world(&mut app, BEDWARS, 4);
    let skywars = game_world(&mut app, SKYWARS, 8);
    join(&mut app, bedwars, "alice");
    join(&mut app, bedwars, "bob");
    join(&mut app, skywars, "carol");
    app.update();

    assert_eq!(waiting(&mut app, bedwars), (2, 2));
    assert_eq!(waiting(&mut app, skywars), (1, 7));
    assert_eq!(app.world().get::<GameWorld>(skywars).unwrap().game, SKYWARS);
}

#[test]
fn a_disconnected_player_frees_its_slot() {
    let mut app = app();
    let bedwars = game_world(&mut app, BEDWARS, 2);
    let alice = join(&mut app, bedwars, "alice");
    join(&mut app, bedwars, "bob");
    app.update();
    assert_eq!(waiting(&mut app, bedwars), (2, 0));

    app.world_mut().entity_mut(alice).remove::<Client>();
    app.update();

    assert!(app.world().get_entity(alice).is_err());
    assert_eq!(waiting(&mut app, bedwars), (1, 1));
}
