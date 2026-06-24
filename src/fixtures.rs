use bevy::prelude::*;

use crate::{
    network::{ArtNetConnection, ArtNetConnections, ArtNetNode},
    timeline::sequence_tree::SequenceTree,
    util::blending::colors::{ColorBlendingMode, blend_colors},
};

pub mod color_light;

/// Bevy plugin for fixtures.
pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_fixtures)
            .add_systems(FixedUpdate, add_data_to_buffer)
            .add_systems(Update, apply_color_fixture_material);
    }
}

/// Bevy component that represents a light or other fixture within the scene.
/// Holds common information including what groups it is a part of.
///
/// TODO: Add fixture series eventually.
#[derive(Component, Debug, Default)]
#[require(Transform, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct Fixture {
    pub groups: Vec<u32>,
}

impl Fixture {
    pub fn new(groups: Vec<u32>) -> Self {
        Self { groups }
    }
}

/// Bevy component that is attached to any fixtures that use a color.
#[derive(Debug, Default, Component)]
#[require(Fixture)]
pub struct ColorFixture {
    pub color: Color,
}

/// Bevy component that is attached to any fixtures that use a vec3.
#[derive(Debug, Default, Component)]
#[require(Fixture)]
pub struct Vec3Fixture {
    pub vec3: Vec3,
}

/// Simple data struct used to group important request information together
/// when pulling data from the scene tree.
#[derive(Debug, Clone)]
pub struct FixtureRequest {
    pub groups: Vec<u32>,
    pub position: Vec3,
    pub has_color: bool,
    pub has_vec3: bool,
}

impl FixtureRequest {
    pub fn default_response(&self) -> FixtureResponse {
        FixtureResponse {
            color: if self.has_color {
                Some(Color::BLACK.with_alpha(0.0))
            } else {
                None
            },
            vec3: if self.has_vec3 {
                Some(Vec3::ZERO)
            } else {
                None
            },
        }
    }
}

/// Simple data struct used to return requested information from the scene
/// tree.
#[derive(Debug, Clone, Default)]
pub struct FixtureResponse {
    pub color: Option<Color>,
    pub vec3: Option<Vec3>,
}

impl FixtureResponse {
    pub fn color_only(color: Color) -> Self {
        Self {
            color: Some(color),
            vec3: None,
        }
    }

    pub fn vec3_only(vec3: Vec3) -> Self {
        Self {
            color: None,
            vec3: Some(vec3),
        }
    }

    pub fn merge_in_place(
        &mut self,
        other: &FixtureResponse,
        factor: f32,
        blending_mode: ColorBlendingMode,
    ) {
        if self.color.is_some() || other.color.is_some() {
            let self_col = self.color.unwrap_or_else(|| Color::BLACK.with_alpha(0.0));
            let other_col = other.color.unwrap_or_else(|| Color::BLACK.with_alpha(0.0));
            self.color = Some(blend_colors(&self_col, &other_col, factor, blending_mode));
        };
        // TODO: implement Vec3s
    }
}

/// Bevy system that updates all fixture information, pulling from the sequence
/// tree. Expects the sequence tree to be up to date.
pub fn update_fixtures(
    sequence_tree: Res<SequenceTree>,
    mut fixture_query: Query<(
        &mut Fixture,
        Option<&mut ColorFixture>,
        Option<&mut Vec3Fixture>,
        &GlobalTransform,
    )>,
) {
    let mut fixture_reqs: Vec<FixtureRequest> = Vec::new();

    for (fixture, color_fixture, vec3_fixture, transform) in fixture_query.iter_mut() {
        fixture_reqs.push(FixtureRequest {
            groups: fixture.groups.clone(),
            position: transform.translation(),
            has_color: color_fixture.is_some(),
            has_vec3: vec3_fixture.is_some(),
        })
    }

    let values = sequence_tree.get_values_recursive(&fixture_reqs);

    assert_eq!(values.len(), fixture_reqs.len());

    for ((_, color_fixture, vec3_fixture, _), value) in fixture_query.iter_mut().zip(values) {
        if let (Some(mut color_fixture), Some(color_value)) = (color_fixture, value.color) {
            println!("Setting color!");
            color_fixture.color = color_value;
        }
        if let (Some(mut vec3_fixture), Some(vec3_value)) = (vec3_fixture, value.vec3) {
            vec3_fixture.vec3 = vec3_value;
        }
    }
}

/// Bevy system that adds fixture color information to the ArtNet buffer.
///
/// TODO: Support other types of information! All necessary data should be sent
/// as one package, not chunked up into color, rotation, etc.
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
                .set_channel(node.channels[0], (srgba.red * srgba.alpha * 255.0) as u8);
            connection
                .data_buffer
                .set_channel(node.channels[1], (srgba.green * srgba.alpha * 255.0) as u8);
            connection
                .data_buffer
                .set_channel(node.channels[2], (srgba.blue * srgba.alpha * 255.0) as u8);
        }
    }
}

/// Bevy system that updates the materials of fixtures that have color fixture
/// components in the visual representation.
pub fn apply_color_fixture_material(
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&ColorFixture, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (color_fixture, mesh_material) in query.iter() {
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
