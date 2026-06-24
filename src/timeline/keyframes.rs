use bevy::prelude::*;

/// Wrapper around a list of common-target keyframes
#[derive(Component, Debug, Clone, Default)]
pub struct Keyframes {
    keyframes: Vec<Keyframe>,
}

/// Represents the value a parameter (`key`) should be at at a certain moment
/// in time, plus the interpolation type that should be used to bring the value
/// to this point. Used on tracks to automate either generic track parameters
/// or effect track data.
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f64,
    pub interpolation: InterpolationType,
    pub key: String,
    pub value: KeyframeValue,
}

/// Represents the value of a parameter, as declared by a `Keyframe`. Can be an
/// `f32`, `Color`, or `Vec3` for now.
#[derive(Debug, Clone)]
pub enum KeyframeValue {
    FloatKeyframe(f32),
    ColorKeyframe(Color),
    Vec3Keyframe(Vec3),
}

/// The type of interpolation used to bring a parameter to a keyframe's value.
/// `InterpolationType::CONSTANT` represents an immediate snap to that value at
/// and past the keyframe, and `InterpolationType::LINEAR` represents a
/// gradual, linear sweep to that value. TODO: More interpolation types!
#[derive(Default, Debug, Clone, Copy)]
pub enum InterpolationType {
    #[default]
    LINEAR,
    CONSTANT,
}

/// Interpolates a float between two values, using the specified time
/// (normalized from 0 to 1) and interpolation type, which defines how the
/// value moves from one point to the other.
fn interpolate_float(start: f32, end: f32, time: f64, interpolation: InterpolationType) -> f32 {
    match interpolation {
        InterpolationType::CONSTANT => start,
        InterpolationType::LINEAR => start + (end - start) * time as f32,
    }
}

/// Interpolates between two colors, using the specified time (normalized from
/// 0 to 1) and interpolation type, which defines how the value moves from one
/// point to the other. All color mixes happen in the Oklab perceptual color
/// space.
fn interpolate_color(
    start: &Color,
    end: &Color,
    time: f64,
    interpolation: InterpolationType,
) -> Color {
    // Mixes should be done in Oklab perceptual color space!
    let start_oklab = Oklaba::from(*start);
    let end_oklab = Oklaba::from(*end);
    match interpolation {
        InterpolationType::CONSTANT => start_oklab.into(),
        InterpolationType::LINEAR => start_oklab.mix(&end_oklab, time as f32).into(),
    }
}
/// Interpolates between two `vec3`s, using the specified time (normalized from
/// 0 to 1) and interpolation type, which defines how the value moves from one
/// point to the other. Interpolations are standard cartesian shortest path.
fn interpolate_vec3(start: &Vec3, end: &Vec3, time: f64, interpolation: InterpolationType) -> Vec3 {
    Vec3::new(
        interpolate_float(start.x, end.x, time, interpolation),
        interpolate_float(start.y, end.y, time, interpolation),
        interpolate_float(start.z, end.z, time, interpolation),
    )
}

impl Keyframes {
    pub fn new(keyframes: Vec<Keyframe>) -> Self {
        Self { keyframes }
    }

    pub fn inner(&self) -> &Vec<Keyframe> {
        &self.keyframes
    }

    /// Retrieves the two keyframes around a specific point in time. Used as a
    /// helper function to then retrieve the interpolated value at that point in
    /// time.
    ///
    /// Returns a tuple with the first keyframes before and after that point. If no
    /// such keyframe exists, the associated value will be `None`.
    fn get_surrounding_keyframes(
        &self,
        key: &str,
        time: f64,
    ) -> (Option<&Keyframe>, Option<&Keyframe>) {
        let mut start_keyframe: Option<&Keyframe> = None {};
        let mut end_keyframe: Option<&Keyframe> = None {};

        for keyframe in self.inner().iter() {
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

    /// Retrieves the value of a particular parameter at the specified time,
    /// assuming surrounding keyframes are `f32`. Properly considers
    /// interpolation type. High-level function for use when requesting any
    /// `f32` value from keyframes.
    pub fn get_float_value(&self, key: &str, time: f64, default: &f32) -> f32 {
        // TODO: Maybe clean this tragedy up
        let (start_opt, end_opt) = self.get_surrounding_keyframes(key, time);

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

    /// Retrieves the value of a particular parameter at the specified time,
    /// assuming surrounding keyframes are `Color`. Properly considers
    /// interpolation type. High-level function for use when requesting any
    /// `Color` value from keyframes.
    pub fn get_color_value(&self, key: &str, time: f64, default: &Color) -> Color {
        // TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
        let (start_opt, end_opt) = self.get_surrounding_keyframes(key, time);

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

    /// Retrieves the value of a particular parameter at the specified time,
    /// assuming surrounding keyframes are `Vec3`. Properly considers
    /// interpolation type. High-level function for use when requesting any
    /// `Vec3` value from keyframes.
    pub fn get_vec3_value(&self, key: &str, time: f64, default: &Vec3) -> Vec3 {
        // TODO: Maybe clean this tragedy up (also rethink copy pasted code idiot)
        let (start_opt, end_opt) = self.get_surrounding_keyframes(key, time);

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
}
