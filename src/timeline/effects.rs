use bevy::prelude::*;

use crate::{
    audio::processing::fft::RecentFftData, fixtures::PanTilt, timeline::keyframes::Keyframes,
};
use derive_more::From;
use enum_dispatch::enum_dispatch;

pub mod color;
pub mod pan_tilt;

/// Global information shared between all effects. Includes playback time, FFT
/// data, and more in the future. Constructed with Bevy resources before
/// sequence tree traversal and passed to all effect update functions.
#[derive(Debug)]
pub struct EffectUpdateCommonInfo<'a> {
    pub recent_fft_data: &'a RecentFftData,
    pub global_time: f64,
}

/// Contains the information used for any particular effect. Wrapper around
/// both `ColorEffectInfo` and `PanTiltEffectInfo`.
#[derive(Debug, Clone, From)]
pub enum EffectInfo {
    ColorEffectInfo(ColorEffectInfo),
    PanTiltEffectInfo(PanTiltEffectInfo),
}

impl EffectInfo {
    /// Calls into the effect info to update it in accordance to the current
    /// time (within the effect's direct sequence, i.e. not global) and any
    /// global information specified as common info. This is also where
    /// keyframes are applied to effects; each individual implementation is
    /// responsible for providing its own update mechanisms.
    pub fn update(
        &mut self,
        keyframes: &Keyframes,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    ) {
        match self {
            EffectInfo::ColorEffectInfo(color_effect_info) => {
                color_effect_info.update(keyframes, current_time, common_info)
            }
            EffectInfo::PanTiltEffectInfo(pan_tilt_effect_info) => {
                pan_tilt_effect_info.update(keyframes, current_time, common_info)
            }
        }
    }
}

/// Contains all color effect implementations in an enum that requires all
/// variants to implement `ColorEffectLike`.
#[derive(Debug, Clone)]
#[enum_dispatch(ColorEffectLike)]
pub enum ColorEffectInfo {
    ColorFillEffect(color::fill::ColorFillEffect),
    ColorShockwaveEffect(color::shockwave::ColorShockwaveEffect),
    ColorFrequencyCascadeEffect(color::frequency_cascade_effect::ColorFrequencyCascadeEffect),
}

/// Contains all pan/tilt effect implementations in an enum that requires all
/// variants to implement `PanTiltEffectLike`.
#[derive(Debug, Clone)]
#[enum_dispatch(PanTiltEffectLike)]
pub enum PanTiltEffectInfo {
    PanTiltAllEffect(pan_tilt::all::PanTiltAllEffect),
}

/// Common methods shared by all color effect implementations.
#[enum_dispatch]
pub trait ColorEffectLike: Send + Sync + std::fmt::Debug {
    /// Gets the color value of the effect at the specified position.
    fn get_value(&self, position: Vec3) -> Color;

    /// Calls into the color effect to update it in accordance to the current
    /// time (within the effect's direct sequence, i.e. not global) and any
    /// global information specified as common info. This is also where
    /// keyframes are applied to effects; each individual implementation is
    /// responsible for providing its own update mechanisms.
    fn update(
        &mut self,
        keyframes: &Keyframes,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    );

    /// Inserts the effect info as a component within the world. This is to be
    /// used for debug/informational graphics within the preview window.
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}

/// Common methods shared by all pan/tilt effect implementations.
#[enum_dispatch]
pub trait PanTiltEffectLike: Send + Sync + std::fmt::Debug {
    /// Gets the pan/tilt value of the effect at the specified position.
    fn get_value(&self, position: Vec3) -> PanTilt;

    /// Calls into the pan/tilt effect to update it in accordance to the current
    /// time (within the effect's direct sequence, i.e. not global) and any
    /// global information specified as common info. This is also where
    /// keyframes are applied to effects; each individual implementation is
    /// responsible for providing its own update mechanisms.
    fn update(
        &mut self,
        keyframes: &Keyframes,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    );

    /// Inserts the effect info as a component within the world. This is to be
    /// used for debug/informational graphics within the preview window.
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}
