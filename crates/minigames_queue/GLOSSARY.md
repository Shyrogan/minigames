# Queue

Squads lining up for a mini game until a waiting game takes them. State machines: [squad](docs/squad.md), [queue status](docs/queue-status.md).

## Language

**Squad**: 1..n players in line together for a mini game. They always enter the same game. A squad is never empty and never split: when one of its players leaves the queue, the whole squad is free again. Squads are served in ticket order, first come first served. _Avoid_: party, group, team, solo player, queue entry.

**Squadmate**: one of the players of a squad. _Avoid_: member.

**Queue status**: where a player stands with the queue: free, queued for a mini game, or busy. Exactly one per player. _Avoid_: waiter, waiting (for a player; a waiting game takes players).

**Free**: the queue status of a player neither queued nor in a game.

**Queued**: the queue status of a player whose squad is in line.

**Busy**: the queue status of a player in a game. Being in a game and being busy are the same thing.
