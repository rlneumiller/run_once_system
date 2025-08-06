use bevy::prelude::*;
use crate::region::Region;
use crate::resources::HasRun;

pub fn describe_region(region: Res<Region>) {
    println!("You're in: {}", region.name);
    println!("Biome: {:?}", region.biome);
    println!("Threat Level: {}", region.threat_level);
    println!(
        "Station Present: {}",
        if region.has_station { "Yes" } else { "No" }
    );
}

pub fn should_describe_region(has_run: Res<HasRun>) -> bool {
    !has_run.0
}