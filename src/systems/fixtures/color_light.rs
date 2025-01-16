use bevy::prelude::*;

use crate::components::fixtures::*;

pub fn spawn_color_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    radius: f32,
    groups: Vec<u32>,
) {
    commands.spawn(
        (
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            transform,
            ColorLight {
                radius,
                color_queue: Vec::new(),
            },
            Fixture {
                groups,
            },
        )
    );
}

pub fn apply_color_light_color_queues(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lights_query: Query<(
        &MeshMaterial2d<ColorMaterial>,
        &mut ColorLight,
    )>,
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

fn apply_color(
    orig_color: &Color,
    new_color: &Color,
) -> Color {
    let orig_srgba = orig_color.to_srgba();
    let new_srgba = new_color.to_srgba();
    LinearRgba::new(
        orig_srgba.red + new_srgba.red,
        orig_srgba.green + new_srgba.green,
        orig_srgba.blue + new_srgba.blue,
        orig_srgba.alpha + new_srgba.alpha,
    ).into()
}
