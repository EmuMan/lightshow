use bevy::prelude::*;

pub mod fill;
pub mod shockwave;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, fill::update_fill_effect)
            .add_systems(FixedUpdate, fill::apply_fill_effect)
            .add_systems(FixedUpdate, shockwave::update_shockwave_effect)
            .add_systems(FixedUpdate, shockwave::apply_shockwave_effect);
    }
}

#[derive(Component)]
pub struct Effect {
    pub groups: Vec<u32>,
    pub start: f64,
    pub end: f64,
}

#[derive(Component)]
pub struct FillEffect {
    pub color: Color,
}

#[derive(Component)]
pub struct ShockwaveEffect {
    pub color: Color,
    pub center: Vec3,
    pub radius: f32,
    pub flat: f32,
    pub head: f32,
    pub tail: f32,
}
