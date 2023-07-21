pub mod simulation;
pub mod systems;

use bevy::prelude::*;

use simulation::*;
use systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
