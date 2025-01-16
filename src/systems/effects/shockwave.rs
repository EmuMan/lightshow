use bevy::prelude::*;
use crate::components::{effects::ShockwaveEffect, keyframes::Keyframes};
use crate::util::keyframes::*;
use crate::resources::simulation::SimulationTime;
use crate::components::fixtures::*;

pub fn update_shockwave_effect(
    simulation_time: Res<SimulationTime>,
    mut query: Query<(&mut ShockwaveEffect, &Keyframes)>,
) {
    for (mut effect, keyframes) in &mut query {
        effect.color = get_color_value(&keyframes.keyframes, "color", simulation_time.time, &effect.color);
        effect.center = get_vec3_value(&keyframes.keyframes, "center", simulation_time.time, &effect.center);
        effect.radius = get_float_value(&keyframes.keyframes, "radius", simulation_time.time, &effect.radius);
        effect.flat = get_float_value(&keyframes.keyframes, "flat", simulation_time.time, &effect.flat);
        effect.head = get_float_value(&keyframes.keyframes, "head", simulation_time.time, &effect.head);
        effect.tail = get_float_value(&keyframes.keyframes, "tail", simulation_time.time, &effect.tail);
    }
}


pub fn apply_shockwave_effect(
    mut color_lights_query: Query<(&mut ColorLight, &Transform)>,
    shockwave_query: Query<&ShockwaveEffect>,
) {
    for shockwave_effect in &shockwave_query {
        for (mut color_light, transform) in &mut color_lights_query {
            let distance = transform.translation.distance(shockwave_effect.center);
            let new_color = get_shockwave_color_value(
                &shockwave_effect.color,
                distance,
                shockwave_effect.radius,
                shockwave_effect.flat,
                shockwave_effect.head,
                shockwave_effect.tail,
            );
            color_light.color_queue.push(new_color);
        }
    }
}

fn get_shockwave_color_value(
    color: &Color,
    distance: f32,
    radius: f32,
    flat: f32,
    head: f32,
    tail: f32,
) -> Color {
    let half_flat = flat / 2.;
    let mut influence: f32 = 0.;

    if distance > radius - half_flat && distance < radius + half_flat {
        influence = 1.;
    } else if distance > radius + half_flat && distance < radius + half_flat + head {
        influence = ((radius + half_flat + head) - distance) / head;
    } else if distance > radius - half_flat - tail && distance < radius - half_flat {
        influence = (distance - (radius - half_flat - tail)) / tail;
    }

    color.mix(&Color::BLACK, 1.0 - influence)
}
