//! Pokemon group models

use super::{
    berries::BerryFlavor,
    games::{Generation, Version, VersionGroup},
    items::Item,
    locations::{LocationArea, PalParkArea},
    moves::{Move, MoveBattleStyle, MoveDamageClass, MoveLearnMethod},
    resource::{
        ApiResource, Description, Effect, FlavorText, GenerationGameIndex, Name, NamedApiResource,
        VerboseEffect, VersionEncounterDetail, VersionGameIndex,
    },
    utility::Language,
};

/// [Ability official documentation](https://pokeapi.co/docs/v2#ability)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Ability {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// Whether or not this ability originated in the main series of the video games.
    pub is_main_series: Option<bool>,
    /// The generation this ability originated in.
    pub generation: Option<NamedApiResource<Generation>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// The effect of this ability listed in different languages.
    pub effect_entries: Option<Vec<VerboseEffect>>,
    /// The list of previous effects this ability has had across version groups.
    pub effect_changes: Option<Vec<AbilityEffectChange>>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Option<Vec<AbilityFlavorText>>,
    /// A list of Pokémon that could potentially have this ability.
    pub pokemon: Option<Vec<AbilityPokemon>>,
}

/// [AbilityEffectChange official documentation](https://pokeapi.co/docs/v2#abilityeffectchange)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AbilityEffectChange {
    /// The previous effect of this ability listed in different languages.
    pub effect_entries: Option<Vec<Effect>>,
    /// The version group in which the previous effect of this ability originated.
    pub version_group: Option<NamedApiResource<VersionGroup>>,
}

/// [AbilityFlavorText official documentation](https://pokeapi.co/docs/v2#abilityflavortext)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AbilityFlavorText {
    /// The localized name for an API resource in a specific language.
    pub flavor_text: Option<String>,
    /// The language this text resource is in.
    pub language: Option<NamedApiResource<Language>>,
    /// The version group that uses this flavor text.
    pub version_group: Option<NamedApiResource<VersionGroup>>,
}

/// [AbilityPokemon official documentation](https://pokeapi.co/docs/v2#abilitypokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AbilityPokemon {
    /// Whether or not this a hidden ability for the referenced Pokémon.
    pub is_hidden: Option<bool>,
    /// Pokémon have 3 ability 'slots' which hold references to possible abilities they could have.
    /// This is the slot of this ability for the referenced pokemon.
    pub slot: Option<i64>,
    /// The Pokémon this ability could belong to.
    pub pokemon: Option<NamedApiResource<Pokemon>>,
}

/// [Characteristic official documentation](https://pokeapi.co/docs/v2#characteristic)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Characteristic {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The remainder of the highest stat/IV divided by 5.
    pub gene_modulo: Option<i64>,
    /// The possible values of the highest stat that would result in a Pokémon
    /// receiving this characteristic when divided by 5.
    pub possible_values: Option<Vec<i64>>,
    /// The description of this resource listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// The highest stat referenced by this characteristic.
    pub highest_stat: Option<NamedApiResource<Stat>>,
}

