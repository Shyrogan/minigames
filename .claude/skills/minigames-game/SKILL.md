---
name: minigames-game
description: How to write a mini game (bedwars, skywars, …) on top of the `minigames` library: game layers, queuing players, starting and ending games. Load when writing or reviewing a game plugin that uses `minigames`.
---

# Writing a game with minigames

A game is a Bevy plugin of its own. The library owns the queue and moving players into a game layer. The game owns what happens around that: opening game layers, deciding when a game starts and ends, and the gameplay. Terms are in this repo's [`GLOSSARY.md`](../../../GLOSSARY.md).

## The lifecycle

1. **Name the game** with a build-time-checked slug: `const BEDWARS: GameSlug = GameSlug::from_static("bedwars");`.
2. **Open game layers.** Spawn a ChunkEdge layer with the game's marker and its spawn: `(LayerBundle, GameLayer { game, capacity }, LayerSpawn { pos, yaw, pitch })`. It starts `Waiting`. The queue fills only waiting games, so keep one open while players may queue.
3. **Queue players** by writing `JoinQueue { game, players }`, from a command, an NPC or a join hook. Several players are a party and always land in the same game. The queue moves them in on its own: they gain `InGame(layer)` and respawn at the layer spawn.
4. **Start** the game with `transition::<Waiting, Running>()` when its rule is met (full, or enough players and a countdown). A running game takes nobody from the queue.
5. **Play** in systems over `Game<Running>`. Stages specific to the game (a countdown, a deathmatch) are the game's own marker components on the layer, inside `Running`.
6. **End** with `transition::<Running, Ending>()`, wrap up, then despawn the layer (`insert(Despawned)`). Despawning strips `InGame` from its players.

## Shape of a game plugin

```rust
use chunkedge::prelude::*;
use minigames::prelude::*;

const BEDWARS: GameSlug = GameSlug::from_static("bedwars");

pub struct BedwarsPlugin;

impl Plugin for BedwarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (open_game, start_full_games));
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
    commands.spawn((
        layer,
        GameLayer { game: BEDWARS, capacity: 8 },
        LayerSpawn { pos: DVec3::new(0.5, 65.0, 0.5), yaw: 0.0, pitch: 0.0 },
    ));
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
- **Let the queue move players.** Players enter a game through `JoinQueue`, or through `InGame(layer)` inserted directly, for example when an admin sends a player in. Either way core does the layer switch. A party with a player already in a game is ignored by the queue.
- **Read who is playing** from `GamePlayers` on the layer (`game.player_count()`, `game.players`), and a player's game from `InGame`.
- **Not in the library yet:** returning players to a lobby after a game, minimum-player rules and countdowns. The lobby return is the gap that matters: without it, a game must move its players out before it despawns its layer. Ask the user before building any of these, in the game or in the library.

## Testing a game

End-to-end tests use ChunkEdge mock clients. Build the fixture like `crates/minigames/tests/common/mod.rs` does (`app()`, `lobby`, `game_layer`, `connect`), add the game's plugin, write `JoinQueue`, call `app.update()`, then assert on `InGame`, the game's phase marker and the packets the client received.
