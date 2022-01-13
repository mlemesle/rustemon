//! Moves endpoints group

/// Moves are the skills of Pokémon in battle. In battle, a Pokémon uses one move each turn.
/// Some moves (including those learned by Hidden Machine) can be used outside of battle as well,
/// usually for the purpose of removing obstacles or exploring new areas.
pub mod move_ {
    crate::endpoint!(crate::model::moves::Move; for "move");
}

/// Move Ailments are status conditions caused by moves used during battle.
/// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/http://bulbapedia.bulbagarden.net/wiki/Status_condition) for greater detail.
pub mod move_ailment {
    crate::endpoint!(crate::model::moves::MoveAilment; for "move-ailment");
}

/// Styles of moves when used in the Battle Palace.
/// See [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Battle_Frontier_(Generation_III)) for greater detail.
pub mod move_battle_style {
    crate::endpoint!(crate::model::moves::MoveBattleStyle; for "move-battle-style");
}

/// Very general categories that loosely group move effects.
pub mod move_category {
    crate::endpoint!(crate::model::moves::MoveCategory; for "move-category");
}

/// Damage classes moves can have, e.g. physical, special, or non-damaging.
pub mod move_damage_class {
    crate::endpoint!(crate::model::moves::MoveDamageClass; for "move-damage-class");
}

/// Methods by which Pokémon can learn moves.
pub mod move_learn_method {
    crate::endpoint!(crate::model::moves::MoveLearnMethod; for "move-learn-method");
}

/// Targets moves can be directed at during battle. Targets can be Pokémon, environments or even other moves.
pub mod move_target {
    crate::endpoint!(crate::model::moves::MoveTarget; for "move-target");
}