/// [EggGroup official documentation](https://pokeapi.co/docs/v2#egggroup)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EggGroup {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of all Pokémon species that are members of this egg group.
    pub pokemon_species: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [Gender official documentation](https://pokeapi.co/docs/v2#gender)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Gender {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A list of Pokémon species that can be this gender and how likely it is that they will be.
    pub pokemon_species_details: Option<Vec<PokemonSpeciesGender>>,
    /// A list of Pokémon species that required this gender in order for a Pokémon to evolve into them.
    pub required_for_evolution: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [PokemonSpeciesGender official documentation](https://pokeapi.co/docs/v2#pokemonspeciesgender)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpeciesGender {
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub rate: Option<i64>,
    /// A Pokémon species that can be the referenced gender.
    pub pokemon_species: Option<NamedApiResource<PokemonSpecies>>,
}

/// [GrowthRate official documentation](https://pokeapi.co/docs/v2#growthrate)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GrowthRate {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The formula used to calculate the rate at which the Pokémon species gains level.
    pub formula: Option<String>,
    /// The descriptions of this characteristic listed in different languages.
    pub descriptions: Option<Vec<Description>>,
    /// A list of levels and the amount of experienced needed to atain them based on this growth rate.
    pub levels: Option<Vec<GrowthRateExperienceLevel>>,
    /// A list of Pokémon species that gain levels at this growth rate.
    pub pokemon_species: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [GrowthRateExperienceLevel official documentation](https://pokeapi.co/docs/v2#growthrateexperiencelevel)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GrowthRateExperienceLevel {
    /// The level gained.
    pub level: Option<i64>,
    /// The amount of experience required to reach the referenced level.
    pub experience: Option<i64>,
}

/// [Nature official documentation](https://pokeapi.co/docs/v2#nature)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Nature {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The stat decreased by 10% in Pokémon with this nature.
    pub decreased_stat: Option<NamedApiResource<Stat>>,
    /// The stat increased by 10% in Pokémon with this nature.
    pub increased_stat: Option<NamedApiResource<Stat>>,
    /// The flavor hated by Pokémon with this nature.
    pub hates_flavor: Option<NamedApiResource<BerryFlavor>>,
    /// The flavor liked by Pokémon with this nature.
    pub likes_flavor: Option<NamedApiResource<BerryFlavor>>,
    /// A list of Pokéathlon stats this nature effects and how much it effects them.
    pub pokeathlon_stat_changes: Option<Vec<NatureStatChange>>,
    /// A list of battle styles and how likely a Pokémon with this nature is to use them in the Battle Palace or Battle Tent.
    pub move_battle_style_preferences: Option<Vec<MoveBattleStylePreference>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [NatureStatChange official documentation](https://pokeapi.co/docs/v2#naturestatchange)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NatureStatChange {
    /// The amount of change.
    pub max_change: Option<i64>,
    /// The stat being affected.
    pub pokeathlon_stat: Option<NamedApiResource<PokeathlonStat>>,
}

/// [MoveBattleStylePreference official documentation](https://pokeapi.co/docs/v2#movebattlestylepreference)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveBattleStylePreference {
    /// Chance of using the move, in percent, if HP is under one half.
    pub low_hp_preference: Option<i64>,
    /// Chance of using the move, in percent, if HP is over one half.
    pub high_hp_preference: Option<i64>,
    /// The move battle style.
    pub move_battle_style: Option<NamedApiResource<MoveBattleStyle>>,
}

/// [PokeathlonStat official documentation](https://pokeapi.co/docs/v2#pokeathlonstat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokeathlonStat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A detail of natures which affect this Pokéathlon stat positively or negatively.
    pub affecting_natures: Option<NaturePokeathlonStatAffectSets>,
}

/// [NaturePokeathlonStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturepokeathlonstataffectsets)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NaturePokeathlonStatAffectSets {
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub increase: Option<Vec<NaturePokeathlonStatAffect>>,
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub decrease: Option<Vec<NaturePokeathlonStatAffect>>,
}

/// [NaturePokeathlonStatAffect official documentation](https://pokeapi.co/docs/v2#naturepokeathlonstataffect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NaturePokeathlonStatAffect {
    /// The maximum amount of change to the referenced Pokéathlon stat.
    pub max_change: Option<i64>,
    /// The nature causing the change.
    pub nature: Option<NamedApiResource<Nature>>,
}

