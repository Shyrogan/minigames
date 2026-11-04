//! Moving a game from one phase to the next, limited to the legal edges.

use chunkedge::prelude::*;

use crate::phase::{Ending, GamePhase, Running, Waiting};

/// `Self` may move on to `To`.
pub trait Next<To: GamePhase>: GamePhase {}

impl Next<Running> for Waiting {}
/// A match cancelled before it started.
impl Next<Ending> for Waiting {}
impl Next<Ending> for Running {}

/// Moves game worlds from one phase to the next.
pub trait GamePhaseCommands {
    /// Moves the game from `From` to `To`. Does nothing if the game is no
    /// longer in `From` when the command applies, so two systems racing to
    /// move the same game on don't skip a phase.
    ///
    /// Only the edges [`Next`] allows compile:
    ///
    /// ```
    /// # use chunkedge::prelude::*;
    /// # use minigames_core::{GamePhaseCommands, Running, Waiting};
    /// fn start(mut commands: Commands, game: Entity) {
    ///     commands.entity(game).transition::<Waiting, Running>();
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// # use chunkedge::prelude::*;
    /// # use minigames_core::{Ending, GamePhaseCommands, Waiting};
    /// fn reopen(mut commands: Commands, game: Entity) {
    ///     commands.entity(game).transition::<Ending, Waiting>();
    /// }
    /// ```
    fn transition<From: Next<To>, To: GamePhase>(&mut self) -> &mut Self;
}

impl GamePhaseCommands for EntityCommands<'_> {
    fn transition<From: Next<To>, To: GamePhase>(&mut self) -> &mut Self {
        self.queue(|mut world: EntityWorldMut| {
            if world.contains::<From>() {
                world.insert(To::default());
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{players, transition, world_of};

    #[test]
    fn a_game_is_in_one_phase_at_a_time() {
        let mut world = World::new();
        let arena = world_of(&mut world, 2);

        transition::<Waiting, Running>(&mut world, arena);
        assert_eq!(players::<Waiting>(&mut world, arena), None);
        assert_eq!(players::<Running>(&mut world, arena), Some(0));

        transition::<Running, Ending>(&mut world, arena);
        assert_eq!(players::<Running>(&mut world, arena), None);
        assert_eq!(players::<Ending>(&mut world, arena), Some(0));

        // Inserting a phase by hand still drops the current one.
        world.entity_mut(arena).insert(Waiting);
        world.flush();
        assert_eq!(players::<Ending>(&mut world, arena), None);
        assert_eq!(players::<Waiting>(&mut world, arena), Some(0));
    }

    #[test]
    fn a_transition_from_another_phase_does_nothing() {
        let mut world = World::new();
        let arena = world_of(&mut world, 2);
        transition::<Running, Ending>(&mut world, arena);
        assert_eq!(players::<Waiting>(&mut world, arena), Some(0));
        assert_eq!(players::<Ending>(&mut world, arena), None);
    }
}
