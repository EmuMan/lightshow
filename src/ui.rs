use bevy::prelude::*;

use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    simple_store::SimpleStore,
    timeline::{playback::*, sequences::*},
};

pub mod timeline;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, ui_playback_system);
    }
}

pub fn ui_playback_system(
    mut playback: ResMut<PlaybackInformation>,
    primary_sequence: Res<PrimarySequence>,
    sequence_store: Res<SimpleStore<Sequence>>,
    mut contexts: EguiContexts,
) {
    match contexts.ctx_mut() {
        Ok(contexts) => {
            egui::Window::new("Playback").show(contexts, |ui| {
                if playback.is_playing {
                    if ui.button("Pause").clicked() {
                        playback.is_playing = false;
                    }
                } else {
                    if ui.button("Play").clicked() {
                        playback.is_playing = true;
                    }
                }

                let keyframe_times: Vec<f64> = Vec::new();
                // TODO: Add keyframe times back?

                let primary_sequence = primary_sequence
                    .0
                    .and_then(|handle| sequence_store.get(handle));

                timeline::draw_timeline(ui, &mut playback, keyframe_times, primary_sequence);
            });
        }
        Err(error) => println!("Error: Could not get egui context:\n{}", error),
    }
}
