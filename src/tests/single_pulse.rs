use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    fixtures::*,
    network::*,
    simple_store::*,
    timeline::{effects::*, keyframes::*, sequences::*, tracks::*},
    util::blending::BlendingMode,
};

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut active_socket: ResMut<ActiveSocket>,
    mut primary_sequence: ResMut<PrimarySequence>,
    mut sequence_store: ResMut<SimpleStore<Sequence>>,
) {
    active_socket.socket = Some({
        let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();
        socket
            .set_broadcast(true)
            .expect("Failed to set broadcast mode on socket");
        socket
    });

    for i in 0..150 {
        color_light::spawn_color_light(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_translation(Vec3::new(-50., (i as f32 * 2.) as f32 - 150., 0.)),
            1.,
            ColorFixture::default(),
            vec![0],
            Some(
                ArtNetDataPointer::new(
                    ArtNetAddress::new(0, 0, 0).expect("ArtNetAddress should be valid"),
                    i * 3,
                )
                .expect("ArtNetDataPointer should be valid"),
            ),
        );
    }

    for i in 0..150 {
        color_light::spawn_color_light(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_translation(Vec3::new(50., (i as f32 * 2.) as f32 - 150., 0.)),
            1.,
            ColorFixture::default(),
            vec![0],
            Some(
                ArtNetDataPointer::new(
                    ArtNetAddress::new(0, 0, 1).expect("ArtNetAddress should be valid"),
                    i * 3,
                )
                .expect("ArtNetDataPointer should be valid"),
            ),
        );
    }

    let effect_keyframes = Keyframes::new(vec![
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
    ]);

    let effect_info = color::shockwave::ColorShockwaveEffect {
        color: Color::WHITE,
        center: Vec3::ZERO,
        radius: 0.,
        flat: 10.,
        head: 30.,
        tail: 30.,
    };

    let track_info = TrackInfo {
        blending_mode: BlendingMode::Add,
        factor: 1.0,
        track_keyframes: Keyframes::default(),
    };

    let track_contents = TrackContents::EffectTrack {
        effect_init_info: ColorEffectInfo::ColorShockwaveEffect(effect_info).into(),
        effect_keyframes,
    };

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
