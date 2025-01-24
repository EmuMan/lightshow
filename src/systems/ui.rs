use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};

use crate::{components::{keyframes::Keyframes, layers::Layer}, resources::simulation::*};

pub mod timeline;

pub fn ui_playback_system(
    mut playback: ResMut<PlaybackInformation>,
    mut contexts: EguiContexts,
    keyframes_query: Query<&Keyframes>,
    layers_query: Query<&Layer>,
) {
    let active_layer = playback.current_layer
        .and_then(|layer| layers_query.get(layer).ok());
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
        let active_keyframes: Option<Vec<&Keyframes>> = active_layer
            .map(|layer| layer.effects.iter().filter_map(|entity| {
                keyframes_query.get(*entity).ok()
            }).collect());
        
        if let Some(active_keyframes) = active_keyframes {
            for keyframes in &active_keyframes {
                for keyframe in &keyframes.keyframes {
                    if !keyframe_times.contains(&keyframe.time) {
                        keyframe_times.push(keyframe.time);
                    }
                }
            }
        }

        timeline::draw_timeline(ui, &mut playback, keyframe_times, active_layer);
    });
}
