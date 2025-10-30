use crate::audio::HeapConsumer;
use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, traits::*};

/*
 * This file was written largely with the help of ClaudeCode.
 *
 * I oversaw the process and made a few adjustments, but I
 * don't want to take credit for the initial implementation,
 * as I am not terribly familiar with programming raw audio
 * device interactions.
 */

/// Audio capture instantiation that feeds the FFT processor
#[derive(Resource)]
pub struct AudioCapture {
    _stream: cpal::Stream,
}

impl AudioCapture {
    /// Create a new audio capture system
    pub fn new(
        sample_rate: u32,
        buffer_size: usize,
    ) -> Result<(Self, HeapConsumer<f32>), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        // Create ring buffer
        let ring = HeapRb::<f32>::new(buffer_size);
        let (mut producer, consumer) = ring.split();

        // Build input stream
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    let _ = producer.try_push(sample);
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?;

        stream.play()?;

        Ok((Self { _stream: stream }, consumer))
    }

    /// Create an audio capture system with default settings
    pub fn default() -> Result<(Self, HeapConsumer<f32>), Box<dyn std::error::Error>> {
        Self::new(44100, 44100 * 10) // 10 seconds of buffer
    }
}
