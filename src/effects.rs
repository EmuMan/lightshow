use bevy::prelude::*;

use crate::{
    effects::{fill::ColorFillEffect, shockwave::ColorShockwaveEffect},
    simple_store::SimpleStore,
};

pub mod fill;
pub mod shockwave;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Effect>>()
            .add_systems(FixedUpdate, fill::update_fill_effect)
            .add_systems(FixedUpdate, fill::apply_fill_effect)
            .add_systems(FixedUpdate, shockwave::update_shockwave_effect)
            .add_systems(FixedUpdate, shockwave::apply_shockwave_effect);
    }
}

// TODO: extract certain values into different structs, since some of
// these are used in the component form and some are used in the stored
// form.
#[derive(Component, Debug, Clone)]
pub struct Effect {
    pub groups: Vec<u32>,
    pub current_time: f64,
    pub init_info: EffectInfo,
}

#[derive(Debug, Clone)]
pub enum EffectInfo {
    ColorFill(ColorFillEffect),
    ColorShockwave(ColorShockwaveEffect),
}

impl EffectInfo {
    pub fn insert_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            EffectInfo::ColorFill(color_fill_effect) => {
                entity_commands.insert(color_fill_effect.clone());
            }
            EffectInfo::ColorShockwave(color_shockwave_effect) => {
                entity_commands.insert(color_shockwave_effect.clone());
            }
        }
    }
}
