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
    util::blending::BlendingMode,
};

pub fn frequency_cascade_test_startup(
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

    let track_info = TrackInfo {
        blending_mode: BlendingMode::Add,
        factor: 1.0,
        track_keyframes: Keyframes::default(),
    };

    let track_contents = TrackContents::EffectTrack {
        effect_init_info: ColorEffectInfo::ColorFrequencyCascadeEffect(effect_info).into(),
        effect_keyframes: Keyframes::default(),
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
