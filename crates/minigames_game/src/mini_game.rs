//! Mini games: one type per kind of game.

use crate::slug::GameSlug;

/// A kind of game, such as bedwars, as a type named by its slug.
///
/// ```
/// # use minigames_game::{GameSlug, MiniGame};
/// struct Bedwars;
///
/// impl MiniGame for Bedwars {
///     const SLUG: GameSlug = GameSlug::from_static("bedwars");
/// }
/// ```
pub trait MiniGame: Send + Sync + 'static {
    const SLUG: GameSlug;
}
