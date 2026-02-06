//! Pokemon group models

use super::{
    berries::BerryFlavor,
    evolution::EvolutionChain,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Ability {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// Whether or not this ability originated in the main series of the video games.
    pub is_main_series: bool,
    /// The generation this ability originated in.
    pub generation: NamedApiResource<Generation>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// The effect of this ability listed in different languages.
    pub effect_entries: Vec<VerboseEffect>,
    /// The list of previous effects this ability has had across version groups.
    pub effect_changes: Vec<AbilityEffectChange>,
    /// The flavor text of this ability listed in different languages.
    pub flavor_text_entries: Vec<AbilityFlavorText>,
    /// A list of Pokémon that could potentially have this ability.
    pub pokemon: Vec<AbilityPokemon>,
}

/// [AbilityEffectChange official documentation](https://pokeapi.co/docs/v2#abilityeffectchange)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AbilityEffectChange {
    /// The previous effect of this ability listed in different languages.
    pub effect_entries: Vec<Effect>,
    /// The version group in which the previous effect of this ability originated.
    pub version_group: NamedApiResource<VersionGroup>,
}

/// [AbilityFlavorText official documentation](https://pokeapi.co/docs/v2#abilityflavortext)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AbilityFlavorText {
    /// The localized name for an API resource in a specific language.
    pub flavor_text: String,
    /// The language this text resource is in.
    pub language: NamedApiResource<Language>,
    /// The version group that uses this flavor text.
    pub version_group: NamedApiResource<VersionGroup>,
}

/// [AbilityPokemon official documentation](https://pokeapi.co/docs/v2#abilitypokemon)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AbilityPokemon {
    /// Whether or not this a hidden ability for the referenced Pokémon.
    pub is_hidden: bool,
    /// Pokémon have 3 ability 'slots' which hold references to possible abilities they could have.
    /// This is the slot of this ability for the referenced pokemon.
    pub slot: i64,
    /// The Pokémon this ability could belong to.
    pub pokemon: NamedApiResource<Pokemon>,
}

/// [Characteristic official documentation](https://pokeapi.co/docs/v2#characteristic)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Characteristic {
    /// The identifier for this resource.
    pub id: i64,
    /// The remainder of the highest stat/IV divided by 5.
    pub gene_modulo: i64,
    /// The possible values of the highest stat that would result in a Pokémon
    /// receiving this characteristic when divided by 5.
    pub possible_values: Vec<i64>,
    /// The description of this resource listed in different languages.
    pub descriptions: Vec<Description>,
    /// The highest stat referenced by this characteristic.
    pub highest_stat: NamedApiResource<Stat>,
}

/// [EggGroup official documentation](https://pokeapi.co/docs/v2#egggroup)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct EggGroup {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of all Pokémon species that are members of this egg group.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [Gender official documentation](https://pokeapi.co/docs/v2#gender)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Gender {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A list of Pokémon species that can be this gender and how likely it is that they will be.
    pub pokemon_species_details: Vec<PokemonSpeciesGender>,
    /// A list of Pokémon species that required this gender in order for a Pokémon to evolve into them.
    pub required_for_evolution: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [PokemonSpeciesGender official documentation](https://pokeapi.co/docs/v2#pokemonspeciesgender)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonSpeciesGender {
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub rate: i64,
    /// A Pokémon species that can be the referenced gender.
    pub pokemon_species: NamedApiResource<PokemonSpecies>,
}

/// [GrowthRate official documentation](https://pokeapi.co/docs/v2#growthrate)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GrowthRate {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The formula used to calculate the rate at which the Pokémon species gains level.
    pub formula: String,
    /// The descriptions of this characteristic listed in different languages.
    pub descriptions: Vec<Description>,
    /// A list of levels and the amount of experienced needed to atain them based on this growth rate.
    pub levels: Vec<GrowthRateExperienceLevel>,
    /// A list of Pokémon species that gain levels at this growth rate.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [GrowthRateExperienceLevel official documentation](https://pokeapi.co/docs/v2#growthrateexperiencelevel)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GrowthRateExperienceLevel {
    /// The level gained.
    pub level: i64,
    /// The amount of experience required to reach the referenced level.
    pub experience: i64,
}

