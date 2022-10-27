//! Pokemon endpoints group

/// Abilities provide passive effects for Pokémon in battle or in the overworld.
/// Pokémon have multiple possible abilities but can have only one ability at a time.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Ability) for greater detail.
pub mod ability {
    crate::endpoint!(crate::model::pokemon::Ability; for "ability");
}

/// Characteristics indicate which stat contains a Pokémon's highest IV.
/// A Pokémon's Characteristic is determined by the remainder of its highest IV divided by 5 (gene_modulo).
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Characteristic) for greater detail.
pub mod characteristic {
    crate::endpoint!(crate::model::pokemon::Characteristic; for "characteristic");
}

/// Egg Groups are categories which determine which Pokémon are able to interbreed.
/// Pokémon may belong to either one or two Egg Groups.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Egg_Group) for greater detail.
pub mod egg_group {
    crate::endpoint!(crate::model::pokemon::EggGroup; for "egg-group");
}

/// Genders were introduced in Generation II for the purposes of breeding Pokémon but can also result
/// in visual differences or even different evolutionary lines.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Gender) for greater detail.
pub mod gender {
    crate::endpoint!(crate::model::pokemon::Gender; for "gender");
}

/// Growth rates are the speed with which Pokémon gain levels through experience.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Experience) for greater detail.
pub mod growth_rate {
    crate::endpoint!(crate::model::pokemon::GrowthRate; for "growth-rate");
}

/// Natures influence how a Pokémon's stats grow. See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Nature) for greater detail.
pub mod nature {
    crate::endpoint!(crate::model::pokemon::Nature; for "nature");
}

/// Pokeathlon Stats are different attributes of a Pokémon's performance in Pokéathlons.
/// In Pokéathlons, competitions happen on different courses; one for each of the different Pokéathlon stats.
/// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Pokéathlon) for greater detail.
pub mod pokeathlon_stat {
    crate::endpoint!(crate::model::pokemon::PokeathlonStat; for "pokeathlon-stat");
}

/// Pokémon are the creatures that inhabit the world of the Pokémon games.
/// They can be caught using Pokéballs and trained by battling with other Pokémon.
/// Each Pokémon belongs to a specific species but may take on a variant which makes it differ
/// from other Pokémon of the same species, such as base stats, available abilities and typings.
/// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Pokémon_(species)) for greater detail.
///
/// Pokémon Location Areas are areas where Pokémon can be found.
#[allow(clippy::module_inception)]
pub mod pokemon {
    crate::endpoint!(crate::model::pokemon::Pokemon; for "pokemon"; with (encounters, Vec<crate::model::pokemon::LocationAreaEncounter>));
}

/// Colors used for sorting Pokémon in a Pokédex. The color listed in the Pokédex is usually the color
/// most apparent or covering each Pokémon's body. No orange category exists; Pokémon that are primarily
/// orange are listed as red or brown.
pub mod pokemon_color {
    crate::endpoint!(crate::model::pokemon::PokemonColor; for "pokemon-color");
}

/// Some Pokémon may appear in one of multiple, visually different forms. These differences are purely cosmetic.
/// For variations within a Pokémon species, which do differ in more than just visuals,
/// the 'Pokémon' entity is used to represent such a variety.
pub mod pokemon_form {
    crate::endpoint!(crate::model::pokemon::PokemonForm; for "pokemon-form");
}

/// Habitats are generally different terrain Pokémon can be found in but can also be
/// areas designated for rare or legendary Pokémon.
pub mod pokemon_habitat {
    crate::endpoint!(crate::model::pokemon::PokemonHabitat; for "pokemon-habitat");
}

/// Shapes used for sorting Pokémon in a Pokédex.
pub mod pokemon_shape {
    crate::endpoint!(crate::model::pokemon::PokemonShape; for "pokemon-shape");
}

/// A Pokémon Species forms the basis for at least one Pokémon. Attributes of a Pokémon species are shared
/// across all varieties of Pokémon within the species. A good example is Wormadam; Wormadam is the species
/// which can be found in three different varieties, Wormadam-Trash, Wormadam-Sandy and Wormadam-Plant.
pub mod pokemon_species {
    crate::endpoint!(crate::model::pokemon::PokemonSpecies; for "pokemon-species");
}

/// Stats determine certain aspects of battles. Each Pokémon has a value for each stat which
/// grows as they gain levels and can be altered momentarily by effects in battles.
pub mod stat {
    crate::endpoint!(crate::model::pokemon::Stat; for "stat");
}

/// Types are properties for Pokémon and their moves. Each type has three properties:
/// which types of Pokémon it is super effective against, which types of Pokémon it is not very effective against,
/// and which types of Pokémon it is completely ineffective against.
pub mod type_ {
    crate::endpoint!(crate::model::pokemon::Type; for "type");
}
