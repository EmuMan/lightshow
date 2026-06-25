use bevy::prelude::*;

use crate::util::blending::{BlendingMode, lerp};

pub fn blend_colors(
    color_1: Color,
    color_2: Color,
    factor: f32,
    blending_mode: BlendingMode,
) -> Color {
    match blending_mode {
        BlendingMode::Mix => mix_colors(color_1, color_2, factor),
        BlendingMode::Add => add_colors(color_1, color_2, factor),
        BlendingMode::Subtract => subtract_colors(color_1, color_2, factor),
        BlendingMode::Multiply => multiply_colors(color_1, color_2, factor),
    }
}

fn mix_colors(color_1: Color, color_2: Color, factor: f32) -> Color {
    let c1: Oklaba = color_1.into();
    let c2: Oklaba = color_2.into();
    Oklaba::new(
        lerp(c1.lightness, c2.lightness, factor),
        lerp(c1.a, c2.a, factor),
        lerp(c1.b, c2.b, factor),
        lerp(c1.alpha, c2.alpha, factor),
    )
    .into()
}

fn add_colors(color_1: Color, color_2: Color, factor: f32) -> Color {
    let c1: LinearRgba = color_1.into();
    let c2: LinearRgba = color_2.into();
    LinearRgba::new(
        (c1.red + c2.red * factor).clamp(0.0, 1.0),
        (c1.green + c2.green * factor).clamp(0.0, 1.0),
        (c1.blue + c2.blue * factor).clamp(0.0, 1.0),
        (c1.alpha + c2.alpha * factor).clamp(0.0, 1.0),
    )
    .into()
}

fn subtract_colors(color_1: Color, color_2: Color, factor: f32) -> Color {
    let c1: LinearRgba = color_1.into();
    let c2: LinearRgba = color_2.into();
    LinearRgba::new(
        (c1.red - c2.red * factor).clamp(0.0, 1.0),
        (c1.green - c2.green * factor).clamp(0.0, 1.0),
        (c1.blue - c2.blue * factor).clamp(0.0, 1.0),
        (c1.alpha - c2.alpha * factor).clamp(0.0, 1.0),
    )
    .into()
}

fn multiply_colors(color_1: Color, color_2: Color, factor: f32) -> Color {
    let c1: LinearRgba = color_1.into();
    let c2: LinearRgba = color_2.into();
    LinearRgba::new(
        lerp(c1.red, c1.red * c2.red, factor),
        lerp(c1.green, c1.green * c2.green, factor),
        lerp(c1.blue, c1.blue * c2.blue, factor),
        lerp(c1.alpha, c1.alpha * c2.alpha, factor),
    )
    .into()
}

/// Interpolates between several color bands from a normalized value
pub fn interpolate_color_bands(color_bands: &[(f32, Color)], value: f32) -> Color {
    let mut band_1_idx: Option<usize> = None;
    let mut band_2_idx: Option<usize> = None;

    for (i, (band, _)) in color_bands.iter().enumerate() {
        if value > *band {
            band_1_idx = Some(i);
        } else {
            band_2_idx = Some(i);
            break;
        }
    }

    match (band_1_idx, band_2_idx) {
        (Some(band_1_idx), Some(band_2_idx)) => {
            let band_1 = color_bands[band_1_idx];
            let band_2 = color_bands[band_2_idx];
            let normalized = (value - band_1.0) / (band_2.0 - band_1.0);
            mix_colors(band_1.1, band_2.1, normalized)
        }
        (Some(band_1_idx), None) => color_bands[band_1_idx].1,
        (None, Some(band_2_idx)) => color_bands[band_2_idx].1,
        (None, None) => Color::NONE,
    }
}