/// [Nature official documentation](https://pokeapi.co/docs/v2#nature)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Nature {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The stat decreased by 10% in Pokémon with this nature.
    pub decreased_stat: Option<NamedApiResource<Stat>>,
    /// The stat increased by 10% in Pokémon with this nature.
    pub increased_stat: Option<NamedApiResource<Stat>>,
    /// The flavor hated by Pokémon with this nature.
    pub hates_flavor: Option<NamedApiResource<BerryFlavor>>,
    /// The flavor liked by Pokémon with this nature.
    pub likes_flavor: Option<NamedApiResource<BerryFlavor>>,
    /// A list of Pokéathlon stats this nature effects and how much it effects them.
    pub pokeathlon_stat_changes: Vec<NatureStatChange>,
    /// A list of battle styles and how likely a Pokémon with this nature is to use them in the Battle Palace or Battle Tent.
    pub move_battle_style_preferences: Vec<MoveBattleStylePreference>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// [NatureStatChange official documentation](https://pokeapi.co/docs/v2#naturestatchange)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NatureStatChange {
    /// The amount of change.
    pub max_change: i64,
    /// The stat being affected.
    pub pokeathlon_stat: NamedApiResource<PokeathlonStat>,
}

/// [MoveBattleStylePreference official documentation](https://pokeapi.co/docs/v2#movebattlestylepreference)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MoveBattleStylePreference {
    /// Chance of using the move, in percent, if HP is under one half.
    pub low_hp_preference: i64,
    /// Chance of using the move, in percent, if HP is over one half.
    pub high_hp_preference: i64,
    /// The move battle style.
    pub move_battle_style: NamedApiResource<MoveBattleStyle>,
}

/// [PokeathlonStat official documentation](https://pokeapi.co/docs/v2#pokeathlonstat)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokeathlonStat {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A detail of natures which affect this Pokéathlon stat positively or negatively.
    pub affecting_natures: NaturePokeathlonStatAffectSets,
}

/// [NaturePokeathlonStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturepokeathlonstataffectsets)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NaturePokeathlonStatAffectSets {
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub increase: Vec<NaturePokeathlonStatAffect>,
    /// A list of natures and how they change the referenced Pokéathlon stat.
    pub decrease: Vec<NaturePokeathlonStatAffect>,
}

/// [NaturePokeathlonStatAffect official documentation](https://pokeapi.co/docs/v2#naturepokeathlonstataffect)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NaturePokeathlonStatAffect {
    /// The maximum amount of change to the referenced Pokéathlon stat.
    pub max_change: i64,
    /// The nature causing the change.
    pub nature: NamedApiResource<Nature>,
}

