use bevy::prelude::*;

use crate::{
    network::{ArtNetBuffers, ArtNetDataPointer},
    timeline::sequence_tree::SequenceTree,
    util::blending::{BlendingMode, colors::blend_colors, pan_tilt::blend_pan_tilt},
};

pub mod color_light;

/// Bevy plugin for fixtures.
pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_fixtures)
            .add_systems(FixedUpdate, add_color_data_to_buffer)
            .add_systems(FixedUpdate, add_pan_tilt_data_to_buffer)
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
#[derive(Component, Debug)]
#[require(Fixture)]
pub struct ColorFixture {
    pub color: Color,
    pub encoding: RgbEncoding,
    pub red_channel: u8,
    pub green_channel: u8,
    pub blue_channel: u8,
    pub white_channel: Option<u8>,
}

impl Default for ColorFixture {
    fn default() -> Self {
        Self {
            color: Color::BLACK.with_alpha(0.0),
            encoding: RgbEncoding::default(),
            red_channel: 0,
            green_channel: 1,
            blue_channel: 2,
            white_channel: None,
        }
    }
}

/// Enum that represents the encoding of the color data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RgbEncoding {
    #[default]
    Linear,
    Srgb,
}

/// Simple data struct that represents the pan and tilt angles of a fixture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanTilt {
    pub pan: f32,
    pub tilt: f32,
}

impl Default for PanTilt {
    fn default() -> Self {
        Self {
            pan: 0.0,
            tilt: 0.0,
        }
    }
}

impl PanTilt {
    pub fn new(pan: f32, tilt: f32) -> Self {
        Self { pan, tilt }
    }
}

/// Bevy component that is attached to any fixtures that can pan/tilt. Both
/// angles are in degrees, with 0 being the default position.
#[derive(Component, Debug, Default)]
#[require(Fixture)]
pub struct PanTiltFixture {
    pub pan: f32,
    pub tilt: f32,
    pub pan_range: (f32, f32),
    pub tilt_range: (f32, f32),
}

/// Simple data struct used to group important request information together
/// when pulling data from the scene tree.
#[derive(Debug, Clone)]
pub struct FixtureRequest {
    pub groups: Vec<u32>,
    pub position: Vec3,
    pub has_color: bool,
    pub has_pan_tilt: bool,
}