/// [Pokemon official documentation](https://pokeapi.co/docs/v2#pokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pokemon {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: Option<i64>,
    /// The height of this Pokémon in decimetres.
    pub height: Option<i64>,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: Option<bool>,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub order: Option<i64>,
    /// The weight of this Pokémon in hectograms.
    pub weight: Option<i64>,
    /// A list of abilities this Pokémon could potentially have.
    pub abilities: Option<Vec<PokemonAbility>>,
    /// A list of forms this Pokémon can take on.
    pub forms: Option<Vec<NamedApiResource<PokemonForm>>>,
    /// A list of game indices relevent to Pokémon item by generation.
    pub game_indices: Option<Vec<VersionGameIndex>>,
    /// A list of items this Pokémon may be holding when encountered.
    pub held_items: Option<Vec<PokemonHeldItem>>,
    /// A link to a list of location areas, as well as encounter details pertaining to specific versions.
    pub location_area_encounters: Option<String>,
    /// A list of moves along with learn methods and level details pertaining to specific version groups.
    pub moves: Option<Vec<PokemonMove>>,
    /// A list of details showing types this pokémon had in previous generations.
    pub past_types: Option<Vec<PokemonTypePast>>,
    /// A set of sprites used to depict this Pokémon in the game.
    /// A visual representation of the various sprites can be found at [PokeAPI/sprites](https://github.com/PokeAPI/sprites).
    pub sprites: Option<PokemonSprites>,
    /// The species this Pokémon belongs to.
    pub species: Option<NamedApiResource<PokemonSpecies>>,
    /// A list of base stat values for this Pokémon.
    pub stats: Option<Vec<PokemonStat>>,
    /// A list of details showing types this Pokémon has.
    pub types: Option<Vec<PokemonType>>,
}

/// [PokemonAbility official documentation](https://pokeapi.co/docs/v2#pokemonability)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonAbility {
    /// Whether or not this is a hidden ability.
    pub is_hidden: Option<bool>,
    /// The slot this ability occupies in this Pokémon species.
    pub slot: Option<i64>,
    /// The ability the Pokémon may have.
    pub ability: Option<NamedApiResource<Ability>>,
}

/// [PokemonType official documentation](https://pokeapi.co/docs/v2#pokemontype)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonType {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The type the referenced Pokémon has.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource<Type>>,
}

/// [PokemonTypePast official documentation](https://pokeapi.co/docs/v2#pokemontypepast)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonTypePast {
    /// The last generation in which the referenced pokémon had the listed types.
    pub generation: Option<NamedApiResource<Generation>>,
    /// The types the referenced pokémon had up to and including the listed generation.
    pub types: Option<Vec<PokemonType>>,
}

/// [PokemonHeldItem official documentation](https://pokeapi.co/docs/v2#pokemonhelditem)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonHeldItem {
    /// The item the referenced Pokémon holds.
    pub item: Option<NamedApiResource<Item>>,
    /// The details of the different versions in which the item is held.
    pub version_details: Option<Vec<PokemonHeldItemVersion>>,
}

/// [PokemonHeldItemVersion official documentation](https://pokeapi.co/docs/v2#pokemonhelditemversion)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonHeldItemVersion {
    /// The version in which the item is held.
    pub version: Option<NamedApiResource<Version>>,
    /// How often the item is held.
    pub rarity: Option<i64>,
}

/// [PokemonMove official documentation](https://pokeapi.co/docs/v2#pokemonmove)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonMove {
    /// The move the Pokémon can learn.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource<Move>>,
    /// The details of the version in which the Pokémon can learn the move.
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

/// [PokemonMoveVersion official documentation](https://pokeapi.co/docs/v2#pokemonmoveversion)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonMoveVersion {
    /// The method by which the move is learned.
    pub move_learn_method: Option<NamedApiResource<MoveLearnMethod>>,
    /// The version group in which the move is learned.
    pub version_group: Option<NamedApiResource<VersionGroup>>,
    /// The minimum level to learn the move.
    pub level_learned_at: Option<i64>,
}

