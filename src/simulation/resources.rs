use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ActiveEffects {
    pub effects: Vec<(f64, f64, Vec<Keyframe>, Effect)>,
}

#[derive(Debug)]
pub enum Effect {
    Fill {
        color: Color,
        groups: Vec<u32>
    },
    Pulse {
        color: Color,
        groups: Vec<u32>,
        center: Vec3,
        radius: f32,
        flat: f32,
        head: f32,
        tail: f32,
    },
}

#[derive(Debug)]
pub struct Keyframe {
    pub time: f64,
    pub interpolation: InterpolationType,
    pub key: String,
    pub value: KeyframeValue,
}

#[derive(Debug)]
pub enum KeyframeValue {
    FloatKeyframe(f32),
    ColorKeyframe(Color),
    Vec3Keyframe(Vec3),
}

#[derive(Default, Debug, Clone, Copy)]
pub enum InterpolationType {
    #[default]
    LINEAR,
    CONSTANT,
}

#[derive(Resource, Default, Debug)]
pub struct SimulationTime {
    pub time: f64,
}
