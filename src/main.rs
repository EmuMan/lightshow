use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use lightshow::camera::CameraPlugin;
use lightshow::fixtures::FixturesPlugin;
use lightshow::midi::MidiPlugin;
use lightshow::network::NetworkPlugin;
use lightshow::timeline::{effects::EffectsPlugin, layers::LayersPlugin, playback::PlaybackPlugin};
use lightshow::ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .insert_resource(Time::<Fixed>::from_hz(44.0))
        .add_plugins(DefaultPlugins)
        .add_plugins(LayersPlugin)
        .add_plugins(PlaybackPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FixturesPlugin)
        .add_plugins(EffectsPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(UiPlugin)
        .add_plugins(MidiPlugin)
        .add_plugins(NetworkPlugin)
        .run();
}
