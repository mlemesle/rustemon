//! Encounters group models

use super::resource::{Name, NamedApiResource};

/// [EncounterMethod official documentation](https:///pokeapi.co/docs/v2#encountermethod)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EncounterMethod {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A good value for sorting.
    pub order: Option<i64>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [EncounterCondition official documentation](https:///pokeapi.co/docs/v2#encountercondition)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EncounterCondition {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of possible values for this encounter condition.
    pub values: Option<Vec<NamedApiResource>>,
}

/// [EncounterConditionValue](https:///pokeapi.co/docs/v2#encounterconditionvalue)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EncounterConditionValue {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The condition this encounter condition value pertains to.
    pub condition: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}
