//! Games group models

use super::resource::{Description, Name, NamedApiResource};

/// [Generation official documentation]
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Generation {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of abilities that were introduced in this generation.
    pub abilities: Option<Vec<NamedApiResource>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The main region travelled in this generation.
    pub main_region: Option<NamedApiResource>,
    /// A list of moves that were introduced in this generation.
    pub moves: Option<Vec<NamedApiResource>>,
    /// A list of Pokémon species that were introduced in this generation.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
    /// A list of types that were introduced in this generation.
    pub types: Option<Vec<NamedApiResource>>,
    /// A list of version groups that were introduced in this generation.
    pub version_groups: Option<Vec<NamedApiResource>>,
}

/// [Pokedex official documentation](https:///pokeapi.co/docs/v2#pokedex)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pokedex {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// Whether or not this Pokédex originated in the main series of the video games.
    pub is_main_series: Option<bool>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of Pokémon catalogued in this Pokédex and their indexes.
    pub pokemon_entries: Option<Vec<PokemonEntry>>,
    /// The region this Pokédex catalogues Pokémon for.
    pub region: Option<NamedApiResource>,
    /// A list of version groups this Pokédex is relevant to.
    pub version_groups: Option<Vec<NamedApiResource>>,
}

/// [PokemonEntry official documentation](https:///pokeapi.co/docs/v2#pokemonentry)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonEntry {
    /// The index of this Pokémon species entry within the Pokédex.
    pub entry_number: Option<i64>,
    /// The Pokémon species being encountered.
    pub pokemon_species: Option<NamedApiResource>,
}

/// [Version offcial documentation](https:///pokeapi.co/docs/v2#version)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Version {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The version group this version belongs to.
    pub version_group: Option<NamedApiResource>,
}

/// [VersionGroup official documentation](https:///pokeapi.co/docs/v2#versiongroup)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VersionGroup {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// Order for sorting. Almost by date of release, except similar versions are grouped together.
    pub order: Option<i64>,
    /// The generation this version was introduced in.
    pub generation: Option<NamedApiResource>,
    /// A list of methods in which Pokémon can learn moves in this version group.
    pub move_learn_methods: Option<Vec<NamedApiResource>>,
    /// A list of Pokédexes introduces in this version group.
    pub pokedexes: Option<Vec<NamedApiResource>>,
    /// A list of regions that can be visited in this version group.
    pub regions: Option<Vec<NamedApiResource>>,
    /// The versions this version group owns.
    pub versions: Option<Vec<NamedApiResource>>,
}
