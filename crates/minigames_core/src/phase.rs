//! The phases a game goes through, as one marker component per phase.
//!
//! A game world holds exactly one of [`Waiting`], [`Running`] and [`Ending`],
//! so systems pick the games they handle by type, `Query<Game<Running>>`, and
//! move a game on with [`GamePhaseCommands::transition`](crate::GamePhaseCommands::transition),
//! which only compiles for the transitions [`Next`](crate::Next) allows.

use chunkedge::prelude::bevy_ecs::lifecycle::HookContext;
use chunkedge::prelude::bevy_ecs::world::DeferredWorld;
use chunkedge::prelude::*;

/// Gathering players before the match starts. The only phase that takes new
/// players.
#[derive(Component, Clone, Copy, Default, Debug)]
#[component(on_insert = keep_only::<Waiting>)]
pub struct Waiting;

/// The match is on.
#[derive(Component, Clone, Copy, Default, Debug)]
#[component(on_insert = keep_only::<Running>)]
pub struct Running;

/// The match is over and the world is being wrapped up.
#[derive(Component, Clone, Copy, Default, Debug)]
#[component(on_insert = keep_only::<Ending>)]
pub struct Ending;

/// One of [`Waiting`], [`Running`] and [`Ending`]. Sealed: the queue and
/// placement rely on this exact set.
pub trait GamePhase: Component + Default + sealed::Sealed {
    /// The other phases, removed when this one is inserted.
    type Others: Bundle;
}

impl GamePhase for Waiting {
    type Others = (Running, Ending);
}

impl GamePhase for Running {
    type Others = (Waiting, Ending);
}

impl GamePhase for Ending {
    type Others = (Waiting, Running);
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Waiting {}
    impl Sealed for super::Running {}
    impl Sealed for super::Ending {}
}

/// Keeps a world in a single phase, even when a phase is inserted by hand.
fn keep_only<P: GamePhase>(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .remove::<P::Others>();
}
