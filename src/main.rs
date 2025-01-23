use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use lightshow::plugins::simulation::SimulationPlugin;
use lightshow::plugins::camera::CameraPlugin;
use lightshow::plugins::fixtures::FixturesPlugin;
use lightshow::plugins::effects::EffectsPlugin;
use lightshow::plugins::ui::UiPlugin;
use lightshow::plugins::network::NetworkPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .insert_resource(Time::<Fixed>::from_hz(44.0))
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FixturesPlugin)
        .add_plugins(EffectsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(NetworkPlugin)
        .run();
}
