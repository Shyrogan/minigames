# Glossary

**Game**: one match of a mini game, held by a game layer. Not the kind of mini game: "a game of bedwars", never "bedwars is a game". In code, the `Game` query reads it.

**Game slug**: names a kind of mini game, such as `bedwars` or `capture-the-flag`: lowercase letters and digits joined by single dashes. `GameSlug`.

**Game layer**: a ChunkEdge layer entity (`LayerBundle`) marked with `GameLayer`, holding exactly one game.

**Layer spawn**: where players arrive in a layer and respawn while in it. `LayerSpawn`, a component on the layer entity, game or not.

**Capacity**: the most players a game layer holds. Nothing enforces it; whoever adds a player checks the free slots of a `Game<Waiting>` first.

**Game phase**: where a game is in its lifecycle: `Waiting` for players, `Running`, or `Ending`, each a marker component, exactly one per game layer. Only a waiting game has free slots. A game moves on with `transition::<From, To>()`, which only compiles for Waiting→Running, Waiting→Ending and Running→Ending.

**Player in game**: a client entity with `InGame(layer)`. A player is in at most one game. Inserting `InGame` moves the player into the game layer, at its layer spawn.

**Party**: players who queue together and always enter the same game. A solo player is a party of one.

**Queue entry**: one party waiting for a game of a given slug: an entity with `QueueEntry` and `EntryMembers`, each member carrying `Queued`. Entries are served in ticket order, first come first served.
