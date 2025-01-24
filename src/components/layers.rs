use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Layer {
    pub effects: Vec<Entity>,
    pub length: f64,
}
