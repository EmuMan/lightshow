use crate::{effects::*, fixtures::*, keyframes::*};

#[derive(Component, Debug, Clone)]
pub struct ColorFillEffect {
    pub color: Color,
}

pub fn update_fill_effect(mut query: Query<(&Effect, &mut ColorFillEffect, &Keyframes)>) {
    for (effect, mut fill_effect, keyframes) in &mut query {
        fill_effect.color = get_color_value(
            &keyframes.keyframes,
            "color",
            effect.current_time,
            &fill_effect.color,
        );
    }
}

pub fn apply_fill_effect(
    mut color_lights_query: Query<&mut ColorLight>,
    fill_query: Query<&ColorFillEffect>,
) {
    for fill_effect in &fill_query {
        for mut color_light in &mut color_lights_query {
            color_light.color_queue.push(fill_effect.color);
        }
    }
}
