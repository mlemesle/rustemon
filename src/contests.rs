//! Contests endpoints group

/// Contest types are categories judges used to weigh a Pokémon's condition in Pokémon contests.
/// Check out [Bulbapedia](https:///bulbapedia.bulbagarden.net/wiki/Contest_condition) for greater detail.
pub mod contest_type {
    crate::endpoint!(crate::model::contests::ContestType; for "contest-type");
}

/// Contest effects refer to the effects of moves when used in contests.
pub mod contest_effect {
    crate::endpoint!(crate::model::contests::ContestEffect; for "contest-effect");
}

/// Super contest effects refer to the effects of moves when used in super contests.
pub mod super_contest_effect {
    crate::endpoint!(crate::model::contests::SuperContestEffect; for "super-contest-effect");
}
