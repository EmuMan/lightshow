use std::net::UdpSocket;

use bevy::prelude::*;
use color_light::spawn_color_light;

use crate::components::effects::{Effect, ShockwaveEffect};
use crate::components::network::ArtNetNode;
use crate::components::layers::Layer;
use crate::components::keyframes::*;
use crate::resources::network::ActiveSocket;
use crate::resources::simulation::PlaybackInformation;
use crate::systems::fixtures::*;

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut playback: ResMut<PlaybackInformation>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut active_socket: ResMut<ActiveSocket>,
) {
    active_socket.socket = Some(UdpSocket::bind(("0.0.0.0", 6454)).unwrap());

    for i in 0..150 {
        spawn_color_light(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_translation(Vec3::new(-50., (i as f32 * 2.) as f32 - 150., 0.)),
            1.,
            vec![0],
            Some(ArtNetNode {
                ip: "192.168.1.156".into(),
                channels: vec![1 + i * 3, 0 + i * 3, 2 + i * 3],
                ..Default::default()
            }),
        );
    }

    for i in 0..150 {
        spawn_color_light(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_translation(Vec3::new(50., (i as f32 * 2.) as f32 - 150., 0.)),
            1.,
            vec![0],
            Some(ArtNetNode {
                ip: "192.168.1.157".into(),
                channels: vec![1 + i * 3, 0 + i * 3, 2 + i * 3],
                ..Default::default()
            }),
        );
    }

    let mut layer = Layer {
        length: 4.,
        effects: vec![],
    };

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
            value: KeyframeValue::FloatKeyframe(300.),
        },
    ];

    let effect = ShockwaveEffect {
        color: Color::WHITE,
        center: Vec3::ZERO,
        radius: 0.,
        flat: 10.,
        head: 30.,
        tail: 30.,
    };
    
    let effect_id = commands.spawn((
        Effect {
            groups: vec![0],
            start: 0.,
            end: 3.,
        },
        effect,
        Keyframes { keyframes },
    )).id();

    layer.effects.push(effect_id);

    let layer_id = commands.spawn((layer,)).id();

    playback.current_layer = Some(layer_id);
}
