use bevy::prelude::*;

pub mod capture;
pub mod processing;

pub struct AudioPlugin;

type HeapConsumer<T> = ringbuf::HeapCons<T>;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(processing::AudioProcessingPlugin);
    }
}
