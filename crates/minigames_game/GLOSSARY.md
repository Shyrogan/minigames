# Game

Games and the layers that hold them: what a game is, its phases, and which players are in it. State machines: [game phase](docs/game-phase.md), [player in game](docs/player-in-game.md).

## Language

**Mini game**: a kind of game, such as bedwars or capture the flag. One type per mini game, which names its game slug. _Avoid_: game (for the kind).

**Game**: one match of a mini game, held by a game layer. Not the kind of mini game: "a game of bedwars", never "bedwars is a game". In code, the `Game` query reads it.

**Game slug**: names a mini game, such as `bedwars` or `capture-the-flag`: lowercase letters and digits joined by single dashes. `GameSlug`.

**Game layer**: a ChunkEdge layer entity (`LayerBundle`) marked with `GameLayer`, holding exactly one game.

**Capacity**: the most players a game layer holds. Nothing enforces it; whoever adds a player checks the free slots of a `Game<Waiting>` first.

**Game phase**: where a game is in its lifecycle: `Waiting` for players, `Running`, or `Ending`, each a marker component, exactly one per game layer. Only a waiting game has free slots. A game moves on with `transition::<From, To>()`, which only compiles for Waiting→Running, Waiting→Ending and Running→Ending.

**Player in game**: a client entity with `Playing(layer)`. A player is in at most one game. Inserting `Playing` moves the player into the game layer; where it stands there is the game's choice.
