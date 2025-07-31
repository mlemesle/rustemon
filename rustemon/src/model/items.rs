//! Items group models

use super::{
    evolution::EvolutionChain,
    games::Version,
    pokemon::Pokemon,
    resource::{
        ApiResource, Description, Effect, GenerationGameIndex, MachineVersionDetail, Name,
        NamedApiResource, VerboseEffect, VersionGroupFlavorText,
    },
};

/// [Item official documentation](https://pokeapi.co/docs/v2#item)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Item {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The price of this item in stores.
    pub cost: i64,
    /// The power of the move Fling when used with this item.
    pub fling_power: Option<i64>,
    /// The effect of the move Fling when used with this item.
    pub fling_effect: Option<NamedApiResource<ItemFlingEffect>>,
    /// A list of attributes this item has.
    pub attributes: Vec<NamedApiResource<ItemAttribute>>,
    /// The category of items this item falls into.
    pub category: NamedApiResource<ItemCategory>,
    /// The effect of this ability listed in different languages.
    pub effect_entries: Vec<VerboseEffect>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Vec<VersionGroupFlavorText>,
    /// A list of game indices relevent to this item by generation.
    pub game_indices: Vec<GenerationGameIndex>,
    /// The name of this item listed in different languages.
    pub names: Vec<Name>,
    /// A set of sprites used to depict this item in the game.
    pub sprites: ItemSprites,
    /// A list of Pokémon that might be found in the wild holding this item.
    pub held_by_pokemon: Vec<ItemHolderPokemon>,
    /// An evolution chain this item requires to produce a bay during mating.
    pub baby_trigger_for: Option<ApiResource<EvolutionChain>>,
    /// A list of the machines related to this item.
    pub machines: Vec<MachineVersionDetail>,
}

/// [ItemSprites official documentation](https://pokeapi.co/docs/v2#itemsprites)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemSprites {
    /// The default depiction of this item.
    pub default: Option<String>,
}

/// [ItemHolderPokemon official documentation](https://pokeapi.co/docs/v2#itemholderpokemon)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemHolderPokemon {
    /// The Pokémon that holds this item.
    pub pokemon: NamedApiResource<Pokemon>,
    /// The details for the version that this item is held in by the Pokémon.
    pub version_details: Vec<ItemHolderPokemonVersionDetail>,
}

/// [ItemHolderPokemonVersionDetail official documentation](https://pokeapi.co/docs/v2#itemholderpokemonversiondetail)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemHolderPokemonVersionDetail {
    /// How often this Pokémon holds this item in this version.
    pub rarity: i64,
    /// The version that this item is held in by the Pokémon.
    pub version: NamedApiResource<Version>,
}

/// [ItemAttribute official documentation](https://pokeapi.co/docs/v2#itemattribute)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemAttribute {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of items that have this attribute.
    pub items: Vec<NamedApiResource<Item>>,
    /// The name of this item attribute listed in different languages.
    pub names: Vec<Name>,
    /// The description of this item attribute listed in different languages.
    pub descriptions: Vec<Description>,
}

/// [ItemCategory official documentation](https://pokeapi.co/docs/v2#itemcategory)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemCategory {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of items that are a part of this category.
    pub items: Vec<NamedApiResource<Item>>,
    /// The name of this item category listed in different languages.
    pub names: Vec<Name>,
    /// The pocket items in this category would be put in.
    pub pocket: NamedApiResource<ItemPocket>,
}

/// [ItemFlingEffect official documentation](https://pokeapi.co/docs/v2#itemflingeffect)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemFlingEffect {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The result of this fling effect listed in different languages.
    pub effect_entries: Vec<Effect>,
    /// A list of items that have this fling effect.
    pub items: Vec<NamedApiResource<Item>>,
}

/// [ItemPocket official documentation](https://pokeapi.co/docs/v2#itempocket)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ItemPocket {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of item categories that are relevant to this item pocket.
    pub categories: Vec<NamedApiResource<ItemCategory>>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}
