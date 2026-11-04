# Mini games are types

Each mini game is a type implementing `MiniGame`, whose `SLUG` names it, instead of only a `GameSlug` value. The library is phase-typed so the compiler rejects code acting on the wrong kind of thing, and a typed mini game carries that to players and squads: `Queued<Bedwars>` selects bedwars' queue in a query, and a squad is a `Squad<Bedwars>`. The cost is generic code: each mini game adds its own `GameQueuePlugin::<G>`, and game layers are built from a type with `GameLayer::new::<G>`.

## Considered Options

- **Slug values only** (`Queued { game: GameSlug }`): simpler and fully runtime, but a system can't select one mini game's players or squads by type, and every system filters on a value instead.
- **A generic `GameLayer<G>`**: fully typed layers, but `Game<P>` and every system over game layers would become generic, for no gain in the queue. Layers keep the slug, and `GameLayer::new::<G>` is the only way to build one, so the slug always names a mini game type.

## Consequences

`GameSlug` stays for runtime input: `QueueMessage::Join` names its mini game by slug, so a command such as `/queue bedwars` needs no type, and each `GameQueuePlugin::<G>` handles the joins for `G::SLUG`. A slug no mini game was added for is ignored.
