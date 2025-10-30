use bevy::prelude::*;

use bevy_egui::EguiPlugin;

use crate::audio::AudioPlugin;
use crate::camera::CameraPlugin;
use crate::fixtures::FixturesPlugin;
use crate::midi::MidiPlugin;
use crate::network::NetworkPlugin;
use crate::timeline::TimelinePlugin;
use crate::ui::UiPlugin;

pub mod audio;
pub mod camera;
pub mod fixtures;
pub mod midi;
pub mod network;
pub mod simple_store;
pub mod tests;
pub mod timeline;
pub mod ui;
pub mod util;

pub struct LightshowPlugin;

impl Plugin for LightshowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
            .insert_resource(Time::<Fixed>::from_hz(44.0))
            .add_plugins(DefaultPlugins)
            .add_plugins(CameraPlugin)
            .add_plugins(TimelinePlugin)
            .add_plugins(FixturesPlugin)
            .add_plugins(EguiPlugin::default())
            .add_plugins(UiPlugin)
            .add_plugins(AudioPlugin)
            .add_plugins(MidiPlugin)
            .add_plugins(NetworkPlugin);
    }
}
