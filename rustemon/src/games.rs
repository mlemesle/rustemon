//! Games endpoints group

/// A generation is a grouping of the Pokémon games that separates them based on the Pokémon they include.
///
/// In each generation, a new set of Pokémon, Moves, Abilities and Types that did not exist in the previous generation are released.
pub mod generation {
    crate::endpoint!(crate::model::games::Generation; for "generation");
}

/// A Pokédex is a handheld electronic encyclopedia device; one which is capable of recording and retaining information
/// of the various Pokémon in a given region with the exception of the national dex and some smaller dexes
/// related to portions of a region.
/// See [Bulbapedia](http:///bulbapedia.bulbagarden.net/wiki/Pokedex) for greater detail.
pub mod pokedex {
    crate::endpoint!(crate::model::games::Pokedex; for "pokedex");
}

/// Versions of the games, e.g., Red, Blue or Yellow.
pub mod version {
    crate::endpoint!(crate::model::games::Version; for "version");
}

/// Version groups categorize highly similar versions of the games.
pub mod version_group {
    crate::endpoint!(crate::model::games::VersionGroup; for "version-group");
}
