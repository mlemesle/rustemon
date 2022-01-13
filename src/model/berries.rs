//! Berries group models

use super::resource::{Name, NamedApiResource};

/// [Berry official documentation](https:///pokeapi.co/docs/v2#berry)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Berry {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// Time it takes the tree to grow one stage, in hours.
    /// Berry trees go through four of these growth stages before they can be picked.
    pub growth_time: Option<i64>,
    /// The maximum number of these berries that can grow on one tree in Generation IV.
    pub max_harvest: Option<i64>,
    /// The power of the move "Natural Gift" when used with this Berry.
    pub natural_gift_power: Option<i64>,
    /// The size of this Berry, in millimeters.
    pub size: Option<i64>,
    /// The smoothness of this Berry, used in making Pokéblocks or Poffins.
    pub smoothness: Option<i64>,
    /// The speed at which this Berry dries out the soil as it grows.
    /// A higher rate means the soil dries more quickly.
    pub soil_dryness: Option<i64>,
    /// The firmness of this berry, used in making Pokéblocks or Poffins.
    pub firmness: Option<NamedApiResource>,
    /// A list of references to each flavor a berry can have and the potency
    /// of each of those flavors in regard to this berry.
    pub flavors: Option<Vec<BerryFlavorMap>>,
    /// Berries are actually items. This is a reference to the item specific data for this berry.
    pub item: Option<NamedApiResource>,
    /// The type inherited by "Natural Gift" when used with this Berry.
    pub natural_gift_type: Option<NamedApiResource>,
}

/// [BerryFlavorMap official documentation](https:///pokeapi.co/docs/v2#berryflavormap)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BerryFlavorMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: Option<i64>,
    /// The referenced berry flavor.
    pub flavor: Option<NamedApiResource>,
}

/// [BerryFirmness official documentation](https:///pokeapi.co/docs/v2#berryfirmness)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BerryFirmness {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of the berries with this firmness.
    pub berries: Option<Vec<NamedApiResource>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [BerryFlavor official documentation](https:///pokeapi.co/docs/v2#berryflavor)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BerryFlavor {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of the berries with this flavor.
    pub berries: Option<Vec<FlavorBerryMap>>,
    /// The contest type that correlates with this berry flavor.
    pub contest_type: Option<NamedApiResource>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [FlavorBerryMap official documentation](https:///pokeapi.co/docs/v2#flavorberrymap)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FlavorBerryMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: Option<i64>,
    /// The berry with the referenced flavor.
    pub berry: Option<NamedApiResource>,
}
