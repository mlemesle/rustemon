//! Evolution endpoints group

/// Evolution chains are essentially family trees. They start with the lowest stage within a family
/// and detail evolution conditions for each as well as Pokémon they can evolve into up through the hierarchy.
pub mod evolution_chain {
    crate::endpoint!(unnamed crate::model::evolution::EvolutionChain; for "evolution-chain");
}

/// Evolution triggers are the events and conditions that cause a Pokémon to evolve.
/// Check out [Bulbapedia](http:///bulbapedia.bulbagarden.net/wiki/Methods_of_evolution) for greater detail.
pub mod evolution_trigger {
    crate::endpoint!(crate::model::evolution::EvolutionTrigger; for "evolution-trigger");
}
