//! Utility group models

use super::resource::Name;

/// [Language official documentation](https://pokeapi.co/docs/v2#language)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Language {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// Whether or not the games are published in this language.
    pub official: Option<bool>,
    /// The two-letter code of the country where this language is spoken. Note that it is not unique.
    pub iso639: Option<String>,
    /// The two-letter code of the language. Note that it is not unique.
    pub iso3166: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}
