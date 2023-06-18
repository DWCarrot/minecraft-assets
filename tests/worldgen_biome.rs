#![cfg(feature = "tests-worldgen-biome")]

use maplit::hashmap;

use minecraft_assets::schemas::worldgen::biome::CustomeBiome;

mod common;


fn parse_all_worldgen_biome_in_version(version: &str) {
    common::parse_all_in_dir::<CustomeBiome>(&format!(
        "tests/assets-{}/data/minecraft/worldgen/biome",
        version
    ));
}

#[test]
fn can_parse_all_worldgen_biome_1_18() {
    parse_all_worldgen_biome_in_version("1.18");
}