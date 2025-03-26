//! Common models

use std::marker::PhantomData;

use super::{
    encounters::{EncounterConditionValue, EncounterMethod},
    games::{Generation, Version, VersionGroup},
    machines::Machine,
    utility::Language,
};

/// [NamedApiResource official documentation](https://pokeapi.co/docs/v2#namedapiresource)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NamedApiResource<T> {
    /// The name of the referenced resource.
    pub name: String,
    /// The URL of the referenced resource.
    pub url: String,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

/// [NamedApiResourceList official documentation](https:///pokeapi.co/docs/v2#namedapiresourcelist)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NamedApiResourceList<T> {
    /// The total number of resources available from this API.
    pub count: i64,
    /// The URL for the next page in the list.
    pub next: Option<String>,
    /// The URL for the previous page in the list.
    pub previous: Option<String>,
    /// A list of  named API resources.
    pub results: Vec<NamedApiResource<T>>,
}

/// [ApiResourceList official documentation](https:///pokeapi.co/docs/v2#apiresourcelist)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ApiResourceList<T> {
    /// The total number of resources available from this API.
    pub count: i64,
    /// The URL for the next page in the list.
    pub next: Option<String>,
    /// The URL for the previous page in the list.
    pub previous: Option<String>,
    /// A list of  named API resources.
    pub results: Vec<ApiResource<T>>,
}

/// [ApiResource official documentation](https://pokeapi.co/docs/v2#apiresource)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ApiResource<T> {
    /// The URL of the referenced resource.
    pub url: String,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

/// [Description official documentation](https://pokeapi.co/docs/v2#description)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Description {
    /// The localized description for an API resource in a specific language.
    pub description: String,
    /// The language this name is in.
    pub language: NamedApiResource<Language>,
}

/// [Effect official documentation](https://pokeapi.co/docs/v2#effect)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Effect {
    /// The localized effect text for an API resource in a specific language.
    pub effect: String,
    /// The language this effect is in.
    pub language: NamedApiResource<Language>,
}

/// [Encounter official documentation](https://pokeapi.co/docs/v2#encounter)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Encounter {
    /// The lowest level the Pokémon could be encountered at.
    pub min_level: i64,
    /// The highest level the Pokémon could be encountered at.
    pub max_level: i64,
    /// A list of condition values that must be in effect for this encounter to occur.
    pub condition_values: Vec<NamedApiResource<EncounterConditionValue>>,
    /// Percent chance that this encounter will occur.
    pub chance: i64,
    /// The method by which this encounter happens.
    pub method: NamedApiResource<EncounterMethod>,
}

/// [FlavorText official documentation](https://pokeapi.co/docs/v2#flavortext)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct FlavorText {
    /// The localized flavor text for an API resource in a specific language.
    pub flavor_text: String,
    /// The language this name is in.
    pub language: NamedApiResource<Language>,
    /// The game version this flavor text is extracted from.
    pub version: Option<NamedApiResource<Version>>,
}

/// [GenerationGameIndex official documentation](https://pokeapi.co/docs/v2#generationgameindex)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationGameIndex {
    /// The internal id of an API resource within game data.
    pub game_index: i64,
    /// The generation relevent to this game index.
    pub generation: NamedApiResource<Generation>,
}

/// [MachineVersionDetail official documentation](https://pokeapi.co/docs/v2#machineversiondetail)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MachineVersionDetail {
    /// The machine that teaches a move from an item.
    pub machine: ApiResource<Machine>,
    /// The version group of this specific machine.
    pub version_group: NamedApiResource<VersionGroup>,
}

/// [Name official documentation](https://pokeapi.co/docs/v2#name)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Name {
    /// The localized name for an API resource in a specific language.
    pub name: String,
    /// The language this name is in.
    pub language: NamedApiResource<Language>,
}

/// [VerboseEffect official documentation](https://pokeapi.co/docs/v2#verboseeffect)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VerboseEffect {
    /// The localized effect text for an API resource in a specific language.
    pub effect: String,
    /// The localized effect text in brief.
    pub short_effect: String,
    /// The language this effect is in.
    pub language: NamedApiResource<Language>,
}

/// [VersionEncounterDetail official documentation](https://pokeapi.co/docs/v2#versionencounterdetail)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VersionEncounterDetail {
    /// The game version this encounter happens in.
    pub version: NamedApiResource<Version>,
    /// The total percentage of all encounter potential.
    pub max_chance: i64,
    /// A list of encounters and their specifics.
    pub encounter_details: Vec<Encounter>,
}

/// [VersionGameIndex official documentation](https://pokeapi.co/docs/v2#versiongameindex)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VersionGameIndex {
    /// The internal id of an API resource within game data.
    pub game_index: i64,
    /// The version relevent to this game index.
    pub version: NamedApiResource<Version>,
}

/// [VersionGroupFlavorText official documentation](https://pokeapi.co/docs/v2#versiongroupflavortext)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VersionGroupFlavorText {
    /// The localized name for an API resource in a specific language.
    pub text: String,
    /// The language this name is in.
    pub language: NamedApiResource<Language>,
    /// The version group which uses this flavor text.
    pub version_group: NamedApiResource<VersionGroup>,
}
