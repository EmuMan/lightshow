use bevy::prelude::*;

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