/// [PokemonStat official documentation](https://pokeapi.co/docs/v2#pokemonstat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonStat {
    /// The stat the Pokémon has.
    pub stat: Option<NamedApiResource<Stat>>,
    /// The effort points (EV) the Pokémon has in the stat.
    pub effort: Option<i64>,
    /// The base value of the stat.
    pub base_stat: Option<i64>,
}

/// [PokemonSprites official documentation](https://pokeapi.co/docs/v2#pokemonsprites)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSprites {
    /// The default depiction of this Pokémon from the front in battle.
    pub front_default: Option<String>,
    /// The shiny depiction of this Pokémon from the front in battle.
    pub front_shiny: Option<String>,
    /// The female depiction of this Pokémon from the front in battle.
    pub front_female: Option<String>,
    /// The shiny female depiction of this Pokémon from the front in battle.
    pub front_shiny_female: Option<String>,
    /// The default depiction of this Pokémon from the back in battle.
    pub back_default: Option<String>,
    /// The shiny depiction of this Pokémon from the back in battle.
    pub back_shiny: Option<String>,
    /// The female depiction of this Pokémon from the back in battle.
    pub back_female: Option<String>,
    /// The shiny female depiction of this Pokémon from the back in battle.
    pub back_shiny_female: Option<String>,
    /// Other sprites.
    pub other: Option<OtherSprites>,
    /// Sprites per version.
    pub versions: Option<VersionsSprites>,
}

/// References sprites that doesn't come from game.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct OtherSprites {
    /// Dream world sprites of this Pokémon.
    pub dream_world: Option<DreamWorldSprites>,
    /// The official artwork of this Pokémon.
    #[serde(rename = "official-artwork")]
    pub official_artwork: Option<OfficialArtworkSprites>,
}

/// References the dream world sprites of a Pokémon.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DreamWorldSprites {
    /// The default despiction of this Pokémon from dream world.
    pub front_default: Option<String>,
    /// The female despiction of this Pokémon from dream world.
    pub front_female: Option<String>,
}

/// References the official artwork of a Pokémon.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct OfficialArtworkSprites {
    /// The default despiction of this Pokémon form the official artwork.
    pub front_default: Option<String>,
}

/// Sprites of a Pokémon, per generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VersionsSprites {
    /// Sprites for the first generation.
    #[serde(rename = "generation-i")]
    pub generation_i: Option<GenerationISprites>,
    /// Sprites for the second generation.
    #[serde(rename = "generation-i")]
    pub generation_ii: Option<GenerationIISprites>,
    /// Sprites for the third generation.
    #[serde(rename = "generation-iii")]
    pub generation_iii: Option<GenerationIIISprites>,
    /// Sprites for the fourth generation.
    #[serde(rename = "generation-iv")]
    pub generation_iv: Option<GenerationIVSprites>,
    /// Sprites for the fifth generation.
    #[serde(rename = "generation-v")]
    pub generation_v: Option<GenerationVSprites>,
    /// Sprites for the sixth generation.
    #[serde(rename = "generation-vi")]
    pub generation_vi: Option<GenerationVISprites>,
    /// Sprites for the seventh generation.
    #[serde(rename = "generation-vii")]
    pub generation_vii: Option<GenerationVIISprites>,
    /// Sprites for the eighth generation.
    #[serde(rename = "generation-viii")]
    pub generation_viii: Option<GenerationVIIISprites>,
}

/// Sprites for the first generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationISprites {
    /// Sprites for Pokémon Red & Pokémon Blue.
    #[serde(rename = "red-blue")]
    pub red_blue: Option<RedBlueSprites>,
    /// Sprites for Pokémon Yellow.
    pub yellow: Option<YellowSprites>,
}

/// Sprites for Pokémon Red & Pokémon Blue.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RedBlueSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
}

/// Sprites for Pokémon Yellow.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct YellowSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
}

/// Sprites for the second generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationIISprites {
    /// Sprites for Pokémon Crystal.
    pub crystal: Option<CrystalSprites>,
    /// Sprites for Pokémon Gold.
    pub gold: Option<GoldSprites>,
    /// Sprites for Pokémon Silver.
    pub silver: Option<SilverSprites>,
}

