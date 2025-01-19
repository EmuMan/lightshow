use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use lightshow::plugins::simulation::SimulationPlugin;
use lightshow::plugins::camera::CameraPlugin;
use lightshow::plugins::fixtures::FixturesPlugin;
use lightshow::plugins::effects::EffectsPlugin;
use lightshow::plugins::ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FixturesPlugin)
        .add_plugins(EffectsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(UiPlugin)
        .run();
}
