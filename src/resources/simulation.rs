use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlaybackInformation {
    pub current_time: f64,
    pub is_playing: bool,
    pub length: f64,
    pub bpm: f64,
    pub beats_per_bar: usize,
}

impl Default for PlaybackInformation {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            is_playing: false,
            length: 5.0,
            bpm: 120.0,
            beats_per_bar: 4,
        }
    }
}
