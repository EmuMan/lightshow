use bevy::prelude::*;
use crate::components::{effects::FillEffect, keyframes::Keyframes};
use crate::util::keyframes::*;
use crate::resources::simulation::SimulationTime;
use crate::components::fixtures::*;

pub fn update_fill_effect(
    simulation_time: Res<SimulationTime>,
    mut query: Query<(&mut FillEffect, &Keyframes)>,
) {
    for (mut effect, keyframes) in &mut query {
        effect.color = get_color_value(&keyframes.keyframes, "color", simulation_time.time, &effect.color);
    }
}

pub fn apply_fill_effect(
    mut color_lights_query: Query<&mut ColorLight>,
    fill_query: Query<&FillEffect>,
) {
    for fill_effect in &fill_query {
        for mut color_light in &mut color_lights_query {
            color_light.color_queue.push(fill_effect.color);
        }
    }
}
