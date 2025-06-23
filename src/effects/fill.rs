use crate::{effects::FillEffect, fixtures::*, keyframes::*, simulation::PlaybackInformation};
use bevy::prelude::*;

pub fn update_fill_effect(
    playback: Res<PlaybackInformation>,
    mut query: Query<(&mut FillEffect, &Keyframes)>,
) {
    for (mut effect, keyframes) in &mut query {
        effect.color = get_color_value(
            &keyframes.keyframes,
            "color",
            playback.current_time,
            &effect.color,
        );
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