/// [Pokemon official documentation](https://pokeapi.co/docs/v2#pokemon)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Pokemon {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: Option<i64>,
    /// The height of this Pokémon in decimetres.
    pub height: i64,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: bool,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub order: i64,
    /// The weight of this Pokémon in hectograms.
    pub weight: i64,
    /// A list of abilities this Pokémon could potentially have.
    pub abilities: Vec<PokemonAbility>,
    /// A list of forms this Pokémon can take on.
    pub forms: Vec<NamedApiResource<PokemonForm>>,
    /// A list of game indices relevent to Pokémon item by generation.
    pub game_indices: Vec<VersionGameIndex>,
    /// A list of items this Pokémon may be holding when encountered.
    pub held_items: Vec<PokemonHeldItem>,
    /// A link to a list of location areas, as well as encounter details pertaining to specific versions.
    pub location_area_encounters: String,
    /// A list of moves along with learn methods and level details pertaining to specific version groups.
    pub moves: Vec<PokemonMove>,
    /// A list of details showing abilities this pokémon had in previous generations
    pub past_abilities: Vec<PokemonAbilityPast>,
    /// A list of details showing stats this pokémon had in previous generations
    pub past_stats: Vec<PokemonStatPast>,
    /// A list of details showing types this pokémon had in previous generations.
    pub past_types: Vec<PokemonTypePast>,
    /// A set of sprites used to depict this Pokémon in the game.
    /// A visual representation of the various sprites can be found at [PokeAPI/sprites](https://github.com/PokeAPI/sprites).
    pub sprites: PokemonSprites,
    /// A set of cries used to depict this Pokémon in the game.
    /// A visual representation of the various cries can be found at [PokeAPI/cries](https://github.com/PokeAPI/cries).
    pub cries: PokemonCries,
    /// The species this Pokémon belongs to.
    pub species: NamedApiResource<PokemonSpecies>,
    /// A list of base stat values for this Pokémon.
    pub stats: Vec<PokemonStat>,
    /// A list of details showing types this Pokémon has.
    pub types: Vec<PokemonType>,
}

/// [PokemonAbility official documentation](https://pokeapi.co/docs/v2#pokemonability)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonAbility {
    /// Whether or not this is a hidden ability.
    pub is_hidden: bool,
    /// The slot this ability occupies in this Pokémon species.
    pub slot: i64,
    /// The ability the Pokémon may have.
    pub ability: Option<NamedApiResource<Ability>>,
}

/// [PokemonAbilityPast official documentation](https://pokeapi.co/docs/v2#pokemonabilitypast)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonAbilityPast {
    /// The last generation in which the referenced pokémon had the listed abilities.
    pub generation: NamedApiResource<Generation>,
    /// The abilities the referenced pokémon had up to and including the listed generation.
    /// If null, the slot was previously empty.
    pub abilities: Vec<PokemonAbility>,
}

/// [PokemonType official documentation](https://pokeapi.co/docs/v2#pokemontype)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonType {
    /// The order the Pokémon's types are listed in.
    pub slot: i64,
    /// The type the referenced Pokémon has.
    #[serde(rename = "type")]
    pub type_: NamedApiResource<Type>,
}

/// [PokemonTypePast official documentation](https://pokeapi.co/docs/v2#pokemontypepast)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonTypePast {
    /// The last generation in which the referenced pokémon had the listed types.
    pub generation: NamedApiResource<Generation>,
    /// The types the referenced pokémon had up to and including the listed generation.
    pub types: Vec<PokemonType>,
}

/// [PokemonHeldItem official documentation](https://pokeapi.co/docs/v2#pokemonhelditem)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonHeldItem {
    /// The item the referenced Pokémon holds.
    pub item: NamedApiResource<Item>,
    /// The details of the different versions in which the item is held.
    pub version_details: Vec<PokemonHeldItemVersion>,
}

/// [PokemonHeldItemVersion official documentation](https://pokeapi.co/docs/v2#pokemonhelditemversion)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonHeldItemVersion {
    /// The version in which the item is held.
    pub version: NamedApiResource<Version>,
    /// How often the item is held.
    pub rarity: i64,
}

/// [PokemonMove official documentation](https://pokeapi.co/docs/v2#pokemonmove)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonMove {
    /// The move the Pokémon can learn.
    #[serde(rename = "move")]
    pub move_: NamedApiResource<Move>,
    /// The details of the version in which the Pokémon can learn the move.
    pub version_group_details: Vec<PokemonMoveVersion>,
}

/// [PokemonMoveVersion official documentation](https://pokeapi.co/docs/v2#pokemonmoveversion)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonMoveVersion {
    /// The method by which the move is learned.
    pub move_learn_method: NamedApiResource<MoveLearnMethod>,
    /// The version group in which the move is learned.
    pub version_group: NamedApiResource<VersionGroup>,
    /// The minimum level to learn the move.
    pub level_learned_at: i64,
}

