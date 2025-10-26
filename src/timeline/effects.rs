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

#[derive(Debug, Clone)]
#[enum_dispatch(EffectLike)]
pub enum EffectInfo {
    ColorFillEffect(color::fill::ColorFillEffect),
    ColorShockwaveEffect(color::shockwave::ColorShockwaveEffect),
}

#[derive(Debug, Clone, From)]
pub enum EffectOutputValue {
    Color(Color),
    Vec3(Vec3),
}

#[enum_dispatch]
pub trait EffectLike: Send + Sync + std::fmt::Debug {
    fn get_value(&self, position: Vec3) -> EffectOutputValue;
    fn update(&mut self, keyframes: &Keyframes, current_time: f64);
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}
