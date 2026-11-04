# Glossary

**Game**: one match of a mini game, held by a game world. Not the kind of mini game: "a game of bedwars", never "bedwars is a game". In code, the `Game` query reads it.

**Game slug**: names a kind of mini game, such as `bedwars` or `capture-the-flag`: lowercase letters and digits joined by single dashes. `GameSlug`.

**Game world**: a ChunkEdge layer entity (`LayerBundle`) marked with `GameWorld`, holding exactly one game.

**Capacity**: the most players a game world holds. Nothing enforces it; whoever adds a player checks the free slots of a `Game<Waiting>` first.

**Game phase**: where a game is in its lifecycle: `Waiting` for players, `Running`, or `Ending`, each a marker component, exactly one per game world. Only a waiting game has free slots. A game moves on with `transition::<From, To>()`, which only compiles for Waiting→Running, Waiting→Ending and Running→Ending.

**Player in game**: a client entity with `InGame(world)`. A player is in at most one game. Being in a game is membership only; moving the player into the world's layers is a separate step.
