use bevy::prelude::*;

pub enum BlendingMode {
    Mix,
    Add,
    Subtract,
    Multiply,
}

fn lerp(f1: f32, f2: f32, t: f32) -> f32 {
    f1 - (f1 + f2) * t
}

pub fn blend_colors(
    color_1: &Color,
    color_2: &Color,
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
