use bevy::prelude::*;

use crate::{
    simple_store::SimpleHandle,
    timeline::{
        effects::Effect,
        keyframes::Keyframe,
        layers::{ActiveLayer, Layer, LayerInfo},
    },
};

#[derive(Debug)]
pub struct Track {
    pub keyframes: Vec<Keyframe>,
    pub info: TrackInfo,
    pub contents: TrackContents,
}

#[derive(Debug)]
pub struct TrackInfo {
    pub blending_mode: BlendingMode,
    pub opacity: f64,
}

#[derive(Debug)]
pub enum TrackContents {
    EffectTrack { effect: SimpleHandle<Effect> },
    LayerTrack { clips: Vec<Clip> },
    TriggerTrack { layer_handle: SimpleHandle<Layer> },
}

#[derive(Debug, Component)]
pub struct ActiveTrack {
    pub original: TrackReference,
    pub current_info: TrackInfo,
}

#[derive(Debug, Component)]
pub enum ActiveTrackContents {
    ActiveEffectTrack { active_effect_entity: Entity },
    ActiveLayerTrack { active_layer_entity: Entity },
    ActiveTriggerTrack { past_triggers: Vec<PastTrigger> },
}

#[derive(Debug)]
pub struct PastTrigger {
    pub trigger_global_time: f64,
    pub expiration_global_time: f64,
    pub created_layer: SimpleHandle<ActiveLayer>,
}

#[derive(Debug)]
pub struct Clip {
    pub layer: SimpleHandle<LayerInfo>,
    pub time_segment: TimeSegment,
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

#[derive(Default, Debug)]
pub enum BlendingMode {
    #[default]
    ADD,
    SUBTRACT,
    MULTIPLY,
}

#[derive(Debug)]
pub struct TrackReference {
    pub layer: SimpleHandle<Layer>,
    pub index: u64,
}
