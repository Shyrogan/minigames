---
name: minigames
description: Design rules, invariants and ChunkEdge/Bevy 0.19 gotchas of the `minigames` library (game layers, phases, players in games, the queue). Load before writing or reviewing any code in this repo, or designing a new crate for it.
---

# minigames

A Rust library for a ChunkEdge mini game server: layers that hold games, and a queue that moves squads into a game with room. Terms (game, game slug, game layer, phase, player in game, squad, queue status) are defined in the glossaries listed in [`GLOSSARY-MAP.md`](../../../GLOSSARY-MAP.md), one per crate; use them exactly, in code and prose.

## Minimal by default

The idea is simple; keep the code and tests that small. Build the fewest features that deliver the behaviour asked for, and let state live in components a caller can read (`Queued`, `Playing`) rather than adding report messages, configuration, reasons or extra system sets. Test each behaviour once, at the lowest level that shows it. Propose anything beyond that to the user before writing it.

## Phase-typed design

The library's core idea: a game's phase is a **type**, so the compiler rejects code that acts on a game in the wrong phase. Each phase is a zero-sized marker component (`Waiting`, `Running`, `Ending`), and a game layer holds exactly one.

- Select games by phase in the query type: `Query<Game<Waiting>>`. A system handles one phase; write a generic system over `P: GamePhase` when the logic is the same in every phase.
- Put a phase-only capability on that phase's item: `impl GameItem<'_, '_, Waiting> { fn free_slots(..) }`. A capability that only makes sense in one phase is unreachable from the others, with no runtime check.
- Move a game on with `commands.entity(layer).transition::<From, To>()`. The legal edges are the `Next` impls in `transition.rs`; adding an edge is adding an impl. `transition` is a no-op when the game already left `From`, so concurrent systems never skip a phase.
- The phase set is sealed in `phase.rs`, because the queue relies on `Waiting` meaning "takes players". Mini-game-specific stages belong in their own marker components inside `Running`.

## Invariants

- **One phase per layer.** The `keep_only` insert hook removes the other phases, so even a hand-inserted marker keeps the invariant.
- **One game per player.** `Playing(layer)` is a Bevy relationship whose target `GamePlayers` Bevy keeps in sync; despawning a layer strips `Playing` from its players and keeps the players.
- **Inserting `Playing` moves the player into the game layer** (`enter.rs`): it repoints `EntityLayerId` and `VisibleChunkLayer` and swaps the old layer for the game layer in `VisibleEntityLayers` (other visible layers stay). ChunkEdge then sends the respawn. Use ChunkEdge's vocabulary: a world is a *layer*.
- **The library owns membership and layers, nothing more.** Positions, looks and spawn points belong to ChunkEdge (`Position`, `Look`, the client's `RespawnPosition`) and are set by the game. Before adding a concept, check whether ChunkEdge already has it, and reuse it.
- **Queued or in a game, never both.** A squad with a player in a game can't queue, and the queue removes `Queued` as it inserts `Playing`.
- **Capacity is advisory.** Whoever adds a player checks `free_slots()` of the `Game<Waiting>` first; `minigames_game` counts over-capacity layers as zero free slots.
- **Slugs are valid by construction.** Known games are `const X: GameSlug = GameSlug::from_static("x")`, checked at build time; runtime input goes through `GameSlug::new`.

## ChunkEdge and Bevy 0.19 gotchas

- ChunkEdge comes from git, `ChunkEdge/ChunkEdge` branch `main`; the crates.io `chunkedge` is an empty placeholder. `warden` uses a fork's `dev` branch, so the two can't share a server binary.
- Import Bevy through ChunkEdge: `chunkedge::prelude::*`, and `chunkedge::prelude::bevy_ecs::…` for paths outside the prelude. No crate here depends on `bevy_ecs` directly.
- A `#[derive(QueryData)]` field can't be a filter: `With<P>` fails to compile. Use a private `&'static P` field; markers are zero-sized.
- A `RelationshipTarget` is absent while empty, so read `GamePlayers` as `Option<&GamePlayers>`.
- ChunkEdge has no join or leave events. A join is `Added<Client>`; a disconnect removes `Client`, and `despawn_disconnected_clients` (which the app adds) marks the entity `Despawned`, despawned in `Last`. React with an `On<Remove, Client>` observer or `RemovedComponents<Client>`.
- Cross-crate communication uses Messages (`#[derive(Message)]`, `MessageReader`/`MessageWriter`), as ChunkEdge does.

## Layout

- Workspace crates under `crates/`: `minigames_game` (shared types), one crate per feature (`minigames_queue`), and `minigames` as the umbrella re-exporting them behind `MinigamesPlugin`.
- One responsibility per file. `lib.rs` holds only crate docs, `mod` lines and re-exports.
- Every public item is exported through `minigames::prelude`; the `minigames-prelude` skill has the rules. How a game uses the library is in the `minigames-game` skill; update it when the public API changes.
- Every stateful object has a state machine in `crates/<crate>/docs/<object>.md`: a mermaid diagram and a table of each edge and when it fires, linked from the crate's glossary. Change it in the same change as the behaviour, and add one for each new stateful object.
- Unit tests live in one `#[cfg(test)] mod tests` per crate: `src/tests/mod.rs` holds the shared fixtures, and `src/tests/<module>.rs` tests `src/<module>.rs`, with sentence-like names (`a_started_game_has_no_free_slots`), on a bare Bevy `World` or `App`. Source files carry production code only.
- End-to-end tests with real ChunkEdge mock clients live only in `crates/minigames/tests/`; share the fixture in `tests/common/mod.rs` (`app()`, `lobby`, `game_layer`, `connect`).
- Prove a compile-time guarantee with a `compile_fail` doctest beside a passing one, as `GameSlug::from_static` and `transition` do.

## Done

The work is done when `nix flake check` passes (clippy `--deny warnings`, every test, treefmt) and the glossary of the crate it touches names every new term.