/// Sprites for Pokémon Crystal.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct CrystalSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon Gold.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GoldSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon Silver.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SilverSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for the third generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationIIISprites {
    /// Sprites for Pokémon Emerald.
    pub emerald: Option<EmeraldSprites>,
    /// Sprites for Pokémon FireRed & Pokémon LeafGreen.
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: Option<FireredLeafgreenSprites>,
    /// Sprites for Pokémon Ruby & Pokémon Sapphire.
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: Option<RubySapphireSprites>,
}

/// Sprites for Pokémon Emerald.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EmeraldSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon FireRed & Pokémon LeafGreen.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FireredLeafgreenSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon Ruby & Pokémon Sapphire.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RubySapphireSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for the fourth generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationIVSprites {
    /// Sprites for Pokémon Diamond & Pokémon Pearl.
    #[serde(rename = "diamond-pearl")]
    pub diamond_pearl: Option<DiamondPearlSprites>,
    /// Sprites for Pokémon Platinum.
    pub platinum: Option<PlatinumSprites>,
    /// Sprites for Pokémon HeartGold & Pokémon SoulSilver.
    #[serde(rename = "heartgold-soulsilver")]
    pub heartgold_soulsilver: Option<HeartgoldSoulsilverSprites>,
}

/// Sprites for Pokémon Diamond & Pokémon Pearl.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DiamondPearlSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for Pokémon Platinum.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PlatinumSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for Pokémon HeartGold & Pokémon SoulSilver.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct HeartgoldSoulsilverSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for the fifth generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationVSprites {
    /// Sprites for Pokémon Black & Pokémon White.
    #[serde(rename = "black-white")]
    pub black_white: Option<BlackWhiteSprites>,
}

/// Sprites for Pokémon Black & Pokémon White.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BlackWhiteSprites {
    /// The animated sprites for a Pokémon.
    pub animated: Option<BlackWhiteAnimatedSprites>,
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// The animated sprites for a Pokémon.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BlackWhiteAnimatedSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for the sixth generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationVISprites {
    /// Sprites for Pokémon OmegaRuby & Pokémon AlphaSapphire.
    #[serde(rename = "omegaruby-alphasapphire")]
    pub omegaruby_alphasapphire: Option<OmegarubyAlphasapphireSprites>,
    /// Sprites for Pokémon X & Pokémon Y.
    #[serde(rename = "x-y")]
    pub x_y: Option<XYSprites>,
}

/// Sprites for Pokémon OmegaRuby & Pokémon AlphaSapphire.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct OmegarubyAlphasapphireSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for Pokémon X & Pokémon Y.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct XYSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for the seventh generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationVIISprites {
    /// The icons sprites of a Pokémon.
    pub icons: Option<IconsSprites>,
    /// Sprites for Pokémon UltraSun & Pokémon UltraMoon.
    #[serde(rename = "ultra-sun-ultra-moon")]
    pub ultrasun_ultramoon: Option<UltrasunUltramoonSprites>,
}

/// The icons sprites of a Pokémon.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct IconsSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
}

/// Sprites for Pokémon UltraSun & Pokémon UltraMoon.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct UltrasunUltramoonSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// Sprites for the eighth generation.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct GenerationVIIISprites {
    /// The icons sprites of a Pokémon.
    pub icons: Option<IconsSprites>,
}

/// [LocationAreaEncounter official documentation](https://pokeapi.co/docs/v2#locationareaencounter)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct LocationAreaEncounter {
    /// The location area the referenced Pokémon can be encountered in.
    pub location_area: Option<NamedApiResource<LocationArea>>,
    /// A list of versions and encounters with the referenced Pokémon that might happen.
    pub version_details: Option<Vec<VersionEncounterDetail>>,
}

