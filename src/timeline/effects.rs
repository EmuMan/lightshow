use bevy::prelude::*;

use crate::{simple_store::SimpleStore, timeline::keyframes::Keyframes};
use derive_more::From;
use enum_dispatch::enum_dispatch;

pub mod color;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Effect>>();
    }
}

#[derive(Debug)]
pub struct Effect {
    pub groups: Vec<u32>,
    pub info: EffectInfo,
    pub keyframes: Keyframes,
}

#[derive(Debug, Clone, From)]
pub enum EffectInfo {
    ColorEffectInfo(ColorEffectInfo),
    Vec3EffectInfo(Vec3EffectInfo),
}

impl EffectInfo {
    pub fn update(&mut self, keyframes: &Keyframes, current_time: f64) {
        match self {
            EffectInfo::ColorEffectInfo(color_effect_info) => {
                color_effect_info.update(keyframes, current_time)
            }
            EffectInfo::Vec3EffectInfo(vec3_effect_info) => {
                // TODO: re-implement once enum_dispatch is back for vec3
                // vec3_effect_info.update(keyframes, current_time)
            }
        }
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch(ColorEffectLike)]
pub enum ColorEffectInfo {
    ColorFillEffect(color::fill::ColorFillEffect),
    ColorShockwaveEffect(color::shockwave::ColorShockwaveEffect),
}

#[derive(Debug, Clone)]
// #[enum_dispatch(Vec3EffectLike)]
pub enum Vec3EffectInfo {}

#[enum_dispatch]
pub trait ColorEffectLike: Send + Sync + std::fmt::Debug {
    fn get_value(&self, position: Vec3) -> Color;
    fn update(&mut self, keyframes: &Keyframes, current_time: f64);
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}

// TODO: add enum_dispatch back once this actually does something
// #[enum_dispatch]
pub trait Vec3EffectLike: Send + Sync + std::fmt::Debug {
    fn get_value(&self, position: Vec3) -> Vec3;
    fn update(&mut self, keyframes: &Keyframes, current_time: f64);
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}
