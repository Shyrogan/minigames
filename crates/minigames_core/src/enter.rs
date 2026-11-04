//! Entering a game: moving a player into its game layer.
//!
//! The move is the one ChunkEdge's `chunk_layer_switching` test makes:
//! pointing `EntityLayerId`, `VisibleChunkLayer` and `VisibleEntityLayers` at
//! the new layer. ChunkEdge then sends the respawn and the new layer's chunks.
//! Where the player stands is the game's: ChunkEdge's `Position` and
//! `RespawnPosition`.

use chunkedge::log::warn;
use chunkedge::prelude::*;

use crate::layer::GameLayer;
use crate::players::Playing;

/// Moves a player into the game layer it just entered.
///
/// In `VisibleEntityLayers`, only the layer the player lived in is swapped
/// for the game layer; other visible layers, such as a scoreboard layer,
/// stay.
pub(crate) fn enter_game(
    insert: On<Insert, Playing>,
    playing: Query<&Playing>,
    layers: Query<(), With<GameLayer>>,
    mut players: Query<(
        &mut EntityLayerId,
        &mut VisibleChunkLayer,
        &mut VisibleEntityLayers,
    )>,
) {
    let player = insert.entity;
    let Ok(&Playing(layer)) = playing.get(player) else {
        return;
    };
    if !layers.contains(layer) {
        warn!("{player} entered {layer}, which isn't a game layer");
        return;
    }
    // Plain entities can be in a game too, in tests; only clients move.
    let Ok((mut entity_layer, mut chunk_layer, mut visible)) = players.get_mut(player) else {
        return;
    };

    visible.0.remove(&entity_layer.0);
    visible.0.insert(layer);
    entity_layer.0 = layer;
    chunk_layer.0 = layer;
}