/// [PokemonColor official documentation](https://pokeapi.co/docs/v2#pokemoncolor)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonColor {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that have this color.
    pub pokemon_species: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [PokemonForm official documentation](https://pokeapi.co/docs/v2#pokemonform)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonForm {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The order in which forms should be sorted within all forms. Multiple forms may have equal order,
    /// in which case they should fall back on sorting by name.
    pub order: Option<i64>,
    /// The order in which forms should be sorted within a species' forms.
    pub form_order: Option<i64>,
    /// True for exactly one form used as the default for each Pokémon.
    pub is_default: Option<bool>,
    /// Whether or not this form can only happen during battle.
    pub is_battle_only: Option<bool>,
    /// Whether or not this form requires mega evolution.
    pub is_mega: Option<bool>,
    /// The name of this form.
    pub form_name: Option<String>,
    /// The Pokémon that can take on this form.
    pub pokemon: Option<NamedApiResource<Pokemon>>,
    /// A list of details showing types this Pokémon form has.
    pub types: Option<Vec<PokemonFormType>>,
    /// A set of sprites used to depict this Pokémon form in the game.
    pub sprites: Option<PokemonFormSprites>,
    /// The version group this Pokémon form was introduced in.
    pub version_group: Option<NamedApiResource<VersionGroup>>,
    /// The form specific full name of this Pokémon form, or empty if the form does not have a specific name.
    pub names: Option<Vec<Name>>,
    /// The form specific form name of this Pokémon form, or empty if the form does not have a specific name.
    pub form_names: Option<Vec<Name>>,
}

/// [PokemonFormType official documentation](https://pokeapi.co/docs/v2#pokemonformtype)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonFormType {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The type the referenced Form has.
    #[serde(rename = "type")]
    pub type_: Option<NamedApiResource<Type>>,
}

/// [PokemonFormSprites official documentation](https://pokeapi.co/docs/v2#pokemonformsprites)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonFormSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default female back sprite of a Pokémon.
    pub back_female: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The shiny female back sprite of a Pokémon.
    pub back_shiny_female: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// [PokemonHabitat official documentation](https://pokeapi.co/docs/v2#pokemonhabitat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonHabitat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that can be found in this habitat.
    pub pokemon_species: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [PokemonShape official documentation](https://pokeapi.co/docs/v2#pokemonshape)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonShape {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The "scientific" name of this Pokémon shape listed in different languages.
    pub awesome_names: Option<Vec<AwesomeName>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of the Pokémon species that have this shape.
    pub pokemon_species: Option<Vec<NamedApiResource<PokemonSpecies>>>,
}

/// [AwesomeName official documentation](https://pokeapi.co/docs/v2#awesomename)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct AwesomeName {
    /// The localized "scientific" name for an API resource in a specific language.
    pub awesome_name: Option<String>,
    /// The language this "scientific" name is in.
    pub language: Option<NamedApiResource<Language>>,
}

