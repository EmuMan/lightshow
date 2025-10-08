use bevy::prelude::*;

pub mod effects;
pub mod keyframes;
pub mod layer_tree;
pub mod layers;
pub mod playback;
pub mod tracks;

#[derive(Component, Default, Debug)]
pub struct CurrentTime {
    pub time: f64,
}
