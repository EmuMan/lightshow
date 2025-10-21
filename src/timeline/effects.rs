use bevy::prelude::*;

use crate::{simple_store::SimpleStore, timeline::keyframes::Keyframes};

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
}

#[derive(Debug, Clone)]
pub enum EffectInfo {
    ColorFillEffect(color::fill::ColorFillEffect),
    ColorShockwaveEffect(color::shockwave::ColorShockwaveEffect),
}

// TODO: Using this will make life MUCH easier.
pub trait EffectTrait<T>: Send + Sync + std::fmt::Debug {
    fn get_value(&self, position: Vec3) -> T;
    fn update(&mut self, keyframes: &Keyframes, current_time: f64);
    fn insert_component(&self, entity_commands: &mut EntityCommands);
}
