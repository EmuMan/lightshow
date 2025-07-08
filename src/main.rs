use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use lightshow::camera::CameraPlugin;
use lightshow::effects::EffectsPlugin;
use lightshow::fixtures::FixturesPlugin;
use lightshow::layers::LayersPlugin;
use lightshow::midi::MidiPlugin;
use lightshow::network::NetworkPlugin;
use lightshow::simulation::SimulationPlugin;
use lightshow::ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .insert_resource(Time::<Fixed>::from_hz(44.0))
        .add_plugins(DefaultPlugins)
        .add_plugins(LayersPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FixturesPlugin)
        .add_plugins(EffectsPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(UiPlugin)
        .add_plugins(MidiPlugin)
        .add_plugins(NetworkPlugin)
        .run();
}
