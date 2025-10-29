use bevy::prelude::*;

use crate::{
    simple_store::SimpleHandle,
    timeline::{effects::Effect, sequences::Sequence},
    util::blending::colors::ColorBlendingMode,
};

#[derive(Debug)]
pub struct Track {
    pub info: TrackInfo,
    pub contents: TrackContents,
}

#[derive(Debug)]
pub struct TrackInfo {
    pub color_blending_mode: ColorBlendingMode,
    pub opacity: f64,
}

#[derive(Debug)]
pub enum TrackContents {
    EffectTrack {
        effect_handle: SimpleHandle<Effect>,
    },
    SequenceTrack {
        clips: Vec<Clip>,
    },
    TriggerTrack {
        sequence_handle: SimpleHandle<Sequence>,
    },
}

#[derive(Debug)]
pub struct Clip {
    pub sequence_handle: SimpleHandle<Sequence>,
    pub time_segment: TimeSegment,
}

pub trait ClipsExt {
    fn find_current(&self, time: f64) -> Option<&Clip>;
}

impl ClipsExt for [Clip] {
    fn find_current(&self, time: f64) -> Option<&Clip> {
        for clip in self {
            if clip.time_segment.start_time <= time
                && (clip.time_segment.start_time + clip.time_segment.duration) > time
            {
                return Some(clip);
            }
        }
        None
    }
}

// used when recursively processing sequences to determine playback specifics
// for individual sequences
#[derive(Default, Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug)]
pub struct TrackReference {
    pub sequence: SimpleHandle<Sequence>,
    pub index: usize,
}

impl TrackReference {
    pub fn new(sequence_handle: SimpleHandle<Sequence>, track_index: usize) -> Self {
        Self {
            sequence: sequence_handle,
            index: track_index,
        }
    }
}
