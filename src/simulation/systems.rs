use bevy::prelude::*;

use super::components::*;
use super::resources::*;

pub fn pulse_test_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut effects: ResMut<ActiveEffects>,
) {
    for i in 0..10 {
        for j in 0..10 {
            spawn_rgb_light(
                &mut commands,
                &mut meshes,
                &mut materials,
                Transform::from_translation(Vec3::new((i as f32 * 30.) - 135., (j as f32 * 30.) as f32 - 135., 0.)),
                RgbLight {
                    groups: vec![0],
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
            value: KeyframeValue::FloatKeyframe(250.),
        },
    ];

    let effect = Effect::Pulse {
        color: Color::WHITE,
        groups: vec![0],
        center: Vec3::ZERO,
        radius: 0.,
        flat: 30.,
        head: 30.,
        tail: 30.,
    };

    effects.effects.push((0., 5., keyframes, effect));
}

pub fn increment_time(
    time: Res<Time>,
    mut simulation_time: ResMut<SimulationTime>,
) {
    simulation_time.time += time.delta_secs_f64();
}

pub fn spawn_rgb_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    rgb_light: RgbLight,
) {
    commands.spawn(
        (
            Mesh2d(meshes.add(Circle::new(rgb_light.radius))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            transform,
            rgb_light,
        )
    );
}

pub fn update_light_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut effects: ResMut<ActiveEffects>,
    time: Res<SimulationTime>,
    lights_query: Query<(&MeshMaterial2d<ColorMaterial>, &RgbLight, &Transform)>,
) {
    // reset all to zero
    for (material, _, _) in &lights_query {
        let color = &mut materials.get_mut(material).unwrap().color;
        *color = Color::BLACK;
    }

    //update for each effect
    for (start_time, end_time, keyframes, effect) in effects.effects.iter_mut() {
        if time.time < *start_time || time.time > *end_time {
            continue;
        }
        update_effect(effect, keyframes, time.time);
        for (material, rgb_light, transform) in &lights_query {
            let color = &mut materials.get_mut(material).unwrap().color;
            apply_effect(color, &transform.translation, effect, &rgb_light.groups);
        }
    }
}

pub fn update_effect(
    effect: &mut Effect,
    keyframes: &Vec<Keyframe>,
    time: f64,
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
            *orig_color = apply_fill(
                &orig_color.to_linear(),
                &color.to_linear(),
            ).into();
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
            
            *orig_color = apply_shockwave(
                &orig_color,
                &color,
                *location - *center,
                *radius,
                *flat,
                *head,
                *tail,
            ).into();
        }
    }
}

pub fn apply_fill(
    orig_color: &LinearRgba,
    color: &LinearRgba,
) -> LinearRgba {
    LinearRgba::new(
        orig_color.red + color.red,
        orig_color.green + color.green,
        orig_color.blue + color.blue,
        orig_color.alpha + color.alpha,
    )
}

pub fn apply_shockwave(
    orig_color: &Color,
    color: &Color,
    displacement: Vec3,
    radius: f32,
    flat: f32,
    head: f32,
    tail: f32
) -> Color {
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

    let faded_color = color.mix(&Color::BLACK, 1.0 - influence);
    (orig_color.to_linear() + faded_color.to_linear()).into()
}

fn interpolate_float(
    start: f32,
    end: f32,
    time: f64,
    interpolation: InterpolationType,
) -> f32 {
    match interpolation {
        InterpolationType::CONSTANT => {
            start
        }
        InterpolationType::LINEAR => {
            start + (end - start) * time as f32
        }
    }
}

fn interpolate_color(
    start: &Color,
    end: &Color,
    time: f64,
    interpolation: InterpolationType,
) -> Color {
    match interpolation {
        InterpolationType::CONSTANT => {
            start.clone()
        }
        InterpolationType::LINEAR => {
            Color::mix(start, end, time as f32)
        }
    }
}

fn interpolate_vec3(
    start: &Vec3,
    end: &Vec3,
    time: f64,
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
    time: f64,
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
    time: f64,
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
    time: f64,
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
                &start_value,
                &end_value,
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
    time: f64,
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
                &start_value,
                &end_value,
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
