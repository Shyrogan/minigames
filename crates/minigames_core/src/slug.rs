//! Names of the kinds of mini game.

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Names a kind of mini game, such as `bedwars` or `capture-the-flag`.
///
/// A slug is one or more groups of lowercase ASCII letters and digits, joined
/// by single dashes: `^[a-z0-9]+(-[a-z0-9]+)*$`. Every `GameSlug` holds a
/// valid slug.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameSlug(Cow<'static, str>);

impl GameSlug {
    /// Checks a slug from runtime input, such as a config file or a command.
    pub fn new(slug: impl Into<Cow<'static, str>>) -> Result<Self, InvalidGameSlug> {
        let slug = slug.into();
        if is_valid(&slug) {
            Ok(Self(slug))
        } else {
            Err(InvalidGameSlug(slug.into_owned()))
        }
    }

    /// A slug known at build time. Panics if it isn't valid, which is a
    /// compile error when used in a `const`:
    ///
    /// ```
    /// # use minigames_core::GameSlug;
    /// const BEDWARS: GameSlug = GameSlug::from_static("bedwars");
    /// ```
    ///
    /// ```compile_fail
    /// # use minigames_core::GameSlug;
    /// const BEDWARS: GameSlug = GameSlug::from_static("Bedwars");
    /// ```
    pub const fn from_static(slug: &'static str) -> Self {
        assert!(
            is_valid(slug),
            "a game slug is lowercase letters and digits joined by single dashes"
        );
        Self(Cow::Borrowed(slug))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for GameSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for GameSlug {
    type Error = InvalidGameSlug;

    fn try_from(slug: String) -> Result<Self, Self::Error> {
        Self::new(slug)
    }
}

impl FromStr for GameSlug {
    type Err = InvalidGameSlug;

    fn from_str(slug: &str) -> Result<Self, Self::Err> {
        Self::new(slug.to_owned())
    }
}

/// A string rejected as a [`GameSlug`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidGameSlug(pub String);

impl fmt::Display for InvalidGameSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} is not a game slug: use lowercase letters and digits joined by single dashes",
            self.0
        )
    }
}

impl Error for InvalidGameSlug {}

/// Whether `slug` matches `^[a-z0-9]+(-[a-z0-9]+)*$`.
const fn is_valid(slug: &str) -> bool {
    let bytes = slug.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let mut i = 0;
    // A dash is only allowed right after a letter or digit, so this also
    // rejects a leading dash and two dashes in a row.
    let mut after_dash = true;
    while i < bytes.len() {
        match bytes[i] {
            b'a'..=b'z' | b'0'..=b'9' => after_dash = false,
            b'-' if !after_dash => after_dash = true,
            _ => return false,
        }
        i += 1;
    }
    !after_dash
}
