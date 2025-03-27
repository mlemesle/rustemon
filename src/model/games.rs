//! Games group models

use super::{
    locations::Region,
    moves::{Move, MoveLearnMethod},
    pokemon::{Ability, PokemonSpecies, Type},
    resource::{Description, Name, NamedApiResource},
};

/// [Generation official documentation]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Generation {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of abilities that were introduced in this generation.
    pub abilities: Vec<NamedApiResource<Ability>>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// The main region travelled in this generation.
    pub main_region: NamedApiResource<Region>,
    /// A list of moves that were introduced in this generation.
    pub moves: Vec<NamedApiResource<Move>>,
    /// A list of Pokémon species that were introduced in this generation.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
    /// A list of types that were introduced in this generation.
    pub types: Vec<NamedApiResource<Type>>,
    /// A list of version groups that were introduced in this generation.
    pub version_groups: Vec<NamedApiResource<VersionGroup>>,
}

/// [Pokedex official documentation](https:///pokeapi.co/docs/v2#pokedex)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Pokedex {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// Whether or not this Pokédex originated in the main series of the video games.
    pub is_main_series: bool,
    /// The description of this resource listed in different languages.
    pub descriptions: Vec<Description>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of Pokémon catalogued in this Pokédex and their indexes.
    pub pokemon_entries: Vec<PokemonEntry>,
    /// The region this Pokédex catalogues Pokémon for.
    pub region: Option<NamedApiResource<Region>>,
    /// A list of version groups this Pokédex is relevant to.
    pub version_groups: Vec<NamedApiResource<VersionGroup>>,
}

/// [PokemonEntry official documentation](https:///pokeapi.co/docs/v2#pokemonentry)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonEntry {
    /// The index of this Pokémon species entry within the Pokédex.
    pub entry_number: i64,
    /// The Pokémon species being encountered.
    pub pokemon_species: NamedApiResource<PokemonSpecies>,
}

/// [Version offcial documentation](https:///pokeapi.co/docs/v2#version)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Version {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// The version group this version belongs to.
    pub version_group: NamedApiResource<VersionGroup>,
}

/// [VersionGroup official documentation](https:///pokeapi.co/docs/v2#versiongroup)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VersionGroup {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// Order for sorting. Almost by date of release, except similar versions are grouped together.
    pub order: i64,
    /// The generation this version was introduced in.
    pub generation: NamedApiResource<Generation>,
    /// A list of methods in which Pokémon can learn moves in this version group.
    pub move_learn_methods: Vec<NamedApiResource<MoveLearnMethod>>,
    /// A list of Pokédexes introduces in this version group.
    pub pokedexes: Vec<NamedApiResource<Pokedex>>,
    /// A list of regions that can be visited in this version group.
    pub regions: Vec<NamedApiResource<Region>>,
    /// The versions this version group owns.
    pub versions: Vec<NamedApiResource<Version>>,
}
