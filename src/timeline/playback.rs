use bevy::prelude::*;

use crate::{simple_store::SimpleStore, tests, timeline::sequences::*};

/// Bevy plugin for playback.
pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlaybackInformation>()
            .add_systems(Update, increment_playback_time)
            .add_systems(Startup, tests::single_pulse::pulse_test_startup);
    }
}

/// Bevy resource that holds information about the current state of playback on
/// the primary sequence, including current playback head time, whether or not
/// playback is currently in progress, and the beats per minute and beats per bar
/// to define gridlines.
///
/// TODO: Separate BPM into some separate measure. Maybe make sequences either
/// time-bound or BPM bound? Really not sure how to best do this.
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

/// Increments current playback time when playback is currently in progress.
/// Loops back to `0` when playback reaches the end of the primary sequence.
pub fn increment_playback_time(
    time: Res<Time>,
    mut playback: ResMut<PlaybackInformation>,
    primary_sequence: Res<PrimarySequence>,
    sequence_store: Res<SimpleStore<Sequence>>,
) {
    let Some(sequence) = primary_sequence
        .0
        .and_then(|handle| sequence_store.get(handle))
    else {
        playback.current_time = 0.0;
        playback.is_playing = false;
        return;
    };
    if playback.is_playing {
        playback.current_time += time.delta_secs_f64();
        if playback.current_time > sequence.length {
            playback.current_time = 0.0;
        }
    }
}
