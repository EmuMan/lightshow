use bevy::prelude::*;

use crate::resources::simulation::*;

pub fn increment_playback_time(
    time: Res<Time>,
    mut playback: ResMut<PlaybackInformation>,
) {
    if playback.is_playing {
        playback.current_time += time.delta_secs_f64();
        if playback.current_time > playback.length {
            playback.current_time = 0.0;
        }
    }
}
