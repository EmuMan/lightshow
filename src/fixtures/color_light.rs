use bevy::prelude::*;

use crate::{
    fixtures::{ColorFixture, Fixture},
    network::ArtNetDataPointer,
};

pub fn spawn_color_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    radius: f32,
    color_fixture: ColorFixture,
    groups: Vec<u32>,
    artnet_data_pointer: Option<ArtNetDataPointer>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        transform,
        Fixture::new(groups),
        color_fixture,
        // TODO: unwrap or none, not default.
        artnet_data_pointer.unwrap_or_default(),
    ));
}
