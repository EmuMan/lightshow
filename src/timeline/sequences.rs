use bevy::prelude::*;

use crate::{
    simple_store::{SimpleHandle, SimpleStore},
    timeline::tracks::Track,
};

pub struct SequencesPlugin;

impl Plugin for SequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Sequence>>()
            .init_resource::<PrimarySequence>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct PrimarySequence(pub Option<SimpleHandle<Sequence>>);

#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub length: f64,
    pub tracks: Vec<Track>,
    pub info: SequenceInfo,
}

#[derive(Debug, Copy, Clone)]
pub struct SequenceInfo {
    pub strength: f64,
}
