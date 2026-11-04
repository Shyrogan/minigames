//! Squads: players in line together for a mini game, and the queued status
//! of their players.

use std::marker::PhantomData;

use chunkedge::prelude::*;
use minigames_game::MiniGame;

use crate::status::{Busy, Free};

/// A squad in line for a game of `G`, on its own entity. Squads are served in
/// ascending [`ticket`](Self::ticket) order.
#[derive(Component)]
pub struct Squad<G: MiniGame> {
    ticket: u64,
    game: PhantomData<fn() -> G>,
}

impl<G: MiniGame> Squad<G> {
    pub(crate) fn new(ticket: u64) -> Self {
        Self {
            ticket,
            game: PhantomData,
        }
    }

    /// The squad's place in line: lower tickets are served first.
    pub fn ticket(&self) -> u64 {
        self.ticket
    }
}

/// The queue status of a player whose squad is in line for a game of `G`.
#[derive(Component)]
#[relationship(relationship_target = Squadmates<G>)]
pub struct Queued<G: MiniGame> {
    #[relationship]
    squad: Entity,
    game: PhantomData<fn() -> G>,
}

impl<G: MiniGame> Queued<G> {
    pub(crate) fn new(squad: Entity) -> Self {
        Self {
            squad,
            game: PhantomData,
        }
    }

    /// The squad the player waits in.
    pub fn squad(&self) -> Entity {
        self.squad
    }
}

/// The players of a squad, kept in sync by Bevy from [`Queued`].
#[derive(Component)]
#[relationship_target(relationship = Queued<G>)]
pub struct Squadmates<G: MiniGame> {
    #[relationship]
    players: Vec<Entity>,
    game: PhantomData<fn() -> G>,
}

impl<G: MiniGame> Squadmates<G> {
    pub fn players(&self) -> &[Entity] {
        &self.players
    }
}

/// Marks a squad the queue placed in a game, so its players entering the
/// game don't dissolve it.
#[derive(Component)]
pub(crate) struct Placed;

/// Hands out queue tickets, in increasing order.
#[derive(Resource, Default, Debug)]
pub(crate) struct QueueTickets(u64);

impl QueueTickets {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

/// Puts `players` in line for `G` as a new squad.
pub(crate) fn spawn_squad<G: MiniGame>(commands: &mut Commands, ticket: u64, players: &[Entity]) {
    let squad = commands.spawn(Squad::<G>::new(ticket)).id();
    for &player in players {
        commands.entity(player).try_insert(Queued::<G>::new(squad));
    }
}

/// Queued drops free and busy.
pub(crate) fn keep_only_queued<G: MiniGame>(insert: On<Insert, Queued<G>>, mut commands: Commands) {
    commands.entity(insert.entity).try_remove::<(Free, Busy)>();
}

/// Free and busy drop queued.
pub(crate) fn drop_queued<G: MiniGame>(insert: On<Insert, (Free, Busy)>, mut commands: Commands) {
    commands.entity(insert.entity).try_remove::<Queued<G>>();
}

/// A squad is never split: when one of its players leaves the queue other
/// than by the queue placing the squad, every player not in a game is free.
pub(crate) fn dissolve_squad<G: MiniGame>(
    remove: On<Remove, Queued<G>>,
    queued: Query<&Queued<G>>,
    squads: Query<&Squadmates<G>, Without<Placed>>,
    busy: Query<(), With<Busy>>,
    mut commands: Commands,
) {
    let Ok(queued) = queued.get(remove.entity) else {
        return;
    };
    let Ok(squadmates) = squads.get(queued.squad) else {
        return;
    };
    for &player in squadmates.players() {
        if !busy.contains(player) {
            commands.entity(player).try_insert(Free);
        }
    }
}

/// Despawns a squad once its last player left: Bevy removes an emptied
/// `Squadmates`.
pub(crate) fn despawn_empty_squad<G: MiniGame>(
    remove: On<Remove, Squadmates<G>>,
    mut commands: Commands,
) {
    commands.entity(remove.entity).try_despawn();
}
