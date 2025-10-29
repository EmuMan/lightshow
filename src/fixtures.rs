use bevy::prelude::*;
use derive_more::From;

use crate::util::blending::colors::{ColorBlendingMode, blend_colors};

pub mod color_light;

pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, color_light::add_data_to_buffer);
    }
}

#[derive(Component)]
pub struct Fixture {
    pub groups: Vec<u32>,
    pub input_type: FixtureType,
}

#[derive(Component)]
pub struct ColorLight {
    pub radius: f32,
    pub color_queue: Vec<Color>,
}

#[derive(Debug, Clone, Copy)]
pub enum FixtureType {
    Color,
    Vec3,
    Combined,
}

impl FixtureType {
    pub fn get_default_input(&self) -> FixtureInput {
        match self {
            FixtureType::Color => Color::NONE.into(),
            FixtureType::Vec3 => Vec3::ZERO.into(),
            FixtureType::Combined => (Color::NONE, Vec3::ZERO).into(),
        }
    }
}

#[derive(Debug, From, Clone, Copy)]
pub enum FixtureInput {
    Color(Color),
    Vec3(Vec3),
    Combined(Color, Vec3),
}

impl FixtureInput {
    pub fn merge(
        &mut self,
        other: &FixtureInput,
        factor: f32,
        color_blending_mode: ColorBlendingMode,
    ) {
        match (self, other) {
            (FixtureInput::Color(self_color), FixtureInput::Color(other_color)) => {
                *self_color = blend_colors(self_color, other_color, factor, color_blending_mode);
            }
            (FixtureInput::Vec3(self_vec3), FixtureInput::Vec3(other_vec3)) => {
                // TODO: implement blending for vec3s
            }
            (
                FixtureInput::Combined(self_color, self_vec3),
                FixtureInput::Combined(other_color, other_vec3),
            ) => {
                // TODO: implement blending for combineds
            }
            // It is okay if the other input is just one of the two if self is combined.
            // It will just get merged with its respective type.
            (FixtureInput::Combined(self_color, _self_vec3), FixtureInput::Color(other_color)) => {
                *self_color = blend_colors(self_color, other_color, factor, color_blending_mode);
            }
            (FixtureInput::Combined(_self_color, self_vec3), FixtureInput::Vec3(other_vec3)) => {
                // TODO: Implement blending for vec3s
            }
            _ => panic!("attempted to merge incompatible fixture input types"),
        }
    }
}

pub trait FixtureInputVec {
    fn merge_all(self, other: &[FixtureInput], factor: f32, color_blending_mode: ColorBlendingMode);
}

impl FixtureInputVec for &mut [FixtureInput] {
    fn merge_all(
        self,
        other: &[FixtureInput],
        factor: f32,
        color_blending_mode: ColorBlendingMode,
    ) {
        assert_eq!(self.len(), other.len());
        for (self_fixture, other_fixture) in self.iter_mut().zip(other) {
            self_fixture.merge(other_fixture, factor, color_blending_mode);
        }
    }
}
