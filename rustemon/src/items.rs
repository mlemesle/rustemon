//! Items endpoints group

/// An item is an object in the games which the player can pick up, keep in their bag, and use in some manner.
///
/// They have various uses, including healing, powering up, helping catch Pokémon, or to access a new area.
pub mod item {
    crate::endpoint!(crate::model::items::Item; for "item");
}

/// Item attributes define particular aspects of items, e.g. "usable in battle" or "consumable".
pub mod item_attribute {
    crate::endpoint!(crate::model::items::ItemAttribute; for "item-attribute");
}

/// Item categories determine where items will be placed in the players bag.
pub mod item_category {
    crate::endpoint!(crate::model::items::ItemCategory; for "item-category");
}

/// The various effects of the move "Fling" when used with different items.
pub mod item_fling_effect {
    crate::endpoint!(crate::model::items::ItemFlingEffect; for "item-fling-effect");
}

/// Pockets within the players bag used for storing items by category.
pub mod item_pocket {
    crate::endpoint!(crate::model::items::ItemPocket; for "item-pocket");
}
