use bevy::prelude::*;

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
