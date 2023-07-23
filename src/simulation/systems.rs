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

    let keyframes = vec![
        Keyframe {
            time: 0.,
            interpolation: InterpolationType::LINEAR,
            key: "radius".to_string(),
            value: KeyframeValue::FloatKeyframe(0.),
        },
        Keyframe {
            time: 3.,
            interpolation: InterpolationType::LINEAR,
            key: "radius".to_string(),
            value: KeyframeValue::FloatKeyframe(3.),
        },
    ];

    let effect = Effect::Pulse {
        color: Color::WHITE,
        groups: vec![0],
        center: Vec3::ZERO,
        radius: 0.,
        flat: 1.,
        head: 1.,
        tail: 1.,
    };

    effects.effects.push((0., 5., keyframes, effect));
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
    mut effects: ResMut<ActiveEffects>,
    time: Res<SimulationTime>,
    material_query: Query<(&Handle<ColorMaterial>, &Light)>,
) {

    // reset all to zero
    for (material, _) in &material_query {
        let color = &mut materials.get_mut(material).unwrap().color;
        color.set_r(0.);
        color.set_g(0.);
        color.set_b(0.);
    }

    //update for each effect
    for (start_time, end_time, keyframes, effect) in effects.effects.iter_mut() {
        if time.time < *start_time || time.time > *end_time {
            continue;
        }
        // this line is the reason i chose to separate the light iterations as such.
        // it's a little inconvenient to have the two loops, but it ensures that each
        // effect is only updated once, which is the more costly operation.
        update_effect(effect, keyframes, time.time);
        for (material, light) in &material_query {
            let color = &mut materials.get_mut(material).unwrap().color;
            apply_effect(color, &light.location, effect, &light.groups);
        }
    }
}

pub fn update_effect(
    effect: &mut Effect,
    keyframes: &Vec<Keyframe>,
    time: f32,
) {
    match effect {
        Effect::Fill {
            color,
            ..
        } => {
            *color = get_color_value(keyframes, "color", time, color);
        }
        Effect::Pulse {
            color,
            center,
            radius,
            flat,
            head,
            tail,
            ..
        } => {
            *color = get_color_value(keyframes, "color", time, color);
            *center = get_vec3_value(keyframes, "center", time, center);
            *radius = get_float_value(keyframes, "radius", time, radius);
            *flat = get_float_value(keyframes, "flat", time, flat);
            *head = get_float_value(keyframes, "head", time, head);
            *tail = get_float_value(keyframes, "tail", time, tail);
        }
    };
}

pub fn apply_effect(
    orig_color: &mut Color,
    location: &Vec3,
    effect: &Effect,
    light_groups: &Vec<u32>,
) {
    match effect {
        Effect::Fill {
            color,
            groups
        } => {
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
            radius,
            flat,
            head,
            tail
        } => {
            if !groups.iter().any(|item| light_groups.contains(item)) {
                return;
            }
            
            apply_shockwave(
                orig_color,
                color,
                *location - *center,
                *radius,
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
    orig_color: &mut Color,
    color: &Color,
    displacement: Vec3,
    radius: f32,
    flat: f32,
    head: f32,
    tail: f32
) {
    let distance = displacement.length();
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

fn interpolate_float(
    start: f32,
    end: f32,
    time: f32,
    interpolation: InterpolationType,
) -> f32 {
    match interpolation {
        InterpolationType::CONSTANT => {
            start
        }
        InterpolationType::LINEAR => {
            start + (end - start) * time
        }
    }
}

fn interpolate_color(
    start: Color,
    end: Color,
    time: f32,
    interpolation: InterpolationType,
) -> Color {
    Color::rgba(
        interpolate_float(start.r(), end.r(), time, interpolation),
        interpolate_float(start.g(), end.g(), time, interpolation),
        interpolate_float(start.b(), end.b(), time, interpolation),
        interpolate_float(start.a(), end.a(), time, interpolation),
    )
}

fn interpolate_vec3(
    start: Vec3,
    end: Vec3,
    time: f32,
    interpolation: InterpolationType,
) -> Vec3 {
    Vec3::new(
        interpolate_float(start.x, end.x, time, interpolation),
        interpolate_float(start.y, end.y, time, interpolation),
        interpolate_float(start.z, end.z, time, interpolation),
    )
}

fn get_surrounding_keyframes<'a>(
    keyframes: &'a Vec<Keyframe>,
    key: &str,
    time: f32,
) -> (Option<&'a Keyframe>, Option<&'a Keyframe>) {
    let mut start_keyframe: Option<&Keyframe> = None {};
    let mut end_keyframe: Option<&Keyframe> = None {};

    for keyframe in keyframes.iter() {
        if keyframe.key != *key {
            continue;
        }
        if keyframe.time > time {
            end_keyframe = Some(keyframe);
            break;
        }
        if keyframe.time < time {
            start_keyframe = Some(keyframe);
        }
    }

    (start_keyframe, end_keyframe)
}

// TODO: Maybe clean this tragedy up
fn get_float_value(
    keyframes: &Vec<Keyframe>,
    key: &str,
    time: f32,
    default: &f32,
) -> f32 {
    let (start_opt, end_opt) =
        get_surrounding_keyframes(keyframes, key, time);
    
    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::FloatKeyframe(start_value) = start_keyframe.value else {
            panic!("tried to interpolate float from non-float start keyframe with key {}", key.to_string());
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::FloatKeyframe(end_value) = end_keyframe.value else {
                panic!("tried to interpolate float from non-float end keyframe with key {}", key.to_string());
            };
            // both start and end
            return interpolate_float(
                start_value,
                end_value,
                (time - start_keyframe.time) /
                    (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::FloatKeyframe(end_value) = end_keyframe.value else {
            panic!("tried to interpolate float from non-float end keyframe with key {}", key.to_string());
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}

// TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
fn get_color_value(
    keyframes: &Vec<Keyframe>,
    key: &str,
    time: f32,
    default: &Color,
) -> Color {
    let (start_opt, end_opt) =
        get_surrounding_keyframes(keyframes, key, time);
    
    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::ColorKeyframe(start_value) = start_keyframe.value else {
            panic!("tried to interpolate color from non-float start keyframe with key {}", key.to_string());
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::ColorKeyframe(end_value) = end_keyframe.value else {
                panic!("tried to interpolate color from non-float end keyframe with key {}", key.to_string());
            };
            // both start and end
            return interpolate_color(
                start_value,
                end_value,
                (time - start_keyframe.time) /
                    (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::ColorKeyframe(end_value) = end_keyframe.value else {
            panic!("tried to interpolate color from non-float end keyframe with key {}", key.to_string());
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}

// TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
fn get_vec3_value(
    keyframes: &Vec<Keyframe>,
    key: &str,
    time: f32,
    default: &Vec3,
) -> Vec3 {
    let (start_opt, end_opt) =
        get_surrounding_keyframes(keyframes, key, time);
    
    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::Vec3Keyframe(start_value) = start_keyframe.value else {
            panic!("tried to interpolate vec3 from non-float start keyframe with key {}", key.to_string());
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::Vec3Keyframe(end_value) = end_keyframe.value else {
                panic!("tried to interpolate vec3 from non-float end keyframe with key {}", key.to_string());
            };
            // both start and end
            return interpolate_vec3(
                start_value,
                end_value,
                (time - start_keyframe.time) /
                    (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::Vec3Keyframe(end_value) = end_keyframe.value else {
            panic!("tried to interpolate vec3 from non-float end keyframe with key {}", key.to_string());
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}