/// [PokemonSpecies official documentation](https://pokeapi.co/docs/v2#pokemonspecies)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpecies {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// The order in which species should be sorted. Based on National Dex order,
    /// except families are grouped together and sorted by stage.
    pub order: Option<i64>,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: Option<i64>,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: Option<i64>,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_hapiness: Option<i64>,
    /// Whether or not this is a baby Pokémon.
    pub is_baby: Option<bool>,
    /// Whether or not this is a legendary Pokémon.
    pub is_legendary: Option<bool>,
    /// Whether or not this is a mythical Pokémon.
    pub is_mythical: Option<bool>,
    /// Initial hatch counter: one must walk 255 × (hatch_counter + 1) steps before this Pokémon's egg hatches,
    /// unless utilizing bonuses like Flame Body's.
    pub hatch_counter: Option<i64>,
    /// Whether or not this Pokémon has visual gender differences.
    pub has_gender_differences: Option<bool>,
    /// Whether or not this Pokémon has multiple forms and can switch between them.
    pub forms_switchable: Option<bool>,
    /// The rate at which this Pokémon species gains levels.
    pub growth_rate: Option<NamedApiResource<GrowthRate>>,
    /// A list of Pokedexes and the indexes reserved within them for this Pokémon species.
    pub pokedex_numbers: Option<Vec<PokemonSpeciesDexEntry>>,
    /// A list of egg groups this Pokémon species is a member of.
    pub egg_groups: Option<Vec<NamedApiResource<EggGroup>>>,
    /// The color of this Pokémon for Pokédex search.
    pub color: Option<NamedApiResource<PokemonColor>>,
    /// The shape of this Pokémon for Pokédex search.
    pub shape: Option<NamedApiResource<PokemonShape>>,
    /// The Pokémon species that evolves into this Pokemon_species.
    pub evolves_from_species: Option<NamedApiResource<PokemonSpecies>>,
    /// The evolution chain this Pokémon species is a member of.
    pub evolution_chain: Option<ApiResource>,
    /// The habitat this Pokémon species can be encountered in.
    pub habitat: Option<NamedApiResource<PokemonHabitat>>,
    /// The generation this Pokémon species was introduced in.
    pub generation: Option<NamedApiResource<Generation>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of encounters that can be had with this Pokémon species in pal park.
    pub pal_park_encounters: Option<Vec<PalParkEncounterArea>>,
    /// A list of flavor text entries for this Pokémon species.
    pub flavor_text_entries: Option<Vec<FlavorText>>,
    /// Descriptions of different forms Pokémon take on within the Pokémon species.
    pub form_descriptions: Option<Vec<Description>>,
    /// The genus of this Pokémon species listed in multiple languages.
    pub genera: Option<Vec<Genus>>,
    /// A list of the Pokémon that exist within this Pokémon species.
    pub varieties: Option<Vec<PokemonSpeciesVariety>>,
}

/// [Genus official documentation](https://pokeapi.co/docs/v2#genus)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Genus {
    /// The localized genus for the referenced Pokémon species.
    pub genus: Option<String>,
    /// The language this genus is in.
    pub language: Option<NamedApiResource<Language>>,
}

/// [PokemonSpeciesDexEntry official documentation](https://pokeapi.co/docs/v2#pokemonspeciesdexentry)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpeciesDexEntry {
    /// The index number within the Pokédex.
    pub entry_number: Option<i64>,
    /// The Pokédex the referenced Pokémon species can be found in.
    pub pokedex: Option<NamedApiResource<Language>>,
}

/// [PalParkEncounterArea official documentation](https://pokeapi.co/docs/v2#palparkencounterarea)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PalParkEncounterArea {
    /// The base score given to the player when the referenced Pokémon is caught during a pal park run.
    pub base_score: Option<i64>,
    /// The base rate for encountering the referenced Pokémon in this pal park area.
    pub rate: Option<i64>,
    /// The pal park area where this encounter happens.
    pub area: Option<NamedApiResource<PalParkArea>>,
}

/// [PokemonSpeciesVariety official documentation](https://pokeapi.co/docs/v2#pokemonspeciesvariety)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct PokemonSpeciesVariety {
    /// Whether this variety is the default variety.
    pub is_default: Option<bool>,
    /// The Pokémon variety.
    pub pokemon: Option<NamedApiResource<Pokemon>>,
}

/// [Stat official documentation](https://pokeapi.co/docs/v2#stat)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Stat {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// ID the games use for this stat.
    pub game_index: Option<i64>,
    /// Whether this stat only exists within a battle.
    pub is_battle_only: Option<bool>,
    /// A detail of moves which affect this stat positively or negatively.
    pub affecting_moves: Option<MoveStatAffectSets>,
    /// A detail of natures which affect this stat positively or negatively.
    pub affecting_natures: Option<NatureStatAffectSets>,
    /// A list of characteristics that are set on a Pokémon when its highest base stat is this stat.
    pub characteristics: Option<Vec<ApiResource>>,
    /// The class of damage this stat is directly related to.
    pub move_damage_class: Option<NamedApiResource<MoveDamageClass>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
}