impl FixtureRequest {
    pub fn default_response(&self) -> FixtureResponse {
        FixtureResponse {
            color: if self.has_color {
                Some(Color::BLACK.with_alpha(0.0))
            } else {
                None
            },
            pan_tilt: if self.has_pan_tilt {
                Some(PanTilt::default())
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
    pub pan_tilt: Option<PanTilt>,
}

impl FixtureResponse {
    pub fn color_only(color: Color) -> Self {
        Self {
            color: Some(color),
            pan_tilt: None,
        }
    }

    pub fn pan_tilt_only(pan_tilt: PanTilt) -> Self {
        Self {
            color: None,
            pan_tilt: Some(pan_tilt),
        }
    }

    pub fn merge_in_place(
        &mut self,
        other: &FixtureResponse,
        factor: f32,
        blending_mode: BlendingMode,
    ) {
        if self.color.is_some() || other.color.is_some() {
            let self_col = self.color.unwrap_or_else(|| Color::BLACK.with_alpha(0.0));
            let other_col = other.color.unwrap_or_else(|| Color::BLACK.with_alpha(0.0));
            self.color = Some(blend_colors(self_col, other_col, factor, blending_mode));
        };
        if self.pan_tilt.is_some() || other.pan_tilt.is_some() {
            let self_pt = self.pan_tilt.unwrap_or_default();
            let other_pt = other.pan_tilt.unwrap_or_default();
            self.pan_tilt = Some(blend_pan_tilt(self_pt, other_pt, factor, blending_mode));
        };
    }
}

/// Bevy system that updates all fixture information, pulling from the sequence
/// tree. Expects the sequence tree to be up to date.
pub fn update_fixtures(
    sequence_tree: Res<SequenceTree>,
    mut fixture_query: Query<(
        &mut Fixture,
        Option<&mut ColorFixture>,
        Option<&mut PanTiltFixture>,
        &GlobalTransform,
    )>,
) {
    let mut fixture_reqs: Vec<FixtureRequest> = Vec::new();

    for (fixture, color_fixture, pan_tilt_fixture, transform) in fixture_query.iter_mut() {
        fixture_reqs.push(FixtureRequest {
            groups: fixture.groups.clone(),
            position: transform.translation(),
            has_color: color_fixture.is_some(),
            has_pan_tilt: pan_tilt_fixture.is_some(),
        })
    }

    let values = sequence_tree.get_values_recursive(&fixture_reqs);

    assert_eq!(values.len(), fixture_reqs.len());

    for ((_, color_fixture, pan_tilt_fixture, _), value) in fixture_query.iter_mut().zip(values) {
        if let (Some(mut color_fixture), Some(color_value)) = (color_fixture, value.color) {
            color_fixture.color = color_value;
        }
        if let (Some(mut pan_tilt_fixture), Some(pan_tilt_value)) =
            (pan_tilt_fixture, value.pan_tilt)
        {
            pan_tilt_fixture.pan = pan_tilt_value.pan;
            pan_tilt_fixture.tilt = pan_tilt_value.tilt;
        }
    }
}

/// Bevy system that adds RGB fixture information to the ArtNet buffer.
pub fn add_color_data_to_buffer(
    mut buffers: ResMut<ArtNetBuffers>,
    color_query: Query<(&ArtNetDataPointer, &ColorFixture)>,
) {
    for (pointer, fixture) in color_query.iter() {
        let (mut r, mut g, mut b) = match fixture.encoding {
            RgbEncoding::Linear => {
                let c = fixture.color.to_linear();
                (c.red, c.green, c.blue)
            }
            RgbEncoding::Srgb => {
                let c = fixture.color.to_srgba();
                (c.red, c.green, c.blue)
            }
        };

        let w = fixture.white_channel.map(|_| {
            let w = r.min(g).min(b);
            r -= w;
            g -= w;
            b -= w;
            w
        });

        let result: Result<(), String> = (|| {
            buffers.write(
                pointer.offset_by(fixture.red_channel as u16)?,
                (r * 255.0).clamp(0.0, 255.0) as u8,
            )?;
            buffers.write(
                pointer.offset_by(fixture.green_channel as u16)?,
                (g * 255.0).clamp(0.0, 255.0) as u8,
            )?;
            buffers.write(
                pointer.offset_by(fixture.blue_channel as u16)?,
                (b * 255.0).clamp(0.0, 255.0) as u8,
            )?;
            if let (Some(white_offset), Some(w)) = (fixture.white_channel, w) {
                buffers.write(
                    pointer.offset_by(white_offset as u16)?,
                    (w * 255.0).clamp(0.0, 255.0) as u8,
                )?;
            }
            Ok(())
        })();

        if let Err(e) = result {
            warn!("Failed to write fixture DMX data: {}", e);
        }
    }
}

/// Bevy system that adds pan/tilt fixture information to the ArtNet buffer.
pub fn add_pan_tilt_data_to_buffer(
    mut buffers: ResMut<ArtNetBuffers>,
    pan_tilt_query: Query<(&ArtNetDataPointer, &PanTiltFixture)>,
) {
    for (pointer, fixture) in pan_tilt_query.iter() {
        let pan = ((fixture.pan - fixture.pan_range.0)
            / (fixture.pan_range.1 - fixture.pan_range.0)
            * 255.0)
            .clamp(0.0, 255.0) as u8;
        let tilt = ((fixture.tilt - fixture.tilt_range.0)
            / (fixture.tilt_range.1 - fixture.tilt_range.0)
            * 255.0)
            .clamp(0.0, 255.0) as u8;

        let result: Result<(), String> = (|| {
            buffers.write(pointer.offset_by(0)?, pan)?;
            buffers.write(pointer.offset_by(1)?, tilt)?;
            Ok(())
        })();

        if let Err(e) = result {
            warn!("Failed to write fixture DMX data: {}", e);
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
        // TODO: do this with base color, make that rgb or smth
        let with_alpha = blend_colors(
            Color::BLACK,
            color_fixture.color,
            color_fixture.color.alpha(),
            BlendingMode::Mix,
        );
        material.color = with_alpha;
    }
}
