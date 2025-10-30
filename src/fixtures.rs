use bevy::{prelude::*, sprite_render::AlphaMode2d};
use derive_more::From;

use crate::{
    network::{ArtNetConnection, ArtNetConnections, ArtNetNode},
    timeline::sequence_tree::{FixtureInfo, SequenceTree},
    util::blending::colors::{ColorBlendingMode, blend_colors},
};

pub mod color_light;

pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ColorFixturesPlugin)
            .add_systems(FixedUpdate, update_fixtures)
            .add_systems(FixedUpdate, apply_color_fixture_pending_values)
            .add_systems(FixedUpdate, add_data_to_buffer);
    }
}

#[derive(Component, Debug, Default)]
#[require(Transform, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct Fixture {
    pub groups: Vec<u32>,
    pub input_type: FixtureType,
    pub pending_value: Option<FixtureInput>,
}

impl Fixture {
    pub fn new(groups: Vec<u32>, input_type: FixtureType) -> Self {
        Self {
            groups,
            input_type,
            pending_value: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FixtureType {
    Color,
    Vec3,
    #[default]
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

pub fn update_fixtures(
    sequence_tree: Res<SequenceTree>,
    mut fixture_query: Query<(&mut Fixture, &GlobalTransform)>,
) {
    let mut fixture_info: Vec<FixtureInfo> = Vec::new();

    for (fixture, transform) in fixture_query.iter_mut() {
        fixture_info.push(FixtureInfo {
            groups: fixture.groups.clone(),
            input_type: fixture.input_type,
            position: transform.translation(),
        })
    }

    let values = sequence_tree.get_values_recursive(&fixture_info);

    assert_eq!(values.len(), fixture_info.len());

    for ((mut fixture, _transform), value) in fixture_query.iter_mut().zip(values) {
        fixture.pending_value = Some(value);
    }
}

pub struct ColorFixturesPlugin;

impl Plugin for ColorFixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_color_fixture_pending_values);
    }
}

#[derive(Debug, Default, Component)]
#[require(Fixture)]
pub struct ColorFixture {
    pub color: Color,
}

pub fn apply_color_fixture_pending_values(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fixture_query: Query<(&Fixture, &mut ColorFixture, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (fixture, mut color_fixture, mesh_material) in fixture_query.iter_mut() {
        let Some(pending_value) = fixture.pending_value else {
            continue;
        };

        match pending_value {
            FixtureInput::Color(color) => {
                color_fixture.color = color;
            }
            FixtureInput::Vec3(_vec3) => {
                panic!("color fixture expected FixtureInput::Color, received FixtureInput::Vec3");
            }
            FixtureInput::Combined(color, _vec3) => {
                color_fixture.color = color;
            }
        }

        let material = materials.get_mut(mesh_material).unwrap();
        // alpha just means the LED is off, not that it becomes transparent.
        let with_alpha = blend_colors(
            &Color::BLACK,
            &color_fixture.color,
            color_fixture.color.alpha(),
            ColorBlendingMode::Mix,
        );
        material.color = with_alpha;
    }
}

pub fn add_data_to_buffer(
    mut connections: ResMut<ArtNetConnections>,
    query: Query<(&ArtNetNode, &ColorFixture)>,
) {
    for (node, color_fixture) in &mut query.iter() {
        if !connections.connection_exists(&node.ip, node.port, node.universe) {
            let connection = ArtNetConnection::new(&node.ip, node.port, node.universe);
            if let Some(connection) = connection {
                connections.add_connection(connection);
            } else {
                continue;
            }
        }

        let connection = connections.get_connection_mut(&node.ip, node.port, node.universe);
        let color = color_fixture.color;
        let srgba = color.to_srgba();

        if let Some(connection) = connection {
            connection
                .data_buffer
                .set_channel(node.channels[0], (srgba.red * 255.0) as u8);
            connection
                .data_buffer
                .set_channel(node.channels[1], (srgba.green * 255.0) as u8);
            connection
                .data_buffer
                .set_channel(node.channels[2], (srgba.blue * 255.0) as u8);
        }
    }
}
