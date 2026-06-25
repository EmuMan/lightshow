use crate::{
    fixtures::PanTilt,
    util::blending::{BlendingMode, lerp},
};

pub fn blend_pan_tilt(
    pt1: PanTilt,
    pt2: PanTilt,
    factor: f32,
    blending_mode: BlendingMode,
) -> PanTilt {
    match blending_mode {
        BlendingMode::Mix => mix_pan_tilt(pt1, pt2, factor),
        BlendingMode::Add => add_pan_tilt(pt1, pt2, factor),
        BlendingMode::Subtract => subtract_pan_tilt(pt1, pt2, factor),
        BlendingMode::Multiply => multiply_pan_tilt(pt1, pt2, factor),
    }
}

fn mix_pan_tilt(pt1: PanTilt, pt2: PanTilt, factor: f32) -> PanTilt {
    PanTilt::new(
        lerp(pt1.pan, pt2.pan, factor),
        lerp(pt1.tilt, pt2.tilt, factor),
    )
}

fn add_pan_tilt(pt1: PanTilt, pt2: PanTilt, factor: f32) -> PanTilt {
    PanTilt::new(
        pt1.pan + pt2.pan * factor,
        pt1.tilt + pt2.tilt * factor,
    )
}

fn subtract_pan_tilt(pt1: PanTilt, pt2: PanTilt, factor: f32) -> PanTilt {
    PanTilt::new(
        pt1.pan - pt2.pan * factor,
        pt1.tilt - pt2.tilt * factor,
    )
}

fn multiply_pan_tilt(pt1: PanTilt, pt2: PanTilt, factor: f32) -> PanTilt {
    PanTilt::new(
        lerp(pt1.pan, pt1.pan * pt2.pan, factor),
        lerp(pt1.tilt, pt1.tilt * pt2.tilt, factor),
    )
}
