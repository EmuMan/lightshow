use bevy::prelude::*;

#[derive(Component)]
pub struct RgbLight {
    pub groups: Vec<u32>,
    pub radius: f32,
}
