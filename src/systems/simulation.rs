use bevy::prelude::*;

use crate::resources::simulation::*;
use crate::components::layers::*;

pub fn increment_playback_time(
    time: Res<Time>,
    mut playback: ResMut<PlaybackInformation>,
    layers_query: Query<&Layer>,
) {
    let Some(layer) = playback.current_layer
        .and_then(|layer| layers_query.get(layer).ok()) else {
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
