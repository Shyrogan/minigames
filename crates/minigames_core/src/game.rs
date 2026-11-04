//! Reading the games of one phase: `Query<Game<Waiting>>`.

use chunkedge::prelude::bevy_ecs::query::QueryData;
use chunkedge::prelude::*;

use crate::phase::{GamePhase, Waiting};
use crate::players::GamePlayers;
use crate::world::GameWorld;

/// A world's game in phase `P`: what it plays and its players. Only matches
/// worlds in `P`, so `Query<Game<Waiting>>` only sees games that take players.
#[derive(QueryData)]
pub struct Game<P: GamePhase> {
    pub world: &'static GameWorld,
    pub players: Option<&'static GamePlayers>,
    /// Restricts the query to phase `P`. A filter can't be a field of a
    /// `QueryData`, and reading the zero-sized marker costs nothing.
    phase: &'static P,
}

impl<P: GamePhase> GameItem<'_, '_, P> {
    pub fn player_count(&self) -> usize {
        self.players.map_or(0, |players| players.players().len())
    }
}

impl GameItem<'_, '_, Waiting> {
    /// How many more players the game takes. Only a waiting game takes
    /// players, so only it has free slots.
    pub fn free_slots(&self) -> usize {
        self.world.capacity.saturating_sub(self.player_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase::Running;
    use crate::players::InGame;
    use crate::test_support::{free_slots, players, transition, world_of};

    #[test]
    fn a_new_game_waits_for_players() {
        let mut world = World::new();
        let arena = world_of(&mut world, 2);
        assert_eq!(free_slots(&mut world, arena), Some(2));
        assert_eq!(players::<Running>(&mut world, arena), None);
    }

    #[test]
    fn free_slots_follow_players() {
        let mut world = World::new();
        let arena = world_of(&mut world, 2);
        world.spawn(InGame(arena));
        assert_eq!(free_slots(&mut world, arena), Some(1));
        // Over capacity, since nothing enforces it: no slot left, not an
        // underflow.
        world.spawn_batch([InGame(arena), InGame(arena)]);
        assert_eq!(players::<Waiting>(&mut world, arena), Some(3));
        assert_eq!(free_slots(&mut world, arena), Some(0));
    }

    #[test]
    fn a_started_game_has_no_free_slots() {
        let mut world = World::new();
        let arena = world_of(&mut world, 4);
        world.spawn(InGame(arena));

        transition::<Waiting, Running>(&mut world, arena);
        assert_eq!(free_slots(&mut world, arena), None);
        assert_eq!(players::<Running>(&mut world, arena), Some(1));
    }
}
