use bevy::prelude::*;

use crate::{layers::*, tests::*};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlaybackInformation>()
            .add_systems(Update, increment_playback_time)
            .add_systems(Startup, pulse_test_startup);
    }
}

#[derive(Resource, Debug)]
pub struct PlaybackInformation {
    pub current_time: f64,
    pub is_playing: bool,
    pub bpm: f64,
    pub beats_per_bar: usize,
    pub current_layer: Option<Entity>,
}

impl Default for PlaybackInformation {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            is_playing: false,
            bpm: 120.0,
            beats_per_bar: 4,
            current_layer: None,
        }
    }
}

pub fn increment_playback_time(
    time: Res<Time>,
    mut playback: ResMut<PlaybackInformation>,
    layers_query: Query<&Layer>,
) {
    let Some(layer) = playback
        .current_layer
        .and_then(|layer| layers_query.get(layer).ok())
    else {
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
