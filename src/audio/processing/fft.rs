use bevy::prelude::*;

use crate::audio::HeapConsumer;
use realfft::{RealFftPlanner, RealToComplex};
use ringbuf::traits::{Consumer, Observer};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

/*
 * This file was written largely with the help of ClaudeCode.
 *
 * I oversaw the process and made a few adjustments, but I
 * don't want to take credit for the initial implementation,
 * as I am not terribly familiar with FFT mechanics and
 * implementations.
 */

#[derive(Clone, Debug)]
pub struct FftConfig {
    pub sample_rate: u32,
    pub window_size: usize,
    pub hop_size: usize,
}

impl Default for FftConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            window_size: 512,
            hop_size: 256,
        }
    }
}

impl FftConfig {
    /// Creates a new FFT configuration
    pub fn new(sample_rate: u32, window_size: usize, hop_size: usize) -> Self {
        Self {
            sample_rate,
            window_size,
            hop_size,
        }
    }

    /// Calculate the frequency resolution (Hz per bin)
    pub fn frequency_resolution(&self) -> f32 {
        self.sample_rate as f32 / self.window_size as f32
    }

    /// Calculate the latency per FFT frame in milliseconds
    pub fn frame_latency_ms(&self) -> f32 {
        (self.window_size as f32 / self.sample_rate as f32) * 1000.0
    }

    /// Calculate the update rate (time between frames) in milliseconds
    pub fn update_rate_ms(&self) -> f32 {
        (self.hop_size as f32 / self.sample_rate as f32) * 1000.0
    }

    /// Get the number of frequency bins (FFT output size)
    pub fn num_bins(&self) -> usize {
        self.window_size / 2 + 1
    }

    /// Convert a bin index to its center frequency in Hz
    pub fn bin_to_frequency(&self, bin: usize) -> f32 {
        (bin as f32 * self.sample_rate as f32) / self.window_size as f32
    }

    /// Find the bin index closest to a given frequency in Hz
    pub fn frequency_to_bin(&self, frequency: f32) -> usize {
        ((frequency * self.window_size as f32) / self.sample_rate as f32).round() as usize
    }
}

/// Holds data about past FFT frames for usage by Bevy systems
#[derive(Resource, Debug, Default)]
pub struct RecentFftData {
    past_second: VecDeque<SpectrumData>,
    new_from_fixed_update_count: usize,
}

impl RecentFftData {
    /// Creates a new recent FFT data store
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a set of new frames to the store
    /// Also updates the new count and removes old frames
    pub fn add(&mut self, mut new_data: VecDeque<SpectrumData>) {
        if new_data.is_empty() {
            return;
        }
        let assumed_config = &new_data.get(0).unwrap().config.clone();

        self.new_from_fixed_update_count = new_data.len();
        self.past_second.append(&mut new_data);
        let samples_per_bin = assumed_config.window_size - assumed_config.hop_size;
        let number_to_keep = assumed_config.sample_rate as usize / samples_per_bin + 1;

        while self.past_second.len() > number_to_keep {
            self.past_second.pop_front();
        }

        self.new_from_fixed_update_count =
            self.new_from_fixed_update_count.min(self.past_second.len());
    }

    /// Retrieves an iterator over frames that cover the past second of data
    pub fn past_second(&self) -> impl Iterator<Item = &SpectrumData> {
        self.past_second.iter()
    }

    /// Retrieves an iterator over frames added in the last FixedUpdate cycle
    pub fn get_new_from_fixed_update(&self) -> impl Iterator<Item = &SpectrumData> {
        if self.new_from_fixed_update_count == 0 {
            return self.past_second.range(0..0);
        }
        let total = self.past_second.len();
        let from = total - self.new_from_fixed_update_count;
        self.past_second.range(from..total)
    }
}

/// Spectrum data from a single FFT frame
#[derive(Debug, Clone)]
pub struct SpectrumData {
    /// Magnitude values for each frequency bin
    pub magnitudes: Vec<f32>,
    /// Configuration used to generate this spectrum
    pub config: FftConfig,
}

impl SpectrumData {
    /// Get the magnitude at a specific frequency (in Hz)
    pub fn magnitude_at_frequency(&self, freq: f32) -> f32 {
        let bin = self.config.frequency_to_bin(freq);
        self.magnitudes.get(bin).copied().unwrap_or(0.0)
    }

    /// Get the average magnitude in a frequency range (in Hz)
    pub fn average_magnitude_range(&self, freq_min: f32, freq_max: f32) -> f32 {
        let bin_min = self.config.frequency_to_bin(freq_min);
        let bin_max = self
            .config
            .frequency_to_bin(freq_max)
            .min(self.magnitudes.len() - 1);

        if bin_min > bin_max {
            return 0.0;
        }

        let sum: f32 = self.magnitudes[bin_min..=bin_max].iter().sum();
        sum / (bin_max - bin_min + 1) as f32
    }

