use bevy::prelude::*;
use color_light::spawn_color_light;

use crate::components::effects::{Effect, ShockwaveEffect};
use crate::systems::fixtures::*;
use crate::components::keyframes::*;

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..10 {
        for j in 0..10 {
            spawn_color_light(
                &mut commands,
                &mut meshes,
                &mut materials,
                Transform::from_translation(Vec3::new((i as f32 * 30.) - 135., (j as f32 * 30.) as f32 - 135., 0.)),
                10.,
                vec![0],
            );
        }
    }

    let keyframes = vec![
        Keyframe {
            time: 0.,
            interpolation: InterpolationType::LINEAR,
            key: "radius".to_string(),
            value: KeyframeValue::FloatKeyframe(0.),
        },
        Keyframe {
            time: 3.,
            interpolation: InterpolationType::LINEAR,
            key: "radius".to_string(),
            value: KeyframeValue::FloatKeyframe(250.),
        },
    ];

    let effect = ShockwaveEffect {
        color: Color::WHITE,
        center: Vec3::ZERO,
        radius: 0.,
        flat: 30.,
        head: 30.,
        tail: 30.,
    };
    
    commands.spawn((
        Effect {
            groups: vec![0],
            start: 0.,
            end: 3.,
        },
        effect,
        Keyframes { keyframes },
    ));
}
