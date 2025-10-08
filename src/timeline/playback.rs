use bevy::prelude::*;

use crate::{simple_store::SimpleStore, tests, timeline::layers::*};

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlaybackInformation>()
            .add_systems(Update, increment_playback_time)
            .add_systems(Startup, tests::single_pulse::pulse_test_startup);
    }
}

#[derive(Resource, Debug)]
pub struct PlaybackInformation {
    pub current_time: f64,
    pub is_playing: bool,
    pub bpm: f64,
    pub beats_per_bar: usize,
}

impl Default for PlaybackInformation {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            is_playing: false,
            bpm: 120.0,
            beats_per_bar: 4,
        }
    }
}

pub fn increment_playback_time(
    time: Res<Time>,
    mut playback: ResMut<PlaybackInformation>,
    primary_layer: Res<PrimaryLayer>,
    layer_store: Res<SimpleStore<Layer>>,
) {
    let Some(layer) = primary_layer.0.and_then(|handle| layer_store.get(handle)) else {
        playback.current_time = 0.0;
        playback.is_playing = false;
        return;
    };
    if playback.is_playing {
        playback.current_time += time.delta_secs_f64();
        if playback.current_time > layer.length {
            playback.current_time = 0.0;
        }
    }
}
