use bevy::prelude::*;

pub mod effects;
pub mod keyframes;
pub mod playback;
pub mod sequence_tree;
pub mod sequences;
pub mod tracks;

pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(effects::EffectsPlugin)
            .add_plugins(playback::PlaybackPlugin)
            .add_plugins(sequences::SequencesPlugin)
            .add_plugins(sequence_tree::SequenceTreePlugin);
    }
}
