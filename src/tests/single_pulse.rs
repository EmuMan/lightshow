use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    fixtures::*,
    network::*,
    simple_store::*,
    timeline::{effects::*, keyframes::*, layers::*, tracks::*},
};

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut active_socket: ResMut<ActiveSocket>,
    mut primary_layer: ResMut<PrimaryLayer>,
    mut layer_store: ResMut<SimpleStore<Layer>>,
    mut effect_store: ResMut<SimpleStore<Effect>>,
) {
    active_socket.socket = Some(UdpSocket::bind(("0.0.0.0", 6454)).unwrap());

    for i in 0..150 {
        color_light::spawn_color_light(
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
        color_light::spawn_color_light(
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

    let effect_info = color::shockwave::ColorShockwaveEffect {
        color: Color::WHITE,
        center: Vec3::ZERO,
        radius: 0.,
        flat: 10.,
        head: 30.,
        tail: 30.,
    };

    let effect_handle = effect_store.add(Effect {
        groups: vec![0],
        info: EffectInfo::ColorShockwaveEffect(effect_info),
    });

    let track_info = TrackInfo {
        blending_mode: BlendingMode::ADD,
        opacity: 1.0,
    };

    let track_contents = TrackContents::EffectTrack {
        effect: effect_handle,
    };

    let track = Track {
        keyframes,
        info: track_info,
        contents: track_contents,
    };

    let layer_info = LayerInfo { strength: 1.0 };

    let layer = Layer {
        name: "Main Layer".into(),
        length: 4.,
        tracks: vec![track],
        info: layer_info,
    };

    let layer_handle = layer_store.add(layer);

    primary_layer.0 = Some(layer_handle);
}
