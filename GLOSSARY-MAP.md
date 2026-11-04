# Glossary Map

## Contexts

- [Game](./crates/minigames_game/GLOSSARY.md): mini games, games, game layers and their phases, and which players are in which game
- [Queue](./crates/minigames_queue/GLOSSARY.md): squads and queue statuses, until a waiting game takes them

## Relationships

- **Queue → Game**: the queue places queued squads into a `Game<Waiting>` with enough free slots, by inserting `Playing`.
- **Queue ↔ Game**: busy means in a game. A player becomes busy when it gains `Playing` and free again when it loses it, whoever moved it.
- **Shared**: a mini game type names its game slug; both contexts key games and queues by it.
