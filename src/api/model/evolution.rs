//! Evolution group models

use super::resource::{Name, NamedApiResource};

/// [EvolutionChain official documentation](https:///pokeapi.co/docs/v2#evolutionchain)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EvolutionChain {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The item that a Pokémon would be holding when mating that would trigger the egg 
    /// hatching a baby Pokémon rather than a basic Pokémon.
    pub baby_trigger_item: Option<NamedApiResource>,
    /// The base chain link object. Each link contains evolution details for a Pokémon in the chain. 
    /// Each link references the next Pokémon in the natural evolution order.
    pub chain: Option<ChainLink>,
}

/// [ChainLink official documentation](https:///pokeapi.co/docs/v2#chainlink)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ChainLink {
    /// Whether or not this link is for a baby Pokémon. This would only ever be true on the base link.
    pub is_baby: Option<bool>,
    /// The Pokémon species at this point in the evolution chain.
    pub species: Option<NamedApiResource>,
    /// All details regarding the specific details of the referenced Pokémon species evolution.
    pub evolution_details: Option<EvolutionDetail>,
    /// A List of chain objects.
    pub evolves_to: Option<Vec<ChainLink>>,
}

/// [EvolutionDetail official documentation](https:///pokeapi.co/docs/v2#evolutiondetail)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EvolutionDetail {
    /// The item required to cause evolution this into Pokémon species.
    pub item: Option<NamedApiResource>,
    /// The type of event that triggers evolution into this Pokémon species.
    pub trigger: Option<NamedApiResource>,
    /// The id of the gender of the evolving Pokémon species must be in order to evolve into this Pokémon species.
    pub gender: Option<i64>,
    /// The item the evolving Pokémon species must be holding during the evolution trigger event 
    /// to evolve into this Pokémon species.
    pub held_item: Option<NamedApiResource>,
    /// The move that must be known by the evolving Pokémon species during the evolution trigger event 
    /// in order to evolve into this Pokémon species.
    pub known_move: Option<NamedApiResource>,
    /// The evolving Pokémon species must know a move with this type during the evolution trigger event 
    /// in order to evolve into this Pokémon species.
    pub known_move_type: Option<NamedApiResource>,
    /// The location the evolution must be triggered at.
    pub location: Option<NamedApiResource>,
    /// The minimum required level of the evolving Pokémon species to evolve into this Pokémon species.
    pub min_level: Option<i64>,
    /// The minimum required level of happiness the evolving Pokémon species to evolve into this Pokémon species.
    pub min_happiness: Option<i64>,
    /// The minimum required level of beauty the evolving Pokémon species to evolve into this Pokémon species.
    pub min_beauty: Option<i64>,
    /// The minimum required level of affection the evolving Pokémon species to evolve into this Pokémon species.
    pub min_affection: Option<i64>,
    /// Whether or not it must be raining in the overworld to cause evolution this Pokémon species.
    pub needs_overworld_rain: Option<bool>,
    /// The Pokémon species that must be in the players party in order for the evolving Pokémon species 
    /// to evolve into this Pokémon species.
    pub party_species: Option<NamedApiResource>,
    /// The player must have a Pokémon of this type in their party during the evolution trigger event 
    /// in order for the evolving Pokémon species to evolve into this Pokémon species.
    pub party_type: Option<NamedApiResource>,
    /// The required relation between the Pokémon's Attack and Defense stats. 
    /// 1 means Attack > Defense. 0 means Attack = Defense. -1 means Attack < Defense.
    pub relative_physical_stats: Option<i64>,
    /// The required time of day. Day or night.
    pub time_of_day: Option<String>,
    /// Pokémon species for which this one must be traded.
    pub trade_species: Option<NamedApiResource>,
    /// Whether or not the 3DS needs to be turned upside-down as this Pokémon levels up.
    pub turn_upside_down: Option<bool>,
}

/// [EvolutionTrigger official documentation](https:///pokeapi.co/docs/v2#evolutiontrigger)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EvolutionTrigger {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of pokemon species that result from this evolution trigger.
    pub pokemon_species: Option<Vec<NamedApiResource>>,
}
