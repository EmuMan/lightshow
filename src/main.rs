pub mod simulation;
pub mod camera;

use bevy::prelude::*;

use simulation::*;
use camera::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
