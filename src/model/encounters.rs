//! Encounters group models

use super::resource::{Name, NamedApiResource};

/// [EncounterMethod official documentation](https:///pokeapi.co/docs/v2#encountermethod)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct EncounterMethod {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A good value for sorting.
    pub order: i64,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// [EncounterCondition official documentation](https:///pokeapi.co/docs/v2#encountercondition)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct EncounterCondition {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of possible values for this encounter condition.
    pub values: Vec<NamedApiResource<EncounterConditionValue>>,
}

/// [EncounterConditionValue](https:///pokeapi.co/docs/v2#encounterconditionvalue)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct EncounterConditionValue {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The condition this encounter condition value pertains to.
    pub condition: NamedApiResource<EncounterCondition>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}
