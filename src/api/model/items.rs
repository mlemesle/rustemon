//! Items group models

use super::resource::{
    ApiResource, Description, Effect, GenerationGameIndex, MachineVersionDetail, Name,
    NamedApiResource, VerboseEffect, VersionGroupFlavorText,
};

/// [Item official documentation](https://pokeapi.co/docs/v2#item)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Item {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The price of this item in stores.
    pub cost: Option<i64>,
    /// The power of the move Fling when used with this item.
    pub fling_power: Option<NamedApiResource>,
    /// The effect of the move Fling when used with this item.
    pub fling_effect: Option<NamedApiResource>,
    /// A list of attributes this item has.
    pub attributes: Option<Vec<NamedApiResource>>,
    /// The category of items this item falls into.
    pub category: Option<NamedApiResource>,
    /// The effect of this ability listed in different languages.
    pub effect_entries: Option<Vec<VerboseEffect>>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Option<Vec<VersionGroupFlavorText>>,
    /// A list of game indices relevent to this item by generation.
    pub game_indices: Option<Vec<GenerationGameIndex>>,
    /// The name of this item listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A set of sprites used to depict this item in the game.
    pub sprites: Option<ItemSprites>,
    /// A list of Pokémon that might be found in the wild holding this item.
    pub held_by_pokemon: Option<Vec<ItemHolderPokemon>>,
    /// An evolution chain this item requires to produce a bay during mating.
    pub baby_trigger_for: Option<ApiResource>,
    /// A list of the machines related to this item.
    pub machines: Option<Vec<MachineVersionDetail>>,
}

/// [ItemSprites official documentation](https://pokeapi.co/docs/v2#itemsprites)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemSprites {
    /// The default depiction of this item.
    pub default: Option<String>,
}

/// [ItemHolderPokemon official documentation](https://pokeapi.co/docs/v2#itemholderpokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemHolderPokemon {
    /// The Pokémon that holds this item.
    pub pokemon: Option<NamedApiResource>,
    /// The details for the version that this item is held in by the Pokémon.
    pub version_details: Option<Vec<ItemHolderPokemonVersionDetail>>,
}

/// [ItemHolderPokemonVersionDetail official documentation](https://pokeapi.co/docs/v2#itemholderpokemonversiondetail)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemHolderPokemonVersionDetail {
    /// How often this Pokémon holds this item in this version.
    pub rarity: Option<i64>,
    /// The version that this item is held in by the Pokémon.
    pub version: Option<NamedApiResource>,
}

/// [ItemAttribute official documentation](https://pokeapi.co/docs/v2#itemattribute)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemAttribute {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of items that have this attribute.
    pub items: Option<Vec<NamedApiResource>>,
    /// The name of this item attribute listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The description of this item attribute listed in different languages.
    pub descriptions: Option<Vec<Description>>,
}

/// [ItemCategory official documentation](https://pokeapi.co/docs/v2#itemcategory)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemCategory {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of items that are a part of this category.
    pub items: Option<Vec<NamedApiResource>>,
    /// The name of this item category listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The pocket items in this category would be put in.
    pub pocket: Option<NamedApiResource>,
}

/// [ItemFlingEffect official documentation](https://pokeapi.co/docs/v2#itemflingeffect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemFlingEffect {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The result of this fling effect listed in different languages.
    pub effect_entries: Option<Vec<Effect>>,
    /// A list of items that have this fling effect.
    pub items: Option<Vec<NamedApiResource>>,
}

/// [ItemPocket official documentation](https://pokeapi.co/docs/v2#itempocket)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ItemPocket {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of item categories that are relevant to this item pocket.
    pub categories: Option<Vec<NamedApiResource>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}
