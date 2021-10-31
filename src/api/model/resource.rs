//! Common models

/// [NamedApiResourceList official documentation](https:///pokeapi.co/docs/v2#namedapiresourcelist)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NamedApiResourceList {
    /// The total number of resources available from this API.
    pub count: Option<i64>,
    /// The URL for the next page in the list.
    pub next: Option<String>,
    /// The URL for the previous page in the list.
    pub previous: Option<::serde_json::Value>,
    /// A list of named API resources.
    pub results: Option<Vec<NamedApiResource>>,
}

/// [ApiResource official documentation](https://pokeapi.co/docs/v2#apiresource)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ApiResource {
    /// The URL of the referenced resource.
    pub url: Option<String>,
}

/// [Description official documentation](https://pokeapi.co/docs/v2#description)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Description {
    /// The localized description for an API resource in a specific language.
    pub description: Option<String>,
    /// The language this name is in.
    pub language: Option<NamedApiResource>,
}

/// [Effect official documentation](https://pokeapi.co/docs/v2#effect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Effect {
    /// The localized effect text for an API resource in a specific language.
    pub effect: Option<String>,
    /// The language this effect is in.
    pub language: Option<NamedApiResource>,
}

/// [Encounter official documentation](https://pokeapi.co/docs/v2#encounter)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Encounter {
    /// The lowest level the Pokémon could be encountered at.
    pub min_level: Option<i64>,
    /// The highest level the Pokémon could be encountered at.
    pub max_level: Option<i64>,
    /// A list of condition values that must be in effect for this encounter to occur.
    pub condition_values: Option<Vec<NamedApiResource>>,
    /// Percent chance that this encounter will occur.
    pub chance: Option<i64>,
    /// The method by which this encounter happens.
    pub method: Option<NamedApiResource>,
}

/// [FlavorText official documentation](https://pokeapi.co/docs/v2#flavortext)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FlavorText {
    /// The localized flavor text for an API resource in a specific language.
    pub flavor_text: Option<String>,
    /// The language this name is in.
    pub language: Option<NamedApiResource>,
    /// The game version this flavor text is extracted from.
    pub version: Option<NamedApiResource>,
}

/// [GenerationGameIndex official documentation](https://pokeapi.co/docs/v2#generationgameindex)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationGameIndex {
    /// The internal id of an API resource within game data.
    pub game_index: Option<i64>,
    /// The generation relevent to this game index.
    pub generation: Option<NamedApiResource>,
}

/// [MachineVersionDetail official documentation](https://pokeapi.co/docs/v2#machineversiondetail)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MachineVersionDetail {
    /// The machine that teaches a move from an item.
    pub machine: Option<ApiResource>,
    /// The version group of this specific machine.
    pub version_group: Option<NamedApiResource>,
}

/// [Name official documentation](https://pokeapi.co/docs/v2#name)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Name {
    /// The localized name for an API resource in a specific language.
    pub name: Option<String>,
    /// The language this name is in.
    pub language: Option<NamedApiResource>,
}

/// [NamedApiResource official documentation](https://pokeapi.co/docs/v2#namedapiresource)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NamedApiResource {
    /// The name of the referenced resource.
    pub name: Option<String>,
    /// The URL of the referenced resource.
    pub url: Option<String>,
}

/// [VerboseEffect official documentation](https://pokeapi.co/docs/v2#verboseeffect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VerboseEffect {
    /// The localized effect text for an API resource in a specific language.
    pub effect: Option<String>,
    /// The localized effect text in brief.
    pub short_effect: Option<String>,
    /// The language this effect is in.
    pub language: Option<NamedApiResource>,
}

/// [VersionEncounterDetail official documentation](https://pokeapi.co/docs/v2#versionencounterdetail)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VersionEncounterDetail {
    /// The game version this encounter happens in.
    pub version: Option<NamedApiResource>,
    /// The total percentage of all encounter potential.
    pub max_chance: Option<i64>,
    /// A list of encounters and their specifics.
    pub encounter_details: Option<Vec<Encounter>>,
}

/// [VersionGameIndex official documentation](https://pokeapi.co/docs/v2#versiongameindex)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VersionGameIndex {
    /// The internal id of an API resource within game data.
    pub game_index: Option<i64>,
    /// The version relevent to this game index.
    pub version: Option<NamedApiResource>,
}

/// [VersionGroupFlavorText official documentation](https://pokeapi.co/docs/v2#versiongroupflavortext)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VersionGroupFlavorText {
    /// The localized name for an API resource in a specific language.
    pub text: Option<String>,
    /// The language this name is in.
    pub language: Option<NamedApiResource>,
    /// The version group which uses this flavor text.
    pub version_group: Option<NamedApiResource>,
}
