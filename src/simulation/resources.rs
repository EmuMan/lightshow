use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ActiveEffects {
    pub effects: Vec<(Vec<Keyframe>, Effect)>,
}

#[derive(Debug)]
pub enum Effect {
    Fill { color: Color, groups: Vec<u32> },
    Pulse {
        color: Color,
        groups: Vec<u32>,
        center: Vec3,
        speed: f32,
        flat: f32,
        head: f32,
        tail: f32,
    },
}

#[derive(Debug, Default)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub interpolation: InterpolationType,
}

#[derive(Default, Debug)]
pub enum InterpolationType {
    #[default]
    LINEAR,
    CONSTANT,
}

#[derive(Resource, Default, Debug)]
pub struct View {
    pub location: Vec3,
    pub zoom: f32,
}

#[derive(Resource, Default, Debug)]
pub struct SimulationTime {
    pub time: f32,
}
