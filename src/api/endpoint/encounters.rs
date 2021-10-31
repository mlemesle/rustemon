//! Encounters endpoints group

/// Methods by which the player might can encounter Pokémon in the wild, e.g., walking in tall grass. 
/// Check out [Bulbapedia](http:///bulbapedia.bulbagarden.net/wiki/Wild_Pokémon) for greater detail.
pub mod encounter_method {
    crate::api::endpoint::endpoint::endpoint!(crate::api::model::encounters::EncounterMethod; for "encounter-method");
}

/// Conditions which affect what pokemon might appear in the wild, e.g., day or night.
pub mod encounter_condition {
    crate::api::endpoint::endpoint::endpoint!(crate::api::model::encounters::EncounterCondition; for "encounter-condition");
}

/// Encounter condition values are the various states that an encounter condition can have, 
/// i.e., time of day can be either day or night.
pub mod encounter_condition_value {
    crate::api::endpoint::endpoint::endpoint!(crate::api::model::encounters::EncounterConditionValue; for "encounter-condition-value");
}
