use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    fixtures::*,
    network::*,
    simple_store::*,
    timeline::{effects::*, keyframes::*, sequences::*, tracks::*},
    util::blending::colors::ColorBlendingMode,
};

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut active_socket: ResMut<ActiveSocket>,
    mut primary_sequence: ResMut<PrimarySequence>,
    mut sequence_store: ResMut<SimpleStore<Sequence>>,
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

    let keyframes = Keyframes {
        keyframes: vec![
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
        ],
    };

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
        info: ColorEffectInfo::ColorShockwaveEffect(effect_info).into(),
        keyframes,
    });

    let track_info = TrackInfo {
        color_blending_mode: ColorBlendingMode::Add,
        opacity: 1.0,
    };

    let track_contents = TrackContents::EffectTrack { effect_handle };

    let track = Track {
        info: track_info,
        contents: track_contents,
    };

    let sequence = Sequence {
        name: "Main Sequence".into(),
        length: 4.,
        tracks: vec![track],
    };

    let sequence_handle = sequence_store.add(sequence);

    primary_sequence.0 = Some(sequence_handle);
}
