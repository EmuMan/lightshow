use std::collections::VecDeque;

use crate::{
    timeline::{effects::*, keyframes::*},
    util::blending::{colors::interpolate_color_bands, sample_windowed},
};

#[derive(Component, Debug, Clone)]
pub struct ColorFrequencyCascadeEffect {
    pub past_values: VecDeque<(f32, f32)>, // freq, strength
    pub color_bands: Vec<(f32, Color)>,
    pub scaled_direction: Vec3,
    pub buffer_size: usize,
    pub window_size: f32,
}

impl ColorFrequencyCascadeEffect {
    pub fn new(
        color_bands: Vec<(f32, Color)>,
        scaled_direction: Vec3,
        buffer_size: usize,
        window_size: f32,
    ) -> Self {
        Self {
            past_values: VecDeque::from(vec![(0.0, 0.0); buffer_size]),
            color_bands,
            scaled_direction,
            buffer_size,
            window_size,
        }
    }
}

impl ColorEffectLike for ColorFrequencyCascadeEffect {
    fn get_value(&self, position: Vec3) -> Color {
        let direction = self.scaled_direction.normalize();
        let projection = position.dot(direction);
        let scaled = projection / self.scaled_direction.length();

        let triangle_wave = 1.0 - ((scaled.abs() % 2.0) - 1.0).abs();

        let (freqs, strengths): (Vec<f32>, Vec<f32>) = self.past_values.iter().cloned().unzip();

        let strength = sample_windowed(&strengths, triangle_wave, self.window_size);
        let freq = sample_windowed(&freqs, triangle_wave, self.window_size);

        let mut color = interpolate_color_bands(&self.color_bands, freq);
        color.set_alpha(color.alpha() * strength);
        println!("alpha: {}", color.alpha());
        color
    }

    fn update(
        &mut self,
        _keyframes: &Keyframes,
        _current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    ) {
        let mut frame_count = 0;
        let mut lows_volume = 0.0f32;
        let mut mids_volume = 0.0f32;
        let mut highs_volume = 0.0f32;

        for frame in common_info.recent_fft_data.get_new_from_fixed_update() {
            frame_count += 1;
            lows_volume += frame.bass();
            mids_volume += (frame.low_mid() + frame.mid()) / 2.0;
            highs_volume += (frame.high_mid() + frame.treble()) / 2.0;
        }

        lows_volume /= frame_count as f32;
        mids_volume /= frame_count as f32;
        highs_volume /= frame_count as f32;

        let overall_volume = (lows_volume + mids_volume + highs_volume) / 3.0;
        let intensity = (overall_volume / 15.0).clamp(0.0, 1.0);

        // with compensation for usual volumes
        let lows_comp = lows_volume * 0.4;
        let mids_comp = mids_volume;
        let highs_comp = highs_volume * 3.0;

        let total_weight = lows_comp + mids_comp + highs_comp;
        let average = if total_weight == 0.0 {
            0.5
        } else {
            0.5 * (mids_comp / total_weight) + 1.0 * (highs_comp / total_weight)
        };

        println!("average_freq: {}", average);

        self.past_values.pop_front();
        self.past_values.push_back((average, intensity));
    }

    fn insert_component(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(self.clone());
    }
}