/// [PokemonStat official documentation](https://pokeapi.co/docs/v2#pokemonstat)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonStat {
    /// The stat the Pokémon has.
    pub stat: NamedApiResource<Stat>,
    /// The effort points (EV) the Pokémon has in the stat.
    pub effort: i64,
    /// The base value of the stat.
    pub base_stat: i64,
}

/// [PokemonStatPast official documentation](https://pokeapi.co/docs/v2#pokemonstatpast)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonStatPast {
    /// The last generation in which the referenced pokémon had the listed stats.
    pub generation: NamedApiResource<Generation>,
    /// The stat the Pokémon had up to and including the listed generation.
    pub stats: Vec<PokemonStat>,
}

/// [PokemonSprites official documentation](https://pokeapi.co/docs/v2#pokemonsprites)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
    pub other: OtherSprites,
    /// Sprites per version.
    pub versions: VersionsSprites,
}

/// References sprites that doesn't come from game.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct OtherSprites {
    /// Dream world sprites of this Pokémon.
    pub dream_world: DreamWorldSprites,
    /// Home sprites of this Pokémon.
    pub home: HomeSprites,
    /// The official artwork of this Pokémon.
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtworkSprites,
}

/// References the dream world sprites of a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct DreamWorldSprites {
    /// The default despiction of this Pokémon from dream world.
    pub front_default: Option<String>,
    /// The female despiction of this Pokémon from dream world.
    pub front_female: Option<String>,
}

/// References the home sprites of a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct HomeSprites {
    /// The default despiction of this Pokémon from dream world.
    pub front_default: Option<String>,
    /// The female despiction of this Pokémon from dream world.
    pub front_female: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The shiny female front sprite of a Pokémon.
    pub front_shiny_female: Option<String>,
}

/// References the official artwork of a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct OfficialArtworkSprites {
    /// The default despiction of this Pokémon form the official artwork.
    pub front_default: Option<String>,
}

/// Sprites of a Pokémon, per generation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct VersionsSprites {
    /// Sprites for the first generation.
    #[serde(rename = "generation-i")]
    pub generation_i: GenerationISprites,
    /// Sprites for the second generation.
    #[serde(rename = "generation-ii")]
    pub generation_ii: GenerationIISprites,
    /// Sprites for the third generation.
    #[serde(rename = "generation-iii")]
    pub generation_iii: GenerationIIISprites,
    /// Sprites for the fourth generation.
    #[serde(rename = "generation-iv")]
    pub generation_iv: GenerationIVSprites,
    /// Sprites for the fifth generation.
    #[serde(rename = "generation-v")]
    pub generation_v: GenerationVSprites,
    /// Sprites for the sixth generation.
    #[serde(rename = "generation-vi")]
    pub generation_vi: GenerationVISprites,
    /// Sprites for the seventh generation.
    #[serde(rename = "generation-vii")]
    pub generation_vii: GenerationVIISprites,
    /// Sprites for the eighth generation.
    #[serde(rename = "generation-viii")]
    pub generation_viii: GenerationVIIISprites,
}

/// Sprites for the first generation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationISprites {
    /// Sprites for Pokémon Red & Pokémon Blue.
    #[serde(rename = "red-blue")]
    pub red_blue: RedBlueSprites,
    /// Sprites for Pokémon Yellow.
    pub yellow: YellowSprites,
}

/// Sprites for Pokémon Red & Pokémon Blue.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct RedBlueSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default back sprite of a Pokémon, transparent.
    pub back_transparent: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
    /// The default front sprite of a Pokémon, transparent.
    pub front_transparent: Option<String>,
}

/// Sprites for Pokémon Yellow.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct YellowSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The default back sprite of a Pokémon, in gray.
    pub back_gray: Option<String>,
    /// The default back sprite of a Pokémon, transparent.
    pub back_transparent: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default front sprite of a Pokémon, in gray.
    pub front_gray: Option<String>,
    /// The default front sprite of a Pokémon, transparent.
    pub front_transparent: Option<String>,
}

