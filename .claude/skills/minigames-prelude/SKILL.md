---
name: minigames-prelude
description: Rules for `minigames::prelude`, shaped like ChunkEdge's prelude. Load when adding, renaming or removing a public item or crate in this repo, or writing imports in tests, examples or docs.
---

# minigames prelude

Downstream code reaches every `minigames` item it names through two glob imports, one per library:

```rust
use chunkedge::prelude::*;
use minigames::prelude::*;
```

The shape copies ChunkEdge's `pub mod prelude` in its `src/lib.rs` (read it with `git -C ~/Documents/personal-projects/ChunkEdge show upstream/main:src/lib.rs`).

## Rules

- **One prelude, in the umbrella crate.** It is `pub mod prelude` in `crates/minigames/src/lib.rs`. Feature crates (`minigames_game`, the queue, …) export through it and have none of their own.
- **Own items only.** The prelude lists `minigames` items. ChunkEdge and Bevy items come from `chunkedge::prelude`, which the caller globs beside it.
- **Named, explicit lists.** One `pub use <crate>::{…};` per source crate, naming each item, as ChunkEdge does. Each new feature crate adds its own line.
- **What goes in:** every item downstream code names: components and markers, `QueryData` types together with their `…Item`, messages, plugins, traits used as bounds or implemented downstream (`GamePhase`, `Next`, `MiniGame`), error types (`InvalidGameSlug`).
- **Method-only extension traits go in as `_`:** `pub use minigames_game::GamePhaseCommands as _;`, like ChunkEdge's `SendMessage as _`. This brings the methods (`transition`) into scope while leaving the name free.
- **Plugins come from the umbrella:** `pub use super::MinigamesPlugin;` at the end of the module.
- **Less common items** stay reachable by path: `minigames_game` items at the crate root (`pub use minigames_game::*`), and each later feature crate as a module, `pub use minigames_queue as queue;`, mirroring `pub use chunkedge_inventory as inventory`.

## Imports by location

- **Feature crates** import siblings by crate path (`crate::phase::Waiting`, `minigames_game::GameSlug`) and Bevy through `chunkedge::prelude::*`. The umbrella depends on them, so its prelude stays out of their reach.
- **The umbrella's tests, doctests and examples** use the two globs above, plus explicit paths for ChunkEdge items outside its prelude (`chunkedge::testing`, `chunkedge::keepalive::KeepaliveSettings`, `chunkedge::ServerSettings`).

## Done

A change that adds, renames or removes a public item is done when:

- The prelude lists every item downstream code names, and nothing that has been removed.
- The crate-level doctest in `crates/minigames/src/lib.rs` still compiles with only the two glob imports. If the change adds a capability downstream code uses directly, the doctest shows it.
- `nix develop -c cargo test --workspace --doc` passes.
