use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum ColorBlendingMode {
    #[default]
    Add,
    Mix,
    Subtract,
    Multiply,
}

fn lerp(f1: f32, f2: f32, t: f32) -> f32 {
    f1 + (f2 - f1) * t
}

pub fn blend_colors(
    color_1: &Color,
    color_2: &Color,
    factor: f32,
    blending_mode: ColorBlendingMode,
) -> Color {
    match blending_mode {
        ColorBlendingMode::Mix => mix_colors(color_1, color_2, factor),
        ColorBlendingMode::Add => add_colors(color_1, color_2, factor),
        ColorBlendingMode::Subtract => subtract_colors(color_1, color_2, factor),
        ColorBlendingMode::Multiply => multiply_colors(color_1, color_2, factor),
    }
}

fn mix_colors(color_1: &Color, color_2: &Color, factor: f32) -> Color {
    let color_1_srgba = color_1.to_srgba();
    let color_2_srgba = color_2.to_srgba();
    LinearRgba::new(
        lerp(color_1_srgba.red, color_2_srgba.red, factor),
        lerp(color_1_srgba.green, color_2_srgba.green, factor),
        lerp(color_1_srgba.blue, color_2_srgba.blue, factor),
        lerp(color_1_srgba.alpha, color_2_srgba.alpha, factor),
    )
    .into()
}

fn add_colors(color_1: &Color, color_2: &Color, factor: f32) -> Color {
    let color_1_srgba = color_1.to_srgba();
    let color_2_srgba = color_2.to_srgba();
    LinearRgba::new(
        color_1_srgba.red + color_2_srgba.red * factor,
        color_1_srgba.green + color_2_srgba.green * factor,
        color_1_srgba.blue + color_2_srgba.blue * factor,
        color_1_srgba.alpha + color_2_srgba.alpha * factor,
    )
    .into()
}

fn subtract_colors(color_1: &Color, color_2: &Color, factor: f32) -> Color {
    let color_1_srgba = color_1.to_srgba();
    let color_2_srgba = color_2.to_srgba();
    // TODO: find good way to clamp (also in add probably)
    LinearRgba::new(
        color_1_srgba.red - color_2_srgba.red * factor,
        color_1_srgba.green - color_2_srgba.green * factor,
        color_1_srgba.blue - color_2_srgba.blue * factor,
        color_1_srgba.alpha - color_2_srgba.alpha * factor,
    )
    .into()
}

fn multiply_colors(color_1: &Color, color_2: &Color, factor: f32) -> Color {
    let color_1_srgba = color_1.to_srgba();
    let color_2_srgba = color_2.to_srgba();
    // TODO: find good way to clamp (also in add probably)
    LinearRgba::new(
        lerp(
            color_1_srgba.red,
            color_1_srgba.red * color_2_srgba.red,
            factor,
        ),
        lerp(
            color_1_srgba.green,
            color_1_srgba.green * color_2_srgba.green,
            factor,
        ),
        lerp(
            color_1_srgba.blue,
            color_1_srgba.blue * color_2_srgba.blue,
            factor,
        ),
        lerp(
            color_1_srgba.alpha,
            color_1_srgba.alpha * color_2_srgba.alpha,
            factor,
        ),
    )
    .into()
}

/// Interpolates between several color bands from a normalized value
pub fn interpolate_color_bands(color_bands: &[(f32, Color)], value: f32) -> Color {
    let mut band_1_idx: Option<usize> = None;
    let mut band_2_idx: Option<usize> = None;

    for (i, (band, _)) in color_bands.iter().enumerate() {
        if value > *band {
            // TODO: clone every time... sad...
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
            mix_colors(&band_1.1, &band_2.1, normalized)
        }
        (Some(band_1_idx), None) => color_bands[band_1_idx].1.clone(),
        (None, Some(band_2_idx)) => color_bands[band_2_idx].1.clone(),
        (None, None) => Color::NONE,
    }
}
