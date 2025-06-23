use bevy::prelude::*;

pub mod color_light;

pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, color_light::apply_color_light_color_queues)
            .add_systems(FixedUpdate, color_light::add_data_to_buffer);
    }
}

#[derive(Component)]
pub struct Fixture {
    pub groups: Vec<u32>,
}

#[derive(Component)]
pub struct ColorLight {
    pub radius: f32,
    pub color_queue: Vec<Color>,
}
