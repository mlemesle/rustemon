//! Utility group models

use super::resource::Name;

/// [Language official documentation](https://pokeapi.co/docs/v2#language)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Language {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// Whether or not the games are published in this language.
    pub official: bool,
    /// The two-letter code of the country where this language is spoken. Note that it is not unique.
    pub iso639: String,
    /// The two-letter code of the language. Note that it is not unique.
    pub iso3166: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}