/// Sprites for the second generation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationIISprites {
    /// Sprites for Pokémon Crystal.
    pub crystal: CrystalSprites,
    /// Sprites for Pokémon Gold.
    pub gold: GoldSprites,
    /// Sprites for Pokémon Silver.
    pub silver: SilverSprites,
}

/// Sprites for Pokémon Crystal.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CrystalSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default back sprite of a Pokémon, transparent.
    pub back_transparent: Option<String>,
    /// The shiny back sprite of a Pokémon, transparent.
    pub back_shiny_transparent: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The default front sprite of a Pokémon, transparent.
    pub front_transparent: Option<String>,
    /// The shiny front sprite of a Pokémon, transparent.
    pub front_shiny_transparent: Option<String>,
}

/// Sprites for Pokémon Gold.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GoldSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The default front sprite of a Pokémon, transparent.
    pub front_transparent: Option<String>,
}

/// Sprites for Pokémon Silver.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct SilverSprites {
    /// The default back sprite of a Pokémon.
    pub back_default: Option<String>,
    /// The shiny back sprite of a Pokémon.
    pub back_shiny: Option<String>,
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
    /// The default front sprite of a Pokémon, transparent.
    pub front_transparent: Option<String>,
}

/// Sprites for the third generation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationIIISprites {
    /// Sprites for Pokémon Emerald.
    pub emerald: EmeraldSprites,
    /// Sprites for Pokémon `FireRed` & Pokémon `LeafGreen`.
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: FireredLeafgreenSprites,
    /// Sprites for Pokémon Ruby & Pokémon Sapphire.
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: RubySapphireSprites,
}

/// Sprites for Pokémon Emerald.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct EmeraldSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The shiny front sprite of a Pokémon.
    pub front_shiny: Option<String>,
}

/// Sprites for Pokémon `FireRed` & Pokémon `LeafGreen`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationIVSprites {
    /// Sprites for Pokémon Diamond & Pokémon Pearl.
    #[serde(rename = "diamond-pearl")]
    pub diamond_pearl: DiamondPearlSprites,
    /// Sprites for Pokémon Platinum.
    pub platinum: PlatinumSprites,
    /// Sprites for Pokémon `HeartGold` & Pokémon `SoulSilver`.
    #[serde(rename = "heartgold-soulsilver")]
    pub heartgold_soulsilver: HeartgoldSoulsilverSprites,
}

/// Sprites for Pokémon Diamond & Pokémon Pearl.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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

/// Sprites for Pokémon `HeartGold` & Pokémon `SoulSilver`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationVSprites {
    /// Sprites for Pokémon Black & Pokémon White.
    #[serde(rename = "black-white")]
    pub black_white: BlackWhiteSprites,
}

/// Sprites for Pokémon Black & Pokémon White.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct BlackWhiteSprites {
    /// The animated sprites for a Pokémon.
    pub animated: BlackWhiteAnimatedSprites,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationVISprites {
    /// Sprites for Pokémon `OmegaRuby` & Pokémon `AlphaSapphire`.
    #[serde(rename = "omegaruby-alphasapphire")]
    pub omegaruby_alphasapphire: OmegarubyAlphasapphireSprites,
    /// Sprites for Pokémon X & Pokémon Y.
    #[serde(rename = "x-y")]
    pub x_y: XYSprites,
}

/// Sprites for Pokémon `OmegaRuby` & Pokémon `AlphaSapphire`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationVIISprites {
    /// The icons sprites of a Pokémon.
    pub icons: IconsSprites,
    /// Sprites for Pokémon `UltraSun` & Pokémon `UltraMoon`.
    #[serde(rename = "ultra-sun-ultra-moon")]
    pub ultrasun_ultramoon: UltrasunUltramoonSprites,
}

