use bevy::prelude::*;

#[derive(Component)]
pub struct Keyframes {
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug)]
pub struct Keyframe {
    pub time: f64,
    pub interpolation: InterpolationType,
    pub key: String,
    pub value: KeyframeValue,
}

#[derive(Debug)]
pub enum KeyframeValue {
    FloatKeyframe(f32),
    ColorKeyframe(Color),
    Vec3Keyframe(Vec3),
}

#[derive(Default, Debug, Clone, Copy)]
pub enum InterpolationType {
    #[default]
    LINEAR,
    CONSTANT,
}

fn interpolate_float(start: f32, end: f32, time: f64, interpolation: InterpolationType) -> f32 {
    match interpolation {
        InterpolationType::CONSTANT => start,
        InterpolationType::LINEAR => start + (end - start) * time as f32,
    }
}

fn interpolate_color(
    start: &Color,
    end: &Color,
    time: f64,
    interpolation: InterpolationType,
) -> Color {
    match interpolation {
        InterpolationType::CONSTANT => start.clone(),
        InterpolationType::LINEAR => Color::mix(start, end, time as f32),
    }
}

fn interpolate_vec3(start: &Vec3, end: &Vec3, time: f64, interpolation: InterpolationType) -> Vec3 {
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
pub fn get_float_value(keyframes: &Vec<Keyframe>, key: &str, time: f64, default: &f32) -> f32 {
    let (start_opt, end_opt) = get_surrounding_keyframes(keyframes, key, time);

    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::FloatKeyframe(start_value) = start_keyframe.value else {
            panic!(
                "tried to interpolate float from non-float start keyframe with key {}",
                key.to_string()
            );
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::FloatKeyframe(end_value) = end_keyframe.value else {
                panic!(
                    "tried to interpolate float from non-float end keyframe with key {}",
                    key.to_string()
                );
            };
            // both start and end
            return interpolate_float(
                start_value,
                end_value,
                (time - start_keyframe.time) / (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::FloatKeyframe(end_value) = end_keyframe.value else {
            panic!(
                "tried to interpolate float from non-float end keyframe with key {}",
                key.to_string()
            );
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}

// TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
pub fn get_color_value(keyframes: &Vec<Keyframe>, key: &str, time: f64, default: &Color) -> Color {
    let (start_opt, end_opt) = get_surrounding_keyframes(keyframes, key, time);

    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::ColorKeyframe(start_value) = start_keyframe.value else {
            panic!(
                "tried to interpolate color from non-float start keyframe with key {}",
                key.to_string()
            );
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::ColorKeyframe(end_value) = end_keyframe.value else {
                panic!(
                    "tried to interpolate color from non-float end keyframe with key {}",
                    key.to_string()
                );
            };
            // both start and end
            return interpolate_color(
                &start_value,
                &end_value,
                (time - start_keyframe.time) / (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::ColorKeyframe(end_value) = end_keyframe.value else {
            panic!(
                "tried to interpolate color from non-float end keyframe with key {}",
                key.to_string()
            );
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}

// TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
pub fn get_vec3_value(keyframes: &Vec<Keyframe>, key: &str, time: f64, default: &Vec3) -> Vec3 {
    let (start_opt, end_opt) = get_surrounding_keyframes(keyframes, key, time);

    if let Some(start_keyframe) = start_opt {
        let KeyframeValue::Vec3Keyframe(start_value) = start_keyframe.value else {
            panic!(
                "tried to interpolate vec3 from non-float start keyframe with key {}",
                key.to_string()
            );
        };
        if let Some(end_keyframe) = end_opt {
            let KeyframeValue::Vec3Keyframe(end_value) = end_keyframe.value else {
                panic!(
                    "tried to interpolate vec3 from non-float end keyframe with key {}",
                    key.to_string()
                );
            };
            // both start and end
            return interpolate_vec3(
                &start_value,
                &end_value,
                (time - start_keyframe.time) / (end_keyframe.time - start_keyframe.time),
                end_keyframe.interpolation,
            );
        }
        // start not end
        return start_value;
    }
    if let Some(end_keyframe) = end_opt {
        let KeyframeValue::Vec3Keyframe(end_value) = end_keyframe.value else {
            panic!(
                "tried to interpolate vec3 from non-float end keyframe with key {}",
                key.to_string()
            );
        };
        // end not start
        return end_value;
    }
    // neither
    *default
}
