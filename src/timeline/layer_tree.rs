use crate::{
    simple_store::{SimpleHandle, SimpleStore},
    timeline::{
        effects::{Effect, EffectInfo},
        layers::{Layer, LayerInfo, PrimaryLayer},
        playback::PlaybackInformation,
        tracks::{TrackContents, TrackReference},
    },
};
use bevy::prelude::*;

#[derive(Debug)]
enum LayerTreeError {
    ExistingKeyError(String),
    InvalidPathError(String),
}

#[derive(Debug)]
pub enum ActiveElement {
    // ActiveLayer -> many ActiveTrack nodes
    ActiveLayer {
        local_time: f64,
        original: SimpleHandle<Layer>,
        current_info: LayerInfo,
        children: Vec<ActiveElement>,
    },
    // EffectActiveTrack -> nothing
    EffectActiveTrack {
        local_time: f64,
        original: SimpleHandle<Effect>,
        current_info: EffectInfo,
    },
    // LayerActiveTrack -> potentially one ActiveLayer node
    LayerActiveTrack {
        local_time: f64,
        child: Option<Box<ActiveElement>>,
    },
    // TriggerActiveTrack -> many ActiveLayer nodes
    TriggerActiveTrack {
        // TODO: implement trigger tracks
    },
}

#[derive(Resource, Debug, Default)]
pub struct LayerTree {
    primary_node: Option<ActiveElement>,
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
                // TODO: The tree will start blank and will need to be initialized. This is a note for all mutual recursors below.
                self.primary_node.get_or_insert(ActiveElement::ActiveLayer {
                    local_time: primary_layer_time,
                    original: primary_layer_handle,
                    current_info: primary_layer.info.clone(),
                    children: Vec::new(),
                }),
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
        current_active_element: &mut ActiveElement,
        current_time: f64,
    ) {
        let Some(current_layer) = layer_store.get(current_layer_handle) else {
            panic!("encountered layer that does not exist while updating layer tree");
        };

        match current_active_element {
            ActiveElement::ActiveLayer {
                local_time,
                original,
                current_info,
                children,
            } => {
                *local_time = current_time;
                // TODO: update current info

                for (track_i, track) in current_layer.tracks.iter().enumerate() {
                    let Some(active_child_element) = children.get_mut(track_i) else {
                        panic!("layer and active layer track counts don't match");
                    };
                    match &track.contents {
                        TrackContents::EffectTrack { effect_handle } => {
                            LayerTree::update_recursive_effect_track(
                                layer_store,
                                effect_store,
                                *effect_handle,
                                active_child_element,
                                current_time, // TODO: this is probably right?
                            );
                        }
                        TrackContents::LayerTrack { clips } => {
                            // Not passing clips here causes a double access, but I think
                            // that's okay? It certainly makes the reasoning much easier.
                            LayerTree::update_recursive_layer_track(
                                layer_store,
                                effect_store,
                                TrackReference::new(current_layer_handle, track_i),
                                active_child_element,
                                current_time, // TODO: this is probably right?
                            );
                        }
                        TrackContents::TriggerTrack { layer_handle } => {
                            // TODO: recurse on trigger tracks
                        }
                    }
                }
            }
            _ => panic!("attempted to update an active layer as a different type"),
        }
    }

    fn update_recursive_effect_track(
        layer_store: &SimpleStore<Layer>,
        effect_store: &SimpleStore<Effect>,
        current_effect_handle: SimpleHandle<Effect>,
        current_active_element: &mut ActiveElement,
        current_time: f64,
    ) {
        let Some(current_effect) = effect_store.get(current_effect_handle) else {
            panic!("encountered effect that does not exist while updating layer tree");
        };

        match current_active_element {
            ActiveElement::EffectActiveTrack {
                local_time,
                original,
                current_info,
            } => {
                *local_time = current_time;
                // TODO: update current info
                // No need to recurse!
            }
            _ => panic!("attempted to update an active effect track as a different type"),
        }
    }

    fn update_recursive_layer_track(
        layer_store: &SimpleStore<Layer>,
        effect_store: &SimpleStore<Effect>,
        current_track_reference: TrackReference,
        current_active_element: &mut ActiveElement,
        current_time: f64,
    ) {
        let Some(current_layer) = layer_store.get(current_track_reference.layer) else {
            panic!("encountered track (layer) that does not exist while updating layer tree");
        };
        let Some(current_track) = current_layer.tracks.get(current_track_reference.index) else {
            panic!("encountered track (index) that does not exist while updating layer tree");
        };

        match current_active_element {
            ActiveElement::LayerActiveTrack { local_time, child } => {
                *local_time = current_time;
                // TODO: figure out which clip is current
                // TODO: update child
                // TODO: recurse if child
            }
            _ => panic!("attempted to update an active layer track as a different type"),
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
