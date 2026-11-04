---
name: minigames-game
description: How to write a mini game (bedwars, skywars, …) on top of the `minigames` library: game layers, queuing players, starting and ending games. Load when writing or reviewing a game plugin that uses `minigames`.
---

# Writing a game with minigames

A game is a Bevy plugin of its own. The library owns the queue and moving players into a game layer. The game owns the rest, using ChunkEdge directly where ChunkEdge has the concept: opening game layers, placing players, deciding when a game starts and ends, and the gameplay. Terms are in the glossaries listed in [`GLOSSARY-MAP.md`](../../../GLOSSARY-MAP.md).

## The lifecycle

1. **Name the game** with a build-time-checked slug: `const BEDWARS: GameSlug = GameSlug::from_static("bedwars");`.
2. **Open game layers.** Spawn a ChunkEdge layer with the game's marker: `(LayerBundle, GameLayer { game, capacity })`. It starts `Waiting`. The queue fills only waiting games, so keep one open while players may queue.
3. **Queue players** by writing `QueueMessage::Join { game, players }`, from a command, an NPC or a join hook. `players` is a squad of 1..n players, and they always land in the same game. The queue moves them in on its own: they gain `Playing(layer)` and ChunkEdge respawns them in the layer.
4. **Place arrivals.** Where a player stands is the game's choice, made with ChunkEdge's own components: on `Added<Playing>`, set `Position` and the client's `RespawnPosition`, in `PostUpdate` before `UpdateClientsSet` so the teleport goes out with the respawn.
5. **Start** the game with `transition::<Waiting, Running>()` when its rule is met (full, or enough players and a countdown). A running game takes nobody from the queue.
6. **Play** in systems over `Game<Running>`. Stages specific to the game (a countdown, a deathmatch) are the game's own marker components on the layer, inside `Running`.
7. **End** with `transition::<Running, Ending>()`, wrap up, then despawn the layer (`insert(Despawned)`). Despawning strips `Playing` from its players.

## Shape of a game plugin

```rust
use chunkedge::client::UpdateClientsSet;
use chunkedge::prelude::*;
use minigames::prelude::*;

const BEDWARS: GameSlug = GameSlug::from_static("bedwars");
const SPAWN: DVec3 = DVec3::new(0.5, 65.0, 0.5);

pub struct BedwarsPlugin;

impl Plugin for BedwarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (open_game, start_full_games))
            .add_systems(PostUpdate, place_arrivals.before(UpdateClientsSet));
    }
}

/// Keeps one waiting bedwars game open for the queue to fill.
fn open_game(
    mut commands: Commands,
    games: Query<&GameLayer, With<Waiting>>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    server: Res<Server>,
) {
    if games.iter().any(|game| game.game == BEDWARS) {
        return;
    }
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);
    layer.chunk.insert_chunk(ChunkPos::new(0, 0), UnloadedChunk::new());
    commands.spawn((layer, GameLayer { game: BEDWARS, capacity: 8 }));
}

/// Puts players who just entered a bedwars game at its spawn.
fn place_arrivals(
    mut arrivals: Query<(&Playing, &mut Position, &mut RespawnPosition), Added<Playing>>,
    layers: Query<&GameLayer>,
) {
    for (&Playing(layer), mut pos, mut respawn) in &mut arrivals {
        if layers.get(layer).is_ok_and(|layer| layer.game == BEDWARS) {
            pos.set(SPAWN);
            respawn.pos = SPAWN.into();
        }
    }
}

fn start_full_games(mut commands: Commands, games: Query<(Entity, Game<Waiting>)>) {
    for (layer, game) in &games {
        if game.layer.game == BEDWARS && game.free_slots() == 0 {
            commands.entity(layer).transition::<Waiting, Running>();
        }
    }
}
```

The server adds `MinigamesPlugin` once, then each game's plugin.

## Rules

- **Filter by slug.** `Game<P>` matches every game in phase `P`, whatever its slug, so a game's systems check `game.layer.game == BEDWARS`.
- **Choose a phase by type, not by branching.** A system that only runs for waiting games queries `Game<Waiting>`; free slots exist only there. An illegal transition, such as `Ending → Waiting`, doesn't compile.
- **Let the queue move players.** Players enter a game through `QueueMessage::Join`, or through `Playing(layer)` inserted directly, for example when an admin sends a player in. Either way core does the layer switch. A squad with a player already in a game is ignored by the queue.
- **Read who is playing** from `GamePlayers` on the layer (`game.player_count()`, `game.players`), and a player's game from `Playing`.
- **Not in the library yet:** returning players to a lobby after a game, minimum-player rules and countdowns. The lobby return is the gap that matters: without it, a game must move its players out before it despawns its layer. Ask the user before building any of these, in the game or in the library.

## Testing a game

End-to-end tests use ChunkEdge mock clients. Build the fixture like `crates/minigames/tests/common/mod.rs` does (`app()`, `lobby`, `game_layer`, `connect`), add the game's plugin, write `QueueMessage::Join`, call `app.update()`, then assert on `Playing`, the game's phase marker and the packets the client received.
