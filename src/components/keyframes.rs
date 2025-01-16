use bevy::prelude::*;

#[derive(Component)]
pub struct Keyframes {
    pub keyframes: Vec<Keyframe>,
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