/// The icons sprites of a Pokémon.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct IconsSprites {
    /// The default front sprite of a Pokémon.
    pub front_default: Option<String>,
    /// The default female front sprite of a Pokémon.
    pub front_female: Option<String>,
}

/// Sprites for Pokémon `UltraSun` & Pokémon `UltraMoon`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GenerationVIIISprites {
    /// The icons sprites of a Pokémon.
    pub icons: IconsSprites,
}

/// [PokemonCries official documentation](https://pokeapi.co/docs/v2#pokemoncries)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonCries {
    /// The latest depiction of this Pokémon's cry.
    pub latest: Option<String>,
    /// The legacy depiction of this Pokémon's cry.
    pub legacy: Option<String>,
}

/// [LocationAreaEncounter official documentation](https://pokeapi.co/docs/v2#locationareaencounter)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct LocationAreaEncounter {
    /// The location area the referenced Pokémon can be encountered in.
    pub location_area: NamedApiResource<LocationArea>,
    /// A list of versions and encounters with the referenced Pokémon that might happen.
    pub version_details: Vec<VersionEncounterDetail>,
}

/// [PokemonColor official documentation](https://pokeapi.co/docs/v2#pokemoncolor)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonColor {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of the Pokémon species that have this color.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [PokemonForm official documentation](https://pokeapi.co/docs/v2#pokemonform)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonForm {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The order in which forms should be sorted within all forms. Multiple forms may have equal order,
    /// in which case they should fall back on sorting by name.
    pub order: i64,
    /// The order in which forms should be sorted within a species' forms.
    pub form_order: i64,
    /// True for exactly one form used as the default for each Pokémon.
    pub is_default: bool,
    /// Whether or not this form can only happen during battle.
    pub is_battle_only: bool,
    /// Whether or not this form requires mega evolution.
    pub is_mega: bool,
    /// The name of this form.
    pub form_name: String,
    /// The Pokémon that can take on this form.
    pub pokemon: NamedApiResource<Pokemon>,
    /// A list of details showing types this Pokémon form has.
    pub types: Vec<PokemonFormType>,
    /// A set of sprites used to depict this Pokémon form in the game.
    pub sprites: PokemonFormSprites,
    /// The version group this Pokémon form was introduced in.
    pub version_group: NamedApiResource<VersionGroup>,
    /// The form specific full name of this Pokémon form, or empty if the form does not have a specific name.
    pub names: Vec<Name>,
    /// The form specific form name of this Pokémon form, or empty if the form does not have a specific name.
    pub form_names: Vec<Name>,
}

/// [PokemonFormType official documentation](https://pokeapi.co/docs/v2#pokemonformtype)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonFormType {
    /// The order the Pokémon's types are listed in.
    pub slot: i64,
    /// The type the referenced Form has.
    #[serde(rename = "type")]
    pub type_: NamedApiResource<Type>,
}

/// [PokemonFormSprites official documentation](https://pokeapi.co/docs/v2#pokemonformsprites)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonHabitat {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of the Pokémon species that can be found in this habitat.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [PokemonShape official documentation](https://pokeapi.co/docs/v2#pokemonshape)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonShape {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The "scientific" name of this Pokémon shape listed in different languages.
    pub awesome_names: Vec<AwesomeName>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of the Pokémon species that have this shape.
    pub pokemon_species: Vec<NamedApiResource<PokemonSpecies>>,
}

/// [AwesomeName official documentation](https://pokeapi.co/docs/v2#awesomename)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AwesomeName {
    /// The localized "scientific" name for an API resource in a specific language.
    pub awesome_name: String,
    /// The language this "scientific" name is in.
    pub language: NamedApiResource<Language>,
}

