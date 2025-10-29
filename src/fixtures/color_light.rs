use bevy::prelude::*;

use crate::fixtures::*;
use crate::network::*;

pub fn spawn_color_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    radius: f32,
    groups: Vec<u32>,
    artnet: Option<ArtNetNode>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        transform,
        Fixture::new(groups, FixtureType::Color),
        ColorFixture::default(),
        artnet.unwrap_or_default(),
    ));
}
