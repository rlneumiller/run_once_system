use bevy::prelude::*;
use resources::HasRun;
use systems::describe::should_describe_region;

mod region;
mod resources;
mod systems {
    pub mod setup;
    pub mod describe;
    pub mod run_once;
}



fn main() {
    
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, systems::setup::setup_region);
    app.insert_resource(HasRun(false));
    app.add_systems(Update, systems::run_once::run_once_system);
    app.add_systems(Update, systems::describe::describe_region.run_if(should_describe_region));
    app.run();
}

