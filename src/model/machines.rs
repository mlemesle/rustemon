//! Machines group models

use super::{games::VersionGroup, items::Item, moves::Move, resource::NamedApiResource};

/// [Machine official documentation](https://pokeapi.co/docs/v2#machine)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Machine {
    /// The identifier for this resource.
    pub id: i64,
    /// The TM or HM item that corresponds to this machine.
    pub item: NamedApiResource<Item>,
    /// The move that is taught by this machine.
    #[serde(rename = "move")]
    pub move_: NamedApiResource<Move>,
    /// The version group that this machine applies to.
    pub version_group: NamedApiResource<VersionGroup>,
}
