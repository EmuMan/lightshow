use bevy::prelude::*;

#[derive(Component)]
pub struct Light {
    pub groups: Vec<u32>,
    pub location: Vec3,
    pub radius: f32,
}
