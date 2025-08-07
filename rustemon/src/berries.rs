//! Berries endpoints group

/// Berries are small fruits that can provide HP and status condition restoration,
/// stat enhancement, and even damage negation when eaten by Pokémon.
/// Check out [Bulbapedia](http:///bulbapedia.bulbagarden.net/wiki/Berry) for greater detail.
pub mod berry {
    crate::endpoint!(crate::model::berries::Berry; for "berry");
}

/// Berries can be soft or hard.
/// Check out [Bulbapedia](https:///bulbapedia.bulbagarden.net/wiki/Category:Berries_by_firmness) for greater detail.
pub mod berry_firmness {
    crate::endpoint!(crate::model::berries::BerryFirmness; for "berry-firmness");
}

/// Flavors determine whether a Pokémon will benefit or suffer from eating a berry based on their nature.
/// Check out [Bulbapedia](http:///bulbapedia.bulbagarden.net/wiki/Flavor) for greater detail.
pub mod berry_flavor {
    crate::endpoint!(crate::model::berries::BerryFlavor; for "berry-flavor");
}
