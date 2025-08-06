use bevy::prelude::*;
use crate::resources::*;

pub fn run_once_system(mut has_run: ResMut<HasRun>) {
    if !has_run.0 {
        println!("Running once during gameplay!");
        has_run.0 = true;
    }
}