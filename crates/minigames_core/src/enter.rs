//! Entering a game: moving a player into its game layer.
//!
//! The move is the one ChunkEdge's `chunk_layer_switching` test makes:
//! pointing `EntityLayerId`, `VisibleChunkLayer` and `VisibleEntityLayers` at
//! the new layer. ChunkEdge then sends the respawn and the new layer's chunks,
//! and teleports the client to its new `Position`.

use chunkedge::log::warn;
use chunkedge::prelude::*;

use crate::layer::GameLayer;
use crate::players::InGame;
use crate::spawn::LayerSpawn;

/// The layer components a client moves with.
type LayerComponents = (
    &'static mut EntityLayerId,
    &'static mut VisibleChunkLayer,
    &'static mut VisibleEntityLayers,
    &'static mut Position,
    &'static mut Look,
    Option<&'static mut RespawnPosition>,
);

/// Moves a player into the game layer it just entered, at the layer's [`LayerSpawn`].
///
/// In `VisibleEntityLayers`, only the layer the player lived in is swapped
/// for the game layer; other visible layers, such as a scoreboard layer,
/// stay.
pub(crate) fn enter_game(
    insert: On<Insert, InGame>,
    in_game: Query<&InGame>,
    layers: Query<&LayerSpawn, With<GameLayer>>,
    mut players: Query<LayerComponents>,
) {
    let player = insert.entity;
    let Ok(&InGame(layer)) = in_game.get(player) else {
        return;
    };
    let Ok(&spawn) = layers.get(layer) else {
        warn!("{player} entered {layer}, which isn't a game layer with a LayerSpawn");
        return;
    };
    // Plain entities can be in a game too, in tests; only clients move.
    let Ok((mut entity_layer, mut chunk_layer, mut visible, mut pos, mut look, respawn)) =
        players.get_mut(player)
    else {
        return;
    };

    visible.0.remove(&entity_layer.0);
    visible.0.insert(layer);
    entity_layer.0 = layer;
    chunk_layer.0 = layer;
    pos.set(spawn.pos);
    look.yaw = spawn.yaw;
    look.pitch = spawn.pitch;
    if let Some(mut respawn) = respawn {
        respawn.pos = spawn.pos.into();
        respawn.yaw = spawn.yaw;
        respawn.pitch = spawn.pitch;
    }
}
