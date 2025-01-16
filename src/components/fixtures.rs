use bevy::prelude::*;

#[derive(Component)]
pub struct Fixture {
    pub groups: Vec<u32>,
}

#[derive(Component)]
pub struct ColorLight {
    pub radius: f32,
    pub color_queue: Vec<Color>,
}
