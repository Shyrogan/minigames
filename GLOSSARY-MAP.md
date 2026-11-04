# Glossary Map

## Contexts

- [Game](./crates/minigames_game/GLOSSARY.md): mini games, games, game layers and their phases, and which players are in which game
- [Queue](./crates/minigames_queue/GLOSSARY.md): squads and queue statuses, until a waiting game takes them

## Relationships

- **Queue → Game**: the queue places queued squads into a `Game<Waiting>` of their mini game with enough free slots, by inserting `Playing`.
- **Queue ↔ Game**: busy means in a game. A player becomes busy when it gains `Playing` and free again when it loses it, whoever moved it.
- **Shared**: a mini game is a type implementing `MiniGame` (defined in Game), whose slug names it. Game layers record the slug; the queue is typed by the mini game and reads joins by slug. See [ADR 0001](./docs/adr/0001-mini-games-are-types.md).