/// [PokemonSpecies official documentation](https://pokeapi.co/docs/v2#pokemonspecies)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonSpecies {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// The order in which species should be sorted. Based on National Dex order,
    /// except families are grouped together and sorted by stage.
    pub order: i64,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: i64,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: i64,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_hapiness: Option<i64>,
    /// Whether or not this is a baby Pokémon.
    pub is_baby: bool,
    /// Whether or not this is a legendary Pokémon.
    pub is_legendary: bool,
    /// Whether or not this is a mythical Pokémon.
    pub is_mythical: bool,
    /// Initial hatch counter: one must walk 255 × (`hatch_counter` + 1) steps before this Pokémon's egg hatches,
    /// unless utilizing bonuses like Flame Body's.
    pub hatch_counter: Option<i64>,
    /// Whether or not this Pokémon has visual gender differences.
    pub has_gender_differences: bool,
    /// Whether or not this Pokémon has multiple forms and can switch between them.
    pub forms_switchable: bool,
    /// The rate at which this Pokémon species gains levels.
    pub growth_rate: NamedApiResource<GrowthRate>,
    /// A list of Pokedexes and the indexes reserved within them for this Pokémon species.
    pub pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    /// A list of egg groups this Pokémon species is a member of.
    pub egg_groups: Vec<NamedApiResource<EggGroup>>,
    /// The color of this Pokémon for Pokédex search.
    pub color: NamedApiResource<PokemonColor>,
    /// The shape of this Pokémon for Pokédex search.
    pub shape: Option<NamedApiResource<PokemonShape>>,
    /// The Pokémon species that evolves into this `Pokemon_species`.
    pub evolves_from_species: Option<NamedApiResource<PokemonSpecies>>,
    /// The evolution chain this Pokémon species is a member of.
    pub evolution_chain: Option<ApiResource<EvolutionChain>>,
    /// The habitat this Pokémon species can be encountered in.
    pub habitat: Option<NamedApiResource<PokemonHabitat>>,
    /// The generation this Pokémon species was introduced in.
    pub generation: NamedApiResource<Generation>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of encounters that can be had with this Pokémon species in pal park.
    pub pal_park_encounters: Vec<PalParkEncounterArea>,
    /// A list of flavor text entries for this Pokémon species.
    pub flavor_text_entries: Vec<FlavorText>,
    /// Descriptions of different forms Pokémon take on within the Pokémon species.
    pub form_descriptions: Vec<Description>,
    /// The genus of this Pokémon species listed in multiple languages.
    pub genera: Vec<Genus>,
    /// A list of the Pokémon that exist within this Pokémon species.
    pub varieties: Vec<PokemonSpeciesVariety>,
}

/// [Genus official documentation](https://pokeapi.co/docs/v2#genus)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Genus {
    /// The localized genus for the referenced Pokémon species.
    pub genus: String,
    /// The language this genus is in.
    pub language: NamedApiResource<Language>,
}

/// [PokemonSpeciesDexEntry official documentation](https://pokeapi.co/docs/v2#pokemonspeciesdexentry)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonSpeciesDexEntry {
    /// The index number within the Pokédex.
    pub entry_number: i64,
    /// The Pokédex the referenced Pokémon species can be found in.
    pub pokedex: NamedApiResource<Language>,
}

/// [PalParkEncounterArea official documentation](https://pokeapi.co/docs/v2#palparkencounterarea)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PalParkEncounterArea {
    /// The base score given to the player when the referenced Pokémon is caught during a pal park run.
    pub base_score: i64,
    /// The base rate for encountering the referenced Pokémon in this pal park area.
    pub rate: i64,
    /// The pal park area where this encounter happens.
    pub area: NamedApiResource<PalParkArea>,
}

/// [PokemonSpeciesVariety official documentation](https://pokeapi.co/docs/v2#pokemonspeciesvariety)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PokemonSpeciesVariety {
    /// Whether this variety is the default variety.
    pub is_default: bool,
    /// The Pokémon variety.
    pub pokemon: NamedApiResource<Pokemon>,
}

