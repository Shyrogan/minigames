//! Which players are in which game world, as a Bevy relationship.

use chunkedge::prelude::*;

/// The [`GameWorld`](crate::GameWorld) a player is in.
///
/// This only records membership. It doesn't move the player into the world's
/// layers (`EntityLayerId`, `VisibleChunkLayer`, `VisibleEntityLayers`,
/// `Position`); whoever inserts it switches those too.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[relationship(relationship_target = GamePlayers)]
pub struct InGame(pub Entity);

/// The players in a game world, kept in sync by Bevy from [`InGame`]. Absent
/// while the world is empty.
///
/// Despawning the world removes `InGame` from its players but keeps them.
#[derive(Component, Debug)]
#[relationship_target(relationship = InGame)]
pub struct GamePlayers(Vec<Entity>);

impl GamePlayers {
    pub fn players(&self) -> &[Entity] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{free_slots, world_of};

    #[test]
    fn moving_a_player_updates_both_games() {
        let mut world = World::new();
        let from = world_of(&mut world, 4);
        let to = world_of(&mut world, 4);
        let player = world.spawn(InGame(from)).id();

        world.entity_mut(player).insert(InGame(to));
        assert_eq!(free_slots(&mut world, from), Some(4));
        assert_eq!(world.get::<GamePlayers>(to).unwrap().players(), [player]);
    }

    #[test]
    fn despawning_a_world_keeps_its_players() {
        let mut world = World::new();
        let arena = world_of(&mut world, 4);
        let player = world.spawn(InGame(arena)).id();

        world.despawn(arena);
        assert!(world.get_entity(player).is_ok());
        assert!(world.get::<InGame>(player).is_none());
    }

    #[test]
    fn a_despawned_player_leaves_its_game() {
        let mut world = World::new();
        let arena = world_of(&mut world, 4);
        let player = world.spawn(InGame(arena)).id();

        world.despawn(player);
        assert_eq!(free_slots(&mut world, arena), Some(4));
    }
}
