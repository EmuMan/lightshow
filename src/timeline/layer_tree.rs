use crate::{
    simple_store::{SimpleHandle, SimpleStore},
    timeline::{
        effects::{Effect, EffectInfo},
        layers::{Layer, LayerInfo, PrimaryLayer},
        playback::PlaybackInformation,
        tracks::{Clip, ClipsExt, TimeSegment, Track, TrackContents},
    },
};
use bevy::prelude::*;

// ActiveLayer -> many ActiveTrack nodes
#[derive(Debug)]
pub struct ActiveLayer {
    local_time: f64,
    original: SimpleHandle<Layer>,
    current_info: LayerInfo,
    children: Vec<ActiveTrack>,
}

impl ActiveLayer {
    fn from_layer(layer: &Layer, layer_handle: SimpleHandle<Layer>) -> Self {
        Self {
            local_time: 0.0,
            original: layer_handle,
            current_info: layer.info.clone(),
            children: Vec::new(),
        }
    }
}

// EffectActiveTrack -> nothing
#[derive(Debug)]
pub struct ActiveEffectTrack {
    local_time: f64,
    original: SimpleHandle<Effect>,
    current_info: EffectInfo,
}

// LayerActiveTrack -> potentially one ActiveLayer node
#[derive(Debug)]
pub struct ActiveLayerTrack {
    local_time: f64,
    child: Option<(TimeSegment, ActiveLayer)>,
}

// TriggerActiveTrack -> many ActiveLayer nodes
#[derive(Debug)]
pub struct ActiveTriggerTrack {
    // TODO: implement trigger tracks
}

#[derive(Debug)]
pub enum ActiveTrack {
    ActiveEffectTrack(ActiveEffectTrack),
    ActiveLayerTrack(ActiveLayerTrack),
    ActiveTriggerTrack(ActiveTriggerTrack),
}

impl From<ActiveEffectTrack> for ActiveTrack {
    fn from(value: ActiveEffectTrack) -> Self {
        Self::ActiveEffectTrack(value)
    }
}

impl From<ActiveLayerTrack> for ActiveTrack {
    fn from(value: ActiveLayerTrack) -> Self {
        Self::ActiveLayerTrack(value)
    }
}

impl From<ActiveTriggerTrack> for ActiveTrack {
    fn from(value: ActiveTriggerTrack) -> Self {
        Self::ActiveTriggerTrack(value)
    }
}

impl ActiveTrack {
    fn from_track(track: &Track, effect_store: &SimpleStore<Effect>) -> Self {
        match &track.contents {
            TrackContents::EffectTrack { effect_handle } => ActiveEffectTrack {
                local_time: 0.0, // will be set later down the line
                original: *effect_handle,
                current_info: effect_store
                    .get(*effect_handle)
                    .expect("attempted to get invalid effect while constructing EffectActiveTrack")
                    .info
                    .clone(),
            }
            .into(),
            TrackContents::LayerTrack { clips } => ActiveLayerTrack {
                local_time: 0.0,
                child: None, // will be set later down the line
            }
            .into(),
            TrackContents::TriggerTrack { layer_handle } => ActiveTriggerTrack {}.into(), // TODO: implement trigger tracks
        }
    }

    fn as_active_effect_track(&mut self) -> &mut ActiveEffectTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(active_effect_track) => active_effect_track,
            ActiveTrack::ActiveLayerTrack(_) => {
                panic!("attempted to unwrap ActiveLayerTrack as ActiveEffectTrack")
            }
            ActiveTrack::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveEffectTrack")
            }
        }
    }

    fn as_active_layer_track(&mut self) -> &mut ActiveLayerTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveLayerTrack")
            }
            ActiveTrack::ActiveLayerTrack(active_layer_track) => active_layer_track,
            ActiveTrack::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveLayerTrack")
            }
        }
    }

    fn as_active_trigger_track(&mut self) -> &mut ActiveTriggerTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveTriggerTrack")
            }
            ActiveTrack::ActiveLayerTrack(_) => {
                panic!("attempted to unwrap ActiveLayerTrack as ActiveTriggerTrack")
            }
            ActiveTrack::ActiveTriggerTrack(active_trigger_track) => active_trigger_track,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct LayerTree {
    primary_node: Option<ActiveLayer>,
}

