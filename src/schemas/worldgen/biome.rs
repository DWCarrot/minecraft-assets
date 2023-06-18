//! Serde-(de)serializable data types for
//! `data/<namespace>/worldgen/biome/*.json`.
//!
//! See <https://minecraft.fandom.com/wiki/Custom_biome>.

use serde::{Deserialize, Serialize};


/// A custom biome info stored in the
/// `data/<namespace>/worldgen/biome/*.json`.
///
/// See also the corresponding section of the [wiki page]
///
/// [wiki page]: <https://minecraft.fandom.com/wiki/Custom_biome>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct CustomeBiome {

    /// Determines whether or not the biome has precipitation.
    pub has_precipitation: bool,

    /// Controls gameplay features like grass and foliage color, and a height adjusted temperature (which controls whether raining or snowing 
    /// if precipitation is rain, and generation details of some features).
    pub temperature: f32,

    /// Modifies temperature before calculating the height adjusted temperature. 
    /// If frozen, makes some places' temperature high enough to rain (0.2).
    #[serde(default)]
    pub temperature_modifier: TemperatureModifier,

    /// Controls grass and foliage color.
    pub downfall: f32,

    /// Ambient effects in this biome. 
    pub effects: Effects,

    /// The carvers to use. 
    pub carvers: Carvers,

    /// (Can be empty) A list of 11 elements. 
    /// Each element can be a tag of placed feature, a list of placed feature IDs, or a list of placed feature objects. 
    /// The features are applied to each chunk in order in each step. 
    /// The same placed feature in the same step in two biomes cannot be in a different order. 
    /// For each step, all feature IDs need to be ordered consistently across biomes. 
    /// For example, in minecraft:plains in UNDERGROUND_ORES step, ore_dirt is before ore_gravel, 
    /// so in other biomes' UNDERGROUND_ORES step, if there are ore_dirt and ore_gravel, 
    /// ore_gravel cannot be after ore_dirt. The generation steps are also used in [stucture features].
    /// 
    /// [structure features]: <https://minecraft.fandom.com/wiki/Custom_structure>
    pub features: Vec<Vec<String>>, 


    /// (optional) Higher value results in more creatures spawned in world generation. 
    /// Must be between 0.0 and 0.9999999 (both inclusive).
    #[serde(default)]
    pub creature_spawn_probability: Option<f32>,

    /// (Required, but can be empty. If this object doesn't contain a certain category, mobs in this category will not be spawned)
    ///  Entity spawning settings. 
    pub spawners: Spawners,
}



/// Modification methods applied to temperature before calculating the height adjusted temperature
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureModifier {
    
    /// No modification
    None,

    ///  Makes some places' temperature high enough to rain (0.2).
    Frozen
}

impl Default for TemperatureModifier {
    
    fn default() -> Self {
        TemperatureModifier::None
    }
}


/// Ambient effects of a biome. 
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct Effects {

    /// Decimal value converted from Hex color to use for fog.
    pub fog_color: u32,

    /// Decimal value converted from Hex color to use for the sky.
    pub sky_color: u32,

    /// Decimal value converted from Hex color to use for water blocks and cauldrons.
    pub water_color: u32,

    /// Decimal value converted from Hex color to use for fog.
    pub water_fog_color: u32,

    /// (optional) Decimal value converted from Hex color to use for tree leaves and vines. 
    /// If not present, the value depends on downfall and the temperature.
    #[serde(default)]
    pub foliage_color: Option<u32>,

    /// (optional) Decimal value converted from Hex color to use for grass blocks, grass, tall grass, ferns, tall ferns, and sugar cane. 
    /// If not present, the value depends on downfall and temperature.
    #[serde(default)]
    pub grass_color: Option<u32>,

    /// (optional, defaults to none) Can be none, dark_forest or swamp.
    #[serde(default)]
    pub grass_color_modifier: EffectsGrassColorModifier,

    /// (optional) The particle to use throughout this biome. 
    #[serde(default)]
    pub particle: Option<EffectsParticle>,

    /// (optional) The namespace ID of the sound event to use for ambient sound.
    #[serde(default)]
    pub ambient_sound: Option<String>,

    /// (optional) Settings for mood sound. 
    #[serde(default)]
    pub mood_sound: Option<EffectsMoodSound>,

    /// (optional) Settings for additions sound. 
    #[serde(default)]
    pub additions_sound: Option<EffectsAdditionsSound>,

    /// (optional) Specific music that should be played in the biome. 
    #[serde(default)]
    pub music: Option<EffectsMusic>,

    /// (Required, but can be empty. If this object doesn't contain a certain category, mobs in this category will not be spawned) 
    /// Entity spawning settings. 
    #[serde(default)]
    pub spawners: Spawners,

    ///  (Required, but can be empty. Only mobs listed here use the spawn cost mechanism) 
    /// See [Spawn#Spawn] costs for details. 
    /// 
    /// [Spawn#Spawn]: <https://minecraft.fandom.com/wiki/Spawn#Spawn_costs>
    #[serde(default)]
    pub spawn_costs: SpawnCosts,
}



/// Modification methods applied to grass color.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EffectsGrassColorModifier {

    /// No modification
    None,

    /// as dark_forest
    DarkForest,

    /// as swamp
    Swamp
}

impl Default for EffectsGrassColorModifier {
    
    fn default() -> Self {
        EffectsGrassColorModifier::None
    }
}



/// The settings for particle to use throughout this biome.
/// 
///  *unimplemented*
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EffectsParticle {

}


/// The settings for mood sound used in effects of biome.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EffectsMoodSound {

    /// The namespace ID of the sound event to use.
    pub sound: String,

    /// The mininum delay between two plays. See also [Ambience#Mood_algorithm].
    /// 
    /// [Ambience#Mood_algorithm]: <https://minecraft.fandom.com/wiki/Ambience#Mood_algorithm>
    pub tick_delay: u32,

    /// Determines the cubic range of possible positions to find place to play the mood sound. 
    /// The player is at the center of the cubic range, and the edge length is `2 * block_search_extent`.
    pub block_search_extent: u32,

    /// The higher the value makes the sound source further away from the player.
    pub offset: f64,
}


/// The settings for additions sound  used in effects of biome.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EffectsAdditionsSound {

    /// The namespace ID of the sound event to use.
    pub sound: String,

    /// The propability to start playing the sound per tick. 
    /// Value higher than 1 is regarded as 1, lower than 0 is regarded as 0.
    pub tick_chance: f64,
}


/// The settings for music that should be played in the biome. 
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EffectsMusic {

    /// The namespace ID of the sound event to use.
    pub sound: String,

    /// The minimum delay between two plays. 
    pub min_delay: u32,

    /// The maximum delay between two plays. 
    pub max_delay: u32,

    /// Whether or not to replace music which is already playing.
    pub replace_current_music: bool,
}



/// The settings for carvers to use in this biome.
/// 
///  *unimplemented*
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct Carvers {

}


/// The settings for spawning entities in this biome.
/// 
///  *unimplemented; should be Hashmap*
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct Spawners {

}



/// The settings for spawning cost in this biome.
/// 
///  *unimplemented; should be Hashmap*
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct SpawnCosts {

}