//! Contests group models

use super::{
    berries::BerryFlavor,
    moves::Move,
    resource::{Effect, FlavorText, NamedApiResource},
    utility::Language,
};

/// [ContestType official documentation] (https:///pokeapi.co/docs/v2#contesttype)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ContestType {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The berry flavor that correlates with this contest type.
    pub berry_flavor: Option<NamedApiResource<BerryFlavor>>,
    /// The name of this contest type listed in different languages.
    pub names: Option<Vec<ContestName>>,
}

/// [ContestName official documentation](https:///pokeapi.co/docs/v2#contestname)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ContestName {
    /// The name for this contest.
    pub name: Option<String>,
    /// The color associated with this contest's name.
    pub color: Option<String>,
    /// The language that this name is in.
    pub language: Option<NamedApiResource<Language>>,
}

/// [ContestEffect official documentation](https:///pokeapi.co/docs/v2#contesteffect)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ContestEffect {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The base number of hearts the user of this move gets.
    pub appeal: Option<i64>,
    /// The base number of hearts the user's opponent loses.
    pub jam: Option<i64>,
    /// The result of this contest effect listed in different languages.
    pub effect_entries: Option<Vec<Effect>>,
    /// The flavor text of this contest effect listed in different languages.
    pub flavor_text_entries: Option<Vec<FlavorText>>,
}

/// [SuperContestEffect official documentation](https:///pokeapi.co/docs/v2#supercontesteffect)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct SuperContestEffect {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The level of appeal this super contest effect has.
    pub appeal: Option<i64>,
    /// The flavor text of this super contest effect listed in different languages.
    pub flavor_text_entries: Option<Vec<FlavorText>>,
    /// A list of moves that have the effect when used in super contests.
    pub moves: Option<Vec<NamedApiResource<Move>>>,
}
