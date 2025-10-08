use bevy_egui::egui::{self, Color32, Sense, Stroke, Ui, Vec2};

use crate::timeline::{layers::*, playback::*};

pub fn draw_timeline(
    ui: &mut Ui,
    playback: &mut PlaybackInformation,
    keyframe_positions: Vec<f64>,
    active_layer: Option<&Layer>,
) -> egui::Response {
    let (response, painter) = ui.allocate_painter(
        Vec2::new(ui.available_width(), 50.0),
        Sense::click_and_drag(),
    );

    // background
    painter.rect_filled(response.rect, 0.0, Color32::from_black_alpha(128));

    let Some(active_layer) = active_layer else {
        return response;
    };

    if response.clicked() || response.dragged() {
        let click_pos = response.interact_pointer_pos();
        if let Some(pos) = click_pos {
            let local_pos = pos - response.rect.min;
            let percentage = local_pos.x / response.rect.width();
            playback.current_time = active_layer.length * percentage as f64;
        }
    }

    let playhead_pos = response.rect.min
        + Vec2::new(
            (playback.current_time / active_layer.length) as f32 * response.rect.width(),
            0.0,
        );

    // bar lines
    let num_bars = (active_layer.length * playback.bpm / 60.0).ceil() as usize;
    for i in 0..num_bars {
        let x = i as f32 * response.rect.width() / num_bars as f32;
        let bar_pos = response.rect.min + Vec2::new(x, 0.0);
        painter.line_segment(
            [bar_pos, bar_pos + Vec2::Y * response.rect.height()],
            Stroke::new(1.0, Color32::from_white_alpha(64)),
        );
    }

    // beat lines
    let num_beats = num_bars * playback.beats_per_bar;
    for i in 0..num_beats {
        let x = i as f32 * response.rect.width() / num_beats as f32;
        let beat_pos = response.rect.min + Vec2::new(x, 0.0);
        painter.line_segment(
            [beat_pos, beat_pos + Vec2::Y * response.rect.height()],
            Stroke::new(1.0, Color32::from_white_alpha(32)),
        );
    }

    // keyframes
    for pos in keyframe_positions {
        let x = pos as f32 / active_layer.length as f32 * response.rect.width();
        let keyframe_pos = response.rect.min + Vec2::new(x, 0.0);
        painter.line_segment(
            [
                keyframe_pos,
                keyframe_pos + Vec2::Y * response.rect.height(),
            ],
            Stroke::new(2.0, Color32::from_rgb(127, 127, 0)),
        );
    }

    // playhead
    painter.line_segment(
        [
            playhead_pos,
            playhead_pos + Vec2::Y * response.rect.height(),
        ],
        Stroke::new(1.0, Color32::WHITE),
    );

    response
}
