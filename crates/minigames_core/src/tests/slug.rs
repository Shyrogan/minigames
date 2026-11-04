use crate::*;

const BEDWARS: GameSlug = GameSlug::from_static("bedwars");

#[test]
fn lowercase_words_joined_by_dashes_are_slugs() {
    for slug in ["bedwars", "capture-the-flag", "1v1", "a"] {
        assert_eq!(GameSlug::new(slug).unwrap().as_str(), slug);
    }
    assert_eq!(BEDWARS.as_str(), "bedwars");
}

#[test]
fn anything_else_is_rejected() {
    for slug in ["", "Bedwars", "-x", "x-", "-", "a--b", "a_b", "a b", "é"] {
        assert_eq!(GameSlug::new(slug), Err(InvalidGameSlug(slug.to_owned())));
    }
}

#[test]
fn parsing_goes_through_the_same_check() {
    assert_eq!("bedwars".parse::<GameSlug>(), Ok(BEDWARS));
    assert!("Bedwars".parse::<GameSlug>().is_err());
    assert!(GameSlug::try_from("x-".to_owned()).is_err());
}

#[test]
#[should_panic]
fn an_invalid_static_slug_panics() {
    GameSlug::from_static("Bedwars");
}
