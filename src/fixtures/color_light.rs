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
        ColorLight {
            radius,
            color_queue: Vec::new(),
        },
        Fixture {
            groups,
            input_type: FixtureType::Color,
        },
        artnet.unwrap_or_default(),
    ));
}

pub fn add_data_to_buffer(
    materials: Res<Assets<ColorMaterial>>,
    mut connections: ResMut<ArtNetConnections>,
    query: Query<(&ArtNetNode, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (node, material) in &mut query.iter() {
        if !connections.connection_exists(&node.ip, node.port, node.universe) {
            let connection = ArtNetConnection::new(&node.ip, node.port, node.universe);
            if let Some(connection) = connection {
                connections.add_connection(connection);
            } else {
                continue;
            }
        }

        let connection = connections.get_connection_mut(&node.ip, node.port, node.universe);
        let material = materials.get(material).unwrap();
        let color = material.color;
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
