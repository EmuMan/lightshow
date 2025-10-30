use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    audio::{
        capture::AudioCapture,
        processing::fft::{FftConfig, FftProcessor},
    },
    fixtures::*,
    network::*,
    simple_store::*,
    timeline::{effects::*, keyframes::*, sequences::*, tracks::*},
    util::blending::colors::ColorBlendingMode,
};

pub fn frequency_cascade_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut active_socket: ResMut<ActiveSocket>,
    mut primary_sequence: ResMut<PrimarySequence>,
    mut sequence_store: ResMut<SimpleStore<Sequence>>,
    mut effect_store: ResMut<SimpleStore<Effect>>,
) {
    active_socket.socket = Some(UdpSocket::bind(("0.0.0.0", 6454)).unwrap());

    let fft_config = FftConfig {
        sample_rate: 44100,
        window_size: 512,
        hop_size: 256,
    };

    let (audio_capture, heap_consumer) = AudioCapture::default().unwrap();

    let fft_processor = FftProcessor::new(fft_config, heap_consumer);

    commands.insert_resource(audio_capture);
    commands.insert_resource(fft_processor);

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

    let effect_info = color::frequency_cascade_effect::ColorFrequencyCascadeEffect::new(
        vec![
            (0.2, Color::linear_rgb(1.0, 0.2, 0.0)),
            (0.4, Color::linear_rgb(0.0, 1.0, 0.0)),
            (0.7, Color::linear_rgb(1.0, 0.0, 1.0)),
        ],
        Vec3::new(0.0, 40.0, 0.0),
        16,
        0.05,
    );

    let effect_handle = effect_store.add(Effect {
        groups: vec![0],
        info: ColorEffectInfo::ColorFrequencyCascadeEffect(effect_info).into(),
        keyframes: Keyframes::default(),
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
