use std::collections::VecDeque;

use bevy::prelude::*;

use crate::simple_store::{SimpleHandle, SimpleStore};
use crate::timeline::CurrentTime;
use crate::{effects::*, keyframes::*, simulation::*};

pub struct LayersPlugin;

impl Plugin for LayersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Layer>>()
            .init_resource::<PrimaryLayer>()
            .add_systems(Update, update_active_layer_time)
            .add_systems(Update, update_spawned_layers_time);
    }
}

#[derive(Resource, Default, Debug)]
pub struct PrimaryLayer(pub Option<SimpleHandle<Layer>>);

// used to derive current_time for layers that weren't spawned off of
// clips in an existing layer
#[derive(Component, Debug, Clone)]
pub struct GloballyDerivedTime {
    pub creation_time: f64,
    pub offset: f64,
}

#[derive(Component, Debug, Clone)]
pub struct SpawnedLayer {
    pub layer_handle: SimpleHandle<Layer>,
}

#[derive(Default, Debug)]
pub struct Layer {
    pub name: String,
    pub clips: Vec<Clip>,
    pub length: f64,
}

#[derive(Debug)]
pub struct Clip {
    pub track: usize,
    pub instance_ref: ClipInstanceRef,
    pub time_segment: TimeSegment,
}

#[derive(Debug)]
pub enum ClipInstanceRef {
    Effect {
        handle: SimpleHandle<Effect>,
        keyframes: Vec<Keyframe>,
        entity: Option<Entity>,
    },
    Layer {
        handle: SimpleHandle<Layer>,
    },
}

// used when recursively processing layers to determine playback specifics
// for individual layers
#[derive(Default, Debug)]
pub struct TimeSegment {
    pub start_time: f64,
    pub duration: f64,
    pub start_offset: f64,
}

impl TimeSegment {
    pub fn new(start_time: f64, duration: f64, start_offset: f64) -> Self {
        TimeSegment {
            start_time,
            duration,
            start_offset,
        }
    }
}

fn update_active_layer_time(
    mut commands: Commands,
    playback_information: Res<PlaybackInformation>,
    mut layer_store: ResMut<SimpleStore<Layer>>,
    effect_store: Res<SimpleStore<Effect>>,
    primary_layer: Res<PrimaryLayer>,
    mut effects_time_query: Query<&mut CurrentTime, With<Effect>>,
) {
    let Some(primary_layer_handle) = primary_layer.0 else {
        // if there is no primary layer, no need to do anything.
        return;
    };
    if layer_store.get(primary_layer_handle).is_none() {
        // if the primary layer is an invalid reference it should be removed.
        commands.insert_resource(PrimaryLayer(None));
        return;
    }

    update_effect_times_in_layer_recursive(
        &mut commands,
        &mut layer_store,
        &effect_store,
        primary_layer_handle,
        &mut effects_time_query,
        playback_information.current_time,
    );
}

fn update_spawned_layers_time(
    mut commands: Commands,
    mut layer_store: ResMut<SimpleStore<Layer>>,
    effect_store: Res<SimpleStore<Effect>>,
    time: Res<Time>,
    spawned_layer_query: Query<
        (&SpawnedLayer, &GloballyDerivedTime, &mut CurrentTime),
        Without<Effect>,
    >,
    mut effects_time_query: Query<&mut CurrentTime, With<Effect>>,
) {
    for (spawned_layer, globally_derived_time, mut current_time) in spawned_layer_query {
        let layer_handle = spawned_layer.layer_handle;
        current_time.time = time.elapsed_secs_f64() - globally_derived_time.creation_time
            + globally_derived_time.offset;
        update_effect_times_in_layer_recursive(
            &mut commands,
            &mut layer_store,
            &effect_store,
            layer_handle,
            &mut effects_time_query,
            current_time.time,
        );
    }
}

fn update_effect_times_in_layer_recursive(
    commands: &mut Commands,
    layer_store: &mut SimpleStore<Layer>,
    effect_store: &SimpleStore<Effect>,
    primary_layer_handle: SimpleHandle<Layer>,
    effects_time_query: &mut Query<&mut CurrentTime, With<Effect>>,
    time_in_clip: f64,
) {
    let mut layer_queue = VecDeque::new();
    layer_queue.push_back((primary_layer_handle, time_in_clip));
    while !layer_queue.is_empty() {
        let (current_layer_handle, relative_time) = layer_queue
            .pop_front()
            .expect("layer queue was somehow empty");
        let Some(current_layer) = layer_store.get_mut(current_layer_handle) else {
            // TODO: something when a layer is invalid here?
            // This means that a clip refers to a layer that no longer exists and passed
            // it along this BFS.
            continue;
        };
        for clip in &mut current_layer.clips {
            let is_in_clip = clip.time_segment.start_time < relative_time
                && clip.time_segment.start_time + clip.time_segment.duration > relative_time;
            let new_clip_time =
                relative_time + clip.time_segment.start_time + clip.time_segment.start_offset;
            match &mut clip.instance_ref {
                ClipInstanceRef::Effect {
                    handle,
                    entity,
                    keyframes,
                } => {
                    if is_in_clip {
                        if let Some(some_entity) = entity {
                            if let Ok(mut current_time) = effects_time_query.get_mut(*some_entity) {
                                current_time.time = new_clip_time;
                            } else {
                                panic!("effect entity should exist but does not exist");
                            }
                        } else {
                            // the entity does not exist yet and needs to be created
                            println!("Spawning effect entity...");
                            let effect = effect_store.get(*handle).expect("effect handle invalid");
                            let mut entity_commands = commands.spawn((
                                effect.clone(),
                                Keyframes {
                                    keyframes: keyframes.clone(),
                                },
                                CurrentTime::default(),
                            ));
                            effect.init_info.insert_component(&mut entity_commands);
                            *entity = Some(entity_commands.id());
                        };
                    } else {
                        if let Some(some_entity) = entity {
                            println!("Despawning effect entity...");
                            commands.entity(*some_entity).despawn();
                            *entity = None;
                        };
                    }
                }
                ClipInstanceRef::Layer { handle } => {
                    if !is_in_clip {
                        continue;
                    }
                    layer_queue.push_back((*handle, new_clip_time));
                }
            }
        }
    }
}
