use bevy::prelude::*;
use crate::region::{Region, BiomeType};

pub fn setup_region(mut commands: Commands) {
    let region = Region {
        name: "Outer Belt".to_string(),
        biome: BiomeType::Icy,
        threat_level: 4,
        has_station: true,
    };

    commands.insert_resource(region);
}
