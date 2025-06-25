use crate::{effects::*, fixtures::*, keyframes::*};

#[derive(Component, Debug, Clone)]
pub struct ColorShockwaveEffect {
    pub color: Color,
    pub center: Vec3,
    pub radius: f32,
    pub flat: f32,
    pub head: f32,
    pub tail: f32,
}

pub fn update_shockwave_effect(mut query: Query<(&Effect, &mut ColorShockwaveEffect, &Keyframes)>) {
    for (effect, mut shockwave_effect, keyframes) in &mut query {
        shockwave_effect.color = get_color_value(
            &keyframes.keyframes,
            "color",
            effect.current_time,
            &shockwave_effect.color,
        );
        shockwave_effect.center = get_vec3_value(
            &keyframes.keyframes,
            "center",
            effect.current_time,
            &shockwave_effect.center,
        );
        shockwave_effect.radius = get_float_value(
            &keyframes.keyframes,
            "radius",
            effect.current_time,
            &shockwave_effect.radius,
        );
        shockwave_effect.flat = get_float_value(
            &keyframes.keyframes,
            "flat",
            effect.current_time,
            &shockwave_effect.flat,
        );
        shockwave_effect.head = get_float_value(
            &keyframes.keyframes,
            "head",
            effect.current_time,
            &shockwave_effect.head,
        );
        shockwave_effect.tail = get_float_value(
            &keyframes.keyframes,
            "tail",
            effect.current_time,
            &shockwave_effect.tail,
        );
    }
}

pub fn apply_shockwave_effect(
    mut color_lights_query: Query<(&mut ColorLight, &Transform)>,
    shockwave_query: Query<&ColorShockwaveEffect>,
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
