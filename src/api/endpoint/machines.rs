//! Machines endpoints group

/// Machines are the representation of items that teach moves to Pokémon. They vary from version to version, 
/// so it is not certain that one specific TM or HM corresponds to a single Machine.
pub mod machine {
    crate::api::endpoint::endpoint::endpoint!(crate::api::model::machines::Machine; for "machine");
}