//! Locations endpoints group

/// Locations that can be visited within the games. Locations make up sizable portions of regions,
/// like cities or routes.
pub mod location {
    crate::blocking::endpoint!(crate::model::locations::Location; for "location");
}

/// Location areas are sections of areas, such as floors in a building or cave.
/// Each area has its own set of possible Pokémon encounters.
pub mod location_area {
    crate::blocking::endpoint!(crate::model::locations::LocationArea; for "location-area");
}

/// Areas used for grouping Pokémon encounters in Pal Park.
/// They're like habitats that are specific to [Pal Park](https://bulbapedia.bulbagarden.net/wiki/Pal_Park).
pub mod pal_park_area {
    crate::blocking::endpoint!(crate::model::locations::PalParkArea; for "pal-park-area");
}

/// A region is an organized area of the Pokémon world.
/// Most often, the main difference between regions is the species of Pokémon that can be encountered within them.
pub mod region {
    crate::blocking::endpoint!(crate::model::locations::Region; for "region");
}
