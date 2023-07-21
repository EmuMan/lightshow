use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::resources::*;

pub fn initialize_view(
    mut view: ResMut<View>,
) {
    view.zoom = 20.;
}

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut effects: ResMut<ActiveEffects>,
) {
    for i in 0..10 {
        for j in 0..10 {
            spawn_light(
                &mut commands,
                &mut meshes,
                &mut materials,
                Light {
                    groups: vec![0],
                    location: Vec3::new(i as f32 - 4.5, j as f32 - 4.5, 0.),
                    radius: 10.,
                }
            );
        }
    }

    let effect = Effect::Pulse {
        color: Color::WHITE,
        groups: vec![0],
        center: Vec3::ZERO,
        speed: 1.,
        flat: 1.,
        head: 1.,
        tail: 1.,
    };

    let keyframes = vec![
        Keyframe {
            time: 0.,
            value: 0.,
            interpolation: InterpolationType::LINEAR,
        },
        Keyframe {
            time: 3.,
            value: 3.,
            interpolation: InterpolationType::LINEAR,
        },
    ];

    effects.effects.push((keyframes, effect));
}

pub fn increment_time(
    time: Res<Time>,
    mut simulation_time: ResMut<SimulationTime>,
) {
    simulation_time.time += time.delta_seconds();
}

pub fn spawn_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    light: Light,
) {
    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(light.radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            light
        )
    );
}

pub fn update_light_positions(
    view: Res<View>,
    mut transform_query: Query<(&mut Transform, &Light)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for (mut transform, light) in transform_query.iter_mut() {
        let light_offset = (light.location - view.location) * view.zoom;
        let window_offset = Vec3::new(window.width() / 2., window.height() / 2., 0.);
        transform.translation = light_offset + window_offset;
    }
}

pub fn update_light_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    effects: Res<ActiveEffects>,
    time: Res<SimulationTime>,
    material_query: Query<(&Handle<ColorMaterial>, &Light)>,
) {
    for (_, effect) in &effects.effects {
        for (material, light) in &material_query {
            let color = &mut materials.get_mut(material).unwrap().color;
            color.set_r(0.);
            color.set_g(0.);
            color.set_b(0.);
            apply_effect(color, &light.location, time.time, effect, &light.groups);
        }
    }
}

pub fn apply_effect(
    orig_color: &mut Color,
    location: &Vec3,
    time: f32,
    effect: &Effect,
    light_groups: &Vec<u32>,
) {
    match effect {
        Effect::Fill { color, groups } => {
            if !groups.iter().any(|item| light_groups.contains(item)) {
                return;
            }
            apply_fill(
                orig_color,
                color,
            )
        }
        Effect::Pulse {
            color,
            groups,
            center,
            speed,
            flat,
            head,
            tail
        } => {
            if !groups.iter().any(|item| light_groups.contains(item)) {
                return;
            }
            apply_shockwave(
                time,
                orig_color,
                color,
                *location - *center,
                *speed,
                *flat,
                *head,
                *tail,
            );
        }
    }
}

pub fn apply_fill(
    orig_color: &mut Color,
    color: &Color,
) {
    orig_color.set_r(orig_color.r() + color.r());
    orig_color.set_g(orig_color.g() + color.g());
    orig_color.set_b(orig_color.b() + color.b());
}

pub fn apply_shockwave(
    time: f32,
    orig_color: &mut Color,
    color: &Color,
    displacement: Vec3,
    speed: f32,
    flat: f32,
    head: f32,
    tail: f32
) {
    let distance = displacement.length();
    let radius = time * speed;
    // println!("{radius}, {distance}");
    let half_flat = flat / 2.;
    let mut influence: f32 = 0.;

    if distance > radius - half_flat && distance < radius + half_flat {
        influence = 1.;
    } else if distance > radius + half_flat && distance < radius + half_flat + head {
        influence = ((radius + half_flat + head) - distance) / head;
    } else if distance > radius - half_flat - tail && distance < radius - half_flat {
        influence = (distance - (radius - half_flat - tail)) / tail;
    }

    orig_color.set_r(orig_color.r() + color.r() * influence);
    orig_color.set_g(orig_color.g() + color.g() * influence);
    orig_color.set_b(orig_color.b() + color.b() * influence);
}
