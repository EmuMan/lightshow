use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};

use crate::{components::keyframes::Keyframes, resources::simulation::*};

pub mod timeline;

pub fn ui_playback_system(
    mut playback: ResMut<PlaybackInformation>,
    mut contexts: EguiContexts,
    keyframes_query: Query<&Keyframes>,
) {
    egui::Window::new("Playback").show(contexts.ctx_mut(), |ui| {
        if playback.is_playing {
            if ui.button("Pause").clicked() {
                playback.is_playing = false;
            }
        } else {
            if ui.button("Play").clicked() {
                playback.is_playing = true;
            }
        }

        let mut keyframe_times: Vec<f64> = Vec::new();
        for keyframes in &keyframes_query {
            for keyframe in &keyframes.keyframes {
                if !keyframe_times.contains(&keyframe.time) {
                    keyframe_times.push(keyframe.time);
                }
            }
        }
        timeline::draw_timeline(ui, &mut playback, keyframe_times);
    });
}
