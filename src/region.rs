use bevy::prelude::*;

#[derive(Resource)]
pub struct Region {
    pub name: String,
    pub biome: BiomeType,
    pub threat_level: u8,
    pub has_station: bool,
}

#[allow(dead_code)] 
#[derive(Debug)]
pub enum BiomeType {
    Rocky,
    Icy,
    Volcanic,
    GasCloud,
}
