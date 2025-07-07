use crate::{effects::*, fixtures::*, keyframes::*, timeline::CurrentTime};

#[derive(Component, Debug, Clone)]
pub struct ColorFillEffect {
    pub color: Color,
}

pub fn update_fill_effect(
    mut query: Query<(&CurrentTime, &mut ColorFillEffect, &Keyframes), With<Effect>>,
) {
    for (current_time, mut fill_effect, keyframes) in &mut query {
        fill_effect.color = get_color_value(
            &keyframes.keyframes,
            "color",
            current_time.time,
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
