//! Berries group models

use super::{
    contests::ContestType,
    items::Item,
    pokemon::Type,
    resource::{Name, NamedApiResource},
};

/// [Berry official documentation](https:///pokeapi.co/docs/v2#berry)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Berry {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// Time it takes the tree to grow one stage, in hours.
    /// Berry trees go through four of these growth stages before they can be picked.
    pub growth_time: i64,
    /// The maximum number of these berries that can grow on one tree in Generation IV.
    pub max_harvest: i64,
    /// The power of the move "Natural Gift" when used with this Berry.
    pub natural_gift_power: i64,
    /// The size of this Berry, in millimeters.
    pub size: i64,
    /// The smoothness of this Berry, used in making Pokéblocks or Poffins.
    pub smoothness: i64,
    /// The speed at which this Berry dries out the soil as it grows.
    /// A higher rate means the soil dries more quickly.
    pub soil_dryness: i64,
    /// The firmness of this berry, used in making Pokéblocks or Poffins.
    pub firmness: NamedApiResource<BerryFirmness>,
    /// A list of references to each flavor a berry can have and the potency
    /// of each of those flavors in regard to this berry.
    pub flavors: Vec<BerryFlavorMap>,
    /// Berries are actually items. This is a reference to the item specific data for this berry.
    pub item: NamedApiResource<Item>,
    /// The type inherited by "Natural Gift" when used with this Berry.
    pub natural_gift_type: NamedApiResource<Type>,
}

/// [BerryFlavorMap official documentation](https:///pokeapi.co/docs/v2#berryflavormap)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BerryFlavorMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: i64,
    /// The referenced berry flavor.
    pub flavor: NamedApiResource<BerryFlavor>,
}

/// [BerryFirmness official documentation](https:///pokeapi.co/docs/v2#berryfirmness)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BerryFirmness {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of the berries with this firmness.
    pub berries: Vec<NamedApiResource<Berry>>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// [BerryFlavor official documentation](https:///pokeapi.co/docs/v2#berryflavor)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BerryFlavor {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of the berries with this flavor.
    pub berries: Vec<FlavorBerryMap>,
    /// The contest type that correlates with this berry flavor.
    pub contest_type: NamedApiResource<ContestType>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// [FlavorBerryMap official documentation](https:///pokeapi.co/docs/v2#flavorberrymap)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FlavorBerryMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: i64,
    /// The berry with the referenced flavor.
    pub berry: NamedApiResource<Berry>,
}
