//! Locations group models

use super::{
    encounters::EncounterMethod,
    games::{Generation, Pokedex, Version, VersionGroup},
    pokemon::{Pokemon, PokemonSpecies},
    resource::{GenerationGameIndex, Name, NamedApiResource, VersionEncounterDetail},
};

/// [Location official documentation](https://pokeapi.co/docs/v2#location)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Location {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The region this location can be found in.
    pub region: Option<NamedApiResource<Region>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of game indices relevent to this location by generation.
    pub game_indices: Option<Vec<GenerationGameIndex>>,
    /// Areas that can be found within this location.
    pub areas: Option<Vec<NamedApiResource<LocationArea>>>,
}

/// [LocationArea official documentation](https://pokeapi.co/docs/v2#locationarea)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct LocationArea {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The internal id of an API resource within game data.
    pub game_index: Option<i64>,
    /// A list of methods in which Pokémon may be encountered in this area
    /// and how likely the method will occur depending on the version of the game.
    pub encounter_method_rates: Option<Vec<EncounterMethodRate>>,
    /// The region this location area can be found in.
    pub location: Option<Location>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of Pokémon that can be encountered in this area
    /// along with version specific details about the encounter.
    pub pokemon_encounters: Option<Vec<PokemonEncounter>>,
}

/// [EncounterMethodRate official documentation](https://pokeapi.co/docs/v2#encountermethodrate)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct EncounterMethodRate {
    /// The method in which Pokémon may be encountered in an area.
    pub encounter_method: Option<NamedApiResource<EncounterMethod>>,
    /// The chance of the encounter to occur on a version of the game.
    pub version_details: Option<Vec<EncounterVersionDetails>>,
}

/// [EncounterVersionDetails official documentation](https://pokeapi.co/docs/v2#encounterversiondetails)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct EncounterVersionDetails {
    /// The chance of an encounter to occur.
    pub rate: Option<i64>,
    /// The version of the game in which the encounter can occur with the given chance.
    pub version: Option<NamedApiResource<Version>>,
}

/// [PokemonEncounter official documentation](https://pokeapi.co/docs/v2#pokemonencounter)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct PokemonEncounter {
    /// The Pokémon being encountered.
    pub pokemon: Option<NamedApiResource<Pokemon>>,
    /// A list of versions and encounters with Pokémon that might happen in the referenced location area.
    pub version_details: Option<Vec<VersionEncounterDetail>>,
}

/// [PalParkArea official documentation](https://pokeapi.co/docs/v2#palparkarea)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct PalParkArea {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of Pokémon encountered in thi pal park area along with details.
    pub pokemon_encounters: Option<Vec<PalParkEncounterSpecies>>,
}

/// [PalParkEncounterSpecies official documentation](https://pokeapi.co/docs/v2#palparkencounterspecies)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct PalParkEncounterSpecies {
    /// The base score given to the player when this Pokémon is caught during a pal park run.
    pub base_score: Option<i64>,
    /// The base rate for encountering this Pokémon in this pal park area.
    pub rate: Option<i64>,
    /// The Pokémon species being encountered.
    pub pokemon_species: Option<NamedApiResource<PokemonSpecies>>,
}

/// [Region official documentation](https://pokeapi.co/docs/v2#region)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Region {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// A list of locations that can be found in this region.
    pub locations: Option<Vec<NamedApiResource<Location>>>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The generation this region was introduced in.
    pub main_generation: Option<NamedApiResource<Generation>>,
    /// A list of pokédexes that catalogue Pokémon in this region.
    pub pokedexes: Option<Vec<NamedApiResource<Pokedex>>>,
    /// A list of version groups where this region can be visited.
    pub version_groups: Option<Vec<NamedApiResource<VersionGroup>>>,
}