impl LayerTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.primary_node = None;
    }

    pub fn update_recursive(
        &mut self,
        layer_store: &SimpleStore<Layer>,
        effect_store: &SimpleStore<Effect>,
        primary_layer_handle: SimpleHandle<Layer>,
        primary_layer_time: f64,
    ) {
        match layer_store.get(primary_layer_handle) {
            Some(primary_layer) => LayerTree::update_recursive_layer(
                layer_store,
                effect_store,
                primary_layer_handle,
                // create the primary node if it does not exist
                self.primary_node
                    .get_or_insert(ActiveLayer::from_layer(primary_layer, primary_layer_handle)),
                primary_layer_time,
            ),
            // TODO: should this be a warning?
            None => self.primary_node = None,
        }
    }

    fn update_recursive_layer(
        layer_store: &SimpleStore<Layer>,
        effect_store: &SimpleStore<Effect>,
        current_layer_handle: SimpleHandle<Layer>,
        current_active_layer: &mut ActiveLayer,
        current_time: f64,
    ) {
        let Some(current_layer) = layer_store.get(current_layer_handle) else {
            panic!("encountered layer that does not exist while updating layer tree");
        };

        current_active_layer.local_time = current_time;
        // TODO: update current info

        let needs_initialization = current_active_layer.children.is_empty();

        for (track_i, track) in current_layer.tracks.iter().enumerate() {
            if needs_initialization {
                current_active_layer
                    .children
                    .push(ActiveTrack::from_track(track, effect_store));
            }

            let active_child_element = current_active_layer
                .children
                .get_mut(track_i)
                .expect("layer and active layer track counts don't match");

            match &track.contents {
                TrackContents::EffectTrack { effect_handle } => {
                    LayerTree::update_recursive_effect_track(
                        effect_store,
                        *effect_handle,
                        active_child_element.as_active_effect_track(),
                        current_time,
                    );
                }
                TrackContents::LayerTrack { clips } => {
                    LayerTree::update_recursive_layer_track(
                        layer_store,
                        effect_store,
                        clips,
                        active_child_element.as_active_layer_track(),
                        current_time,
                    );
                }
                TrackContents::TriggerTrack { layer_handle } => {
                    // TODO: recurse on trigger tracks
                }
            }
        }
    }

    fn update_recursive_effect_track(
        effect_store: &SimpleStore<Effect>,
        current_effect_handle: SimpleHandle<Effect>,
        current_active_track: &mut ActiveEffectTrack,
        current_time: f64,
    ) {
        let current_effect = effect_store
            .get(current_effect_handle)
            .expect("encountered effect that does not exist while updating layer tree");

        current_active_track.local_time = current_time;
        // TODO: update current info
        // No need to recurse!
    }

    fn update_recursive_layer_track(
        layer_store: &SimpleStore<Layer>,
        effect_store: &SimpleStore<Effect>,
        clips: &Vec<Clip>,
        current_active_track: &mut ActiveLayerTrack,
        current_time: f64,
    ) {
        current_active_track.local_time = current_time;
        let current_clip = clips.find_current(current_time);
        match current_clip {
            Some(current_clip) => {
                let current_layer = layer_store
                    .get(current_clip.layer)
                    .expect("attempted to get layer that does not exist from active track clip");
                match &mut current_active_track.child {
                    Some((time_segment, active_layer)) => {
                        // the clips are already guaranteed to be on the same
                        // track, so only the time segment needs to be checked
                        if *time_segment != current_clip.time_segment {
                            // there is already a clip but it is the wrong one, so swap it out
                            *active_layer =
                                ActiveLayer::from_layer(current_layer, current_clip.layer);
                        }
                    }
                    None => {
                        // there was no clip, so make a new one
                        current_active_track.child = Some((
                            current_clip.time_segment,
                            ActiveLayer::from_layer(current_layer, current_clip.layer),
                        ));
                    }
                }

                // now that the child has been updated, recurse on it
                let (time_segment, next_active_layer) = current_active_track
                    .child
                    .as_mut()
                    .expect("ActiveLayerTrack child somehow does not exist after creation");
                LayerTree::update_recursive_layer(
                    layer_store,
                    effect_store,
                    current_clip.layer,
                    next_active_layer,
                    current_time - time_segment.start_time + time_segment.start_offset,
                );
            }
            // no further checks needed
            None => current_active_track.child = None,
        }
    }
}

#[derive(Event)]
pub struct ClearLayerTree {}

fn clear_layer_tree(_reset: On<ClearLayerTree>, mut layer_tree: ResMut<LayerTree>) {
    layer_tree.clear();
}

fn update_layer_tree(
    layer_store: Res<SimpleStore<Layer>>,
    effect_store: Res<SimpleStore<Effect>>,
    primary_layer: Res<PrimaryLayer>,
    mut layer_tree: ResMut<LayerTree>,
    playback_info: Res<PlaybackInformation>,
) {
    let Some(primary_layer_handle) = primary_layer.0 else {
        return;
    };

    layer_tree.update_recursive(
        &layer_store,
        &effect_store,
        primary_layer_handle,
        playback_info.current_time,
    );
}
