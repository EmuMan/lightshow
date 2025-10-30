use bevy::prelude::*;

pub mod fft;

pub struct AudioProcessingPlugin;

impl Plugin for AudioProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<fft::RecentFftData>()
            .add_systems(FixedUpdate, fft::fft_process_recent_samples);
    }
}
