//! Moving a game from one phase to the next, limited to the legal edges.

use chunkedge::prelude::*;

use crate::phase::{Ending, GamePhase, Running, Waiting};

/// `Self` may move on to `To`.
pub trait Next<To: GamePhase>: GamePhase {}

impl Next<Running> for Waiting {}
/// A match cancelled before it started.
impl Next<Ending> for Waiting {}
impl Next<Ending> for Running {}

/// Moves games from one phase to the next.
pub trait GamePhaseCommands {
    /// Moves the game from `From` to `To`. Does nothing if the game is no
    /// longer in `From` when the command applies, so two systems racing to
    /// move the same game on don't skip a phase.
    ///
    /// Only the edges [`Next`] allows compile:
    ///
    /// ```
    /// # use chunkedge::prelude::*;
    /// # use minigames_game::{GamePhaseCommands, Running, Waiting};
    /// fn start(mut commands: Commands, game: Entity) {
    ///     commands.entity(game).transition::<Waiting, Running>();
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// # use chunkedge::prelude::*;
    /// # use minigames_game::{Ending, GamePhaseCommands, Waiting};
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