/// [MoveStatAffectSets official documentation](https://pokeapi.co/docs/v2#movestataffectsets)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveStatAffectSets {
    /// A list of moves and how they change the referenced stat.
    pub increase: Option<Vec<MoveStatAffect>>,
    /// A list of moves and how they change the referenced stat.
    pub decrease: Option<Vec<MoveStatAffect>>,
}

/// [MoveStatAffect official documentation](https://pokeapi.co/docs/v2#movestataffect)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MoveStatAffect {
    /// The maximum amount of change to the referenced stat.
    pub change: Option<i64>,
    /// The move causing the change.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource<Move>>,
}

/// [NatureStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturestataffectsets)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct NatureStatAffectSets {
    /// A list of natures and how they change the referenced stat.
    pub increase: Option<Vec<NamedApiResource<Nature>>>,
    /// A list of nature sand how they change the referenced stat.
    pub decrease: Option<Vec<NamedApiResource<Nature>>>,
}

/// [Type official documentation](https://pokeapi.co/docs/v2#type)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Type {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The name for this resource.
    pub name: Option<String>,
    /// A detail of how effective this type is toward others and vice versa.
    pub damage_relations: Option<TypeRelations>,
    /// A list of details of how effective this type was toward others and vice versa in previous generations.
    pub past_damage_relations: Option<Vec<TypeRelationsPast>>,
    /// A list of game indices relevent to this item by generation.
    pub game_indices: Option<Vec<GenerationGameIndex>>,
    /// The generation this type was introduced in.
    pub generation: Option<NamedApiResource<Generation>>,
    /// The class of damage inflicted by this type.
    pub move_damage_class: Option<NamedApiResource<MoveDamageClass>>,
    /// The name of this resource listed in different languages.
    pub names: Option<Vec<Name>>,
    /// A list of details of Pokémon that have this type.
    pub pokemon: Option<Vec<TypePokemon>>,
    /// A list of moves that have this type.
    pub moves: Option<Vec<NamedApiResource<Move>>>,
}

/// [TypePokemon official documentation](https://pokeapi.co/docs/v2#typepokemon)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TypePokemon {
    /// The order the Pokémon's types are listed in.
    pub slot: Option<i64>,
    /// The Pokémon that has the referenced type.
    pub pokemon: Option<NamedApiResource<Pokemon>>,
}

/// [TypeRelations official documentation](https://pokeapi.co/docs/v2#typerelations)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TypeRelations {
    /// A list of types this type has no effect on.
    pub no_damage_to: Option<Vec<NamedApiResource<Type>>>,
    /// A list of types this type is not very effect against.
    pub half_damage_to: Option<Vec<NamedApiResource<Type>>>,
    /// A list of types this type is very effect against.
    pub double_damage_to: Option<Vec<NamedApiResource<Type>>>,
    /// A list of types that have no effect on this type.
    pub no_damage_from: Option<Vec<NamedApiResource<Type>>>,
    /// A list of types that are not very effective against this type.
    pub half_damage_from: Option<Vec<NamedApiResource<Type>>>,
    /// A list of types that are very effective against this type.
    pub double_damage_from: Option<Vec<NamedApiResource<Type>>>,
}

/// [TypeRelationsPast official documentation](https://pokeapi.co/docs/v2#typerelationspast)
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TypeRelationsPast {
    /// The last generation in which the referenced type had the listed damage relations.
    pub generation: Option<NamedApiResource<Generation>>,
    /// The damage relations the referenced type had up to and including the listed generation.
    pub damage_relations: Option<TypeRelations>,
}
