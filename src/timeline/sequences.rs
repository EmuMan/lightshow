use bevy::prelude::*;

use crate::{
    simple_store::{SimpleHandle, SimpleStore},
    timeline::tracks::Track,
};

/// Bevy plugin for sequences.
pub struct SequencesPlugin;

impl Plugin for SequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Sequence>>()
            .init_resource::<PrimarySequence>();
    }
}

/// Bevy resource that points to the sequence the user currently has open. If
/// there is no such sequence, contains `None`.
#[derive(Resource, Default, Debug)]
pub struct PrimarySequence(pub Option<SimpleHandle<Sequence>>);

/// Primary representation of visuals. Along with metadata, contains a list of
/// `Track`s that allow for effects to be added and for sequences to be nested
/// within each other.
#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub length: f64,
    pub tracks: Vec<Track>,
}