/// [Stat official documentation](https://pokeapi.co/docs/v2#stat)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Stat {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// ID the games use for this stat.
    pub game_index: i64,
    /// Whether this stat only exists within a battle.
    pub is_battle_only: bool,
    /// A detail of moves which affect this stat positively or negatively.
    pub affecting_moves: MoveStatAffectSets,
    /// A detail of natures which affect this stat positively or negatively.
    pub affecting_natures: NatureStatAffectSets,
    /// A list of characteristics that are set on a Pokémon when its highest base stat is this stat.
    pub characteristics: Vec<ApiResource<Characteristic>>,
    /// The class of damage this stat is directly related to.
    pub move_damage_class: Option<NamedApiResource<MoveDamageClass>>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// [MoveStatAffectSets official documentation](https://pokeapi.co/docs/v2#movestataffectsets)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MoveStatAffectSets {
    /// A list of moves and how they change the referenced stat.
    pub increase: Vec<MoveStatAffect>,
    /// A list of moves and how they change the referenced stat.
    pub decrease: Vec<MoveStatAffect>,
}

/// [MoveStatAffect official documentation](https://pokeapi.co/docs/v2#movestataffect)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MoveStatAffect {
    /// The maximum amount of change to the referenced stat.
    pub change: i64,
    /// The move causing the change.
    #[serde(rename = "move")]
    pub move_: NamedApiResource<Move>,
}

/// [NatureStatAffectSets official documentation](https://pokeapi.co/docs/v2#naturestataffectsets)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NatureStatAffectSets {
    /// A list of natures and how they change the referenced stat.
    pub increase: Vec<NamedApiResource<Nature>>,
    /// A list of nature sand how they change the referenced stat.
    pub decrease: Vec<NamedApiResource<Nature>>,
}

/// [Type official documentation](https://pokeapi.co/docs/v2#type)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Type {
    /// The identifier for this resource.
    pub id: i64,
    /// The name for this resource.
    pub name: String,
    /// A detail of how effective this type is toward others and vice versa.
    pub damage_relations: TypeRelations,
    /// A list of details of how effective this type was toward others and vice versa in previous generations.
    pub past_damage_relations: Vec<TypeRelationsPast>,
    /// A list of game indices relevent to this item by generation.
    pub game_indices: Vec<GenerationGameIndex>,
    /// The generation this type was introduced in.
    pub generation: NamedApiResource<Generation>,
    /// The class of damage inflicted by this type.
    pub move_damage_class: Option<NamedApiResource<MoveDamageClass>>,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of details of Pokémon that have this type.
    pub pokemon: Vec<TypePokemon>,
    /// A list of moves that have this type.
    pub moves: Vec<NamedApiResource<Move>>,
}

/// [TypePokemon official documentation](https://pokeapi.co/docs/v2#typepokemon)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TypePokemon {
    /// The order the Pokémon's types are listed in.
    pub slot: i64,
    /// The Pokémon that has the referenced type.
    pub pokemon: NamedApiResource<Pokemon>,
}

/// [TypeRelations official documentation](https://pokeapi.co/docs/v2#typerelations)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TypeRelations {
    /// A list of types this type has no effect on.
    pub no_damage_to: Vec<NamedApiResource<Type>>,
    /// A list of types this type is not very effect against.
    pub half_damage_to: Vec<NamedApiResource<Type>>,
    /// A list of types this type is very effect against.
    pub double_damage_to: Vec<NamedApiResource<Type>>,
    /// A list of types that have no effect on this type.
    pub no_damage_from: Vec<NamedApiResource<Type>>,
    /// A list of types that are not very effective against this type.
    pub half_damage_from: Vec<NamedApiResource<Type>>,
    /// A list of types that are very effective against this type.
    pub double_damage_from: Vec<NamedApiResource<Type>>,
}

/// [TypeRelationsPast official documentation](https://pokeapi.co/docs/v2#typerelationspast)
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TypeRelationsPast {
    /// The last generation in which the referenced type had the listed damage relations.
    pub generation: NamedApiResource<Generation>,
    /// The damage relations the referenced type had up to and including the listed generation.
    pub damage_relations: TypeRelations,
}
