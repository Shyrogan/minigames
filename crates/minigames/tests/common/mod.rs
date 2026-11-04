//! The ChunkEdge fixture shared by the end-to-end tests: an app with
//! `MinigamesPlugin`, layers, and mock clients living in a lobby.

// Each test file uses its own part of the fixture.
#![allow(dead_code)]

use std::time::Duration;

use chunkedge::ServerSettings;
use chunkedge::keepalive::KeepaliveSettings;
use chunkedge::prelude::*;
use chunkedge::testing::{self, MockClientHelper, create_mock_client};
use minigames::prelude::*;

pub struct Bedwars;

impl MiniGame for Bedwars {
    const SLUG: GameSlug = GameSlug::from_static("bedwars");
}

pub fn app() -> App {
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
    app.add_plugins((MinigamesPlugin, GameQueuePlugin::<Bedwars>::default()))
        .add_systems(Update, despawn_disconnected_clients);
    app.update();
    app
}

/// A ChunkEdge layer with one chunk under the spawn.
pub fn layer(app: &mut App) -> LayerBundle {
    let mut layer = LayerBundle::new(
        ident!("overworld"),
        app.world().resource::<DimensionTypeRegistry>(),
        app.world().resource::<BiomeRegistry>(),
        app.world().resource::<Server>(),
    );
    layer
        .chunk
        .insert_chunk(ChunkPos::new(0, 0), UnloadedChunk::default());
    layer
}

/// A layer holding a game of `G`.
pub fn game_layer<G: MiniGame>(app: &mut App, capacity: usize) -> Entity {
    let layer = layer(app);
    app.world_mut()
        .spawn((layer, GameLayer::new::<G>(capacity)))
        .id()
}

/// A mock client living in `lobby`, its join traffic already received.
pub fn connect(app: &mut App, lobby: Entity, name: &str) -> (Entity, MockClientHelper) {
    let (mut bundle, mut helper) =
        create_mock_client(name, &mut app.world_mut().resource_mut::<EntityManager>());
    bundle.layer.0 = lobby;
    bundle.visible_chunk_layer.0 = lobby;
    bundle.visible_entity_layers.0.insert(lobby);
    let client = app.world_mut().spawn(bundle).id();
    app.update();
    helper.clear_received();
    (client, helper)
}

/// A lobby layer, which is no game.
pub fn lobby(app: &mut App) -> Entity {
    let layer = layer(app);
    app.world_mut().spawn(layer).id()
}

/// Player count and free slots of a waiting game.
pub fn waiting(app: &mut App, layer: Entity) -> (usize, usize) {
    let mut query = app.world_mut().query::<Game<Waiting>>();
    let game = query.get(app.world(), layer).unwrap();
    (game.player_count(), game.free_slots())
}
