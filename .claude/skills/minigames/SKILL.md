---
name: minigames
description: Design rules, invariants and ChunkEdge/Bevy 0.19 gotchas of the `minigames` library (mini game worlds, phases, players in games, the upcoming queue and placement). Load before writing or reviewing any code in this repo, or designing a new crate for it.
---

# minigames

A Rust library for a ChunkEdge mini game server: worlds that hold games, and later a queue that moves players into a game with room. Terms (game, game slug, game world, phase, player in game) are defined in [`GLOSSARY.md`](../../../GLOSSARY.md); use them exactly, in code and prose.

## Phase-typed design

The library's core idea: a game's phase is a **type**, so the compiler rejects code that acts on a game in the wrong phase. Each phase is a zero-sized marker component (`Waiting`, `Running`, `Ending`), and a game world holds exactly one.

- Select games by phase in the query type: `Query<Game<Waiting>>`. A system handles one phase; write a generic system over `P: GamePhase` when the logic is the same in every phase.
- Put a phase-only capability on that phase's item: `impl GameItem<'_, '_, Waiting> { fn free_slots(..) }`. A capability that only makes sense in one phase is unreachable from the others, with no runtime check.
- Move a game on with `commands.entity(world).transition::<From, To>()`. The legal edges are the `Next` impls in `transition.rs`; adding an edge is adding an impl. `transition` is a no-op when the game already left `From`, so concurrent systems never skip a phase.
- The phase set is sealed in `phase.rs`, because the queue and placement rely on `Waiting` meaning "takes players". Mini-game-specific stages belong in their own marker components inside `Running`.

## Invariants

- **One phase per world.** The `keep_only` insert hook removes the other phases, so even a hand-inserted marker keeps the invariant.
- **One game per player.** `InGame(world)` is a Bevy relationship whose target `GamePlayers` Bevy keeps in sync; despawning a world strips `InGame` from its players and keeps the players.
- **`InGame` is membership only.** Placing a player in a game also switches its layers: `EntityLayerId`, `VisibleChunkLayer`, `VisibleEntityLayers` (clear, then insert the world), `Position` and `RespawnPosition`, before `UpdateClientsSet` in `PostUpdate`. Changing `VisibleChunkLayer` makes ChunkEdge send the respawn.
- **Capacity is advisory.** Whoever adds a player checks `free_slots()` of the `Game<Waiting>` first; core counts over-capacity worlds as zero free slots.
- **Slugs are valid by construction.** Known games are `const X: GameSlug = GameSlug::from_static("x")`, checked at build time; runtime input goes through `GameSlug::new`.

## ChunkEdge and Bevy 0.19 gotchas

- ChunkEdge comes from git, `ChunkEdge/ChunkEdge` branch `main`; the crates.io `chunkedge` is an empty placeholder. `warden` uses a fork's `dev` branch, so the two can't share a server binary.
- Import Bevy through ChunkEdge: `chunkedge::prelude::*`, and `chunkedge::prelude::bevy_ecs::…` for paths outside the prelude. No crate here depends on `bevy_ecs` directly.
- A `#[derive(QueryData)]` field can't be a filter: `With<P>` fails to compile. Use a private `&'static P` field; markers are zero-sized.
- A `RelationshipTarget` is absent while empty, so read `GamePlayers` as `Option<&GamePlayers>`.
- ChunkEdge has no join or leave events. A join is `Added<Client>`; a disconnect removes `Client`, and `despawn_disconnected_clients` (which the app adds) marks the entity `Despawned`, despawned in `Last`. React with an `On<Remove, Client>` observer or `RemovedComponents<Client>`.
- Cross-crate communication uses Messages (`#[derive(Message)]`, `MessageReader`/`MessageWriter`), as ChunkEdge does.

## Layout

- Workspace crates under `crates/`: `minigames_core` (shared types), one crate per feature (the queue next), and `minigames` as the umbrella re-exporting them behind `MinigamesPlugin`.
- One responsibility per file. `lib.rs` holds only crate docs, `mod` lines and re-exports.
- Every public item is exported through `minigames::prelude`; the `minigames-prelude` skill has the rules.
- Each file carries its own `#[cfg(test)] mod tests` with sentence-like names (`a_started_game_has_no_free_slots`), on a bare Bevy `World`. Shared fixtures live in `test_support.rs`.
- End-to-end tests with real ChunkEdge mock clients live only in `crates/minigames/tests/`; copy the fixture in `tests/world.rs` (`app()`, `game_world`, `join`).
- Prove a compile-time guarantee with a `compile_fail` doctest beside a passing one, as `GameSlug::from_static` and `transition` do.

## Done

The work is done when `nix flake check` passes (clippy `--deny warnings`, every test, treefmt) and `GLOSSARY.md` names every new term.
