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
        Fixture { groups },
        artnet.unwrap_or_default(),
    ));
}

pub fn apply_color_light_color_queues(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lights_query: Query<(&MeshMaterial2d<ColorMaterial>, &mut ColorLight)>,
) {
    for (material, mut light) in &mut lights_query {
        let mut new_color = Color::BLACK;
        for color in &light.color_queue {
            new_color = apply_color(&new_color, color);
        }
        let new_material = materials.get_mut(material).unwrap();
        new_material.color = new_color;
        light.color_queue.clear();
    }
}

fn apply_color(orig_color: &Color, new_color: &Color) -> Color {
    let orig_srgba = orig_color.to_srgba();
    let new_srgba = new_color.to_srgba();
    LinearRgba::new(
        orig_srgba.red + new_srgba.red,
        orig_srgba.green + new_srgba.green,
        orig_srgba.blue + new_srgba.blue,
        orig_srgba.alpha + new_srgba.alpha,
    )
    .into()
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