    /// Get the peak frequency (ignoring DC component)
    pub fn peak_frequency(&self) -> (f32, f32) {
        let (peak_idx, &peak_mag) = self
            .magnitudes
            .iter()
            .enumerate()
            .skip(1)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap_or((0, &0.0));

        (self.config.bin_to_frequency(peak_idx), peak_mag)
    }

    /// Common frequency band helpers for music visualization
    pub fn bass(&self) -> f32 {
        self.average_magnitude_range(20.0, 150.0)
    }

    pub fn low_mid(&self) -> f32 {
        self.average_magnitude_range(150.0, 500.0)
    }

    pub fn mid(&self) -> f32 {
        self.average_magnitude_range(500.0, 2000.0)
    }

    pub fn high_mid(&self) -> f32 {
        self.average_magnitude_range(2000.0, 4000.0)
    }

    pub fn treble(&self) -> f32 {
        self.average_magnitude_range(4000.0, 20000.0)
    }
}

/// Real-time FFT processor
#[derive(Resource)]
pub struct FftProcessor {
    config: FftConfig,
    fft: Arc<dyn RealToComplex<f32> + Send + Sync>,
    input_buffer: Vec<f32>,
    spectrum_buffer: Vec<num_complex::Complex<f32>>,
    window: Vec<f32>,
    consumer: Mutex<HeapConsumer<f32>>,
}

impl FftProcessor {
    /// Create a new FFT processor with a ring buffer consumer
    pub fn new(config: FftConfig, consumer: HeapConsumer<f32>) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(config.window_size);
        let spectrum_buffer = fft.make_output_vec();
        let input_buffer = vec![0.0f32; config.window_size];

        // Create Hanning window
        let window = apodize::hanning_iter(config.window_size)
            .map(|x| x as f32)
            .collect::<Vec<f32>>();

        Self {
            config,
            fft,
            input_buffer,
            spectrum_buffer,
            window,
            consumer: Mutex::new(consumer),
        }
    }

    /// Process the next FFT frame if enough samples are available
    /// Returns Some(SpectrumData) if a frame was processed, None otherwise
    /// This will likely need to be reviewed for accuracy later. For now, it is good enough.
    pub fn process_next_frame(&mut self) -> Option<SpectrumData> {
        let mut consumer = self
            .consumer
            .lock()
            .expect("could not unlock audio consumer");

        // Check if we have enough samples
        if consumer.occupied_len() < self.config.window_size {
            return None;
        }

        // Read samples from ring buffer
        for i in 0..self.config.window_size {
            self.input_buffer[i] = consumer.try_pop().unwrap_or(0.0);
        }

        // Apply window function
        for i in 0..self.config.window_size {
            self.input_buffer[i] *= self.window[i];
        }

        // Perform FFT
        self.fft
            .process(&mut self.input_buffer, &mut self.spectrum_buffer)
            .expect("FFT processing failed");

        // Calculate magnitude spectrum
        let magnitudes: Vec<f32> = self
            .spectrum_buffer
            .iter()
            .map(|c| (c.re * c.re + c.im * c.im).sqrt())
            .collect();

        // Skip forward by hop size (for overlap)
        for _ in 0..(self.config.window_size - self.config.hop_size) {
            let _ = consumer.try_pop();
        }

        Some(SpectrumData {
            magnitudes,
            config: self.config.clone(),
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &FftConfig {
        &self.config
    }

    /// Check if enough samples are available for processing
    pub fn can_process(&self) -> bool {
        let consumer = self
            .consumer
            .lock()
            .expect("could not unlock audio consumer");
        consumer.occupied_len() >= self.config.window_size
    }

    /// Get the number of samples currently buffered
    pub fn buffered_samples(&self) -> usize {
        let consumer = self
            .consumer
            .lock()
            .expect("could not unlock audio consumer");
        consumer.occupied_len()
    }
}

/// Bevy system to FFT process recent audio samples
pub fn fft_process_recent_samples(
    fft_processor: Option<ResMut<FftProcessor>>,
    mut recent_fft_data: ResMut<RecentFftData>,
) {
    let Some(mut fft_processor) = fft_processor else {
        // The FFT processor has not been initialized. That's fine,
        // it just means we can't go any further.
        return;
    };

    if !fft_processor.can_process() {
        // There somehow haven't been enough samples collected to fill a window.
        return;
    }

    let mut recent_frames: VecDeque<SpectrumData> = VecDeque::new();

    loop {
        let Some(frame) = fft_processor.process_next_frame() else {
            break;
        };
        recent_frames.push_back(frame);
    }

    recent_fft_data.add(recent_frames);
}
