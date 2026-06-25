use bevy::prelude::*;

use crate::{
    simple_store::SimpleHandle,
    timeline::{effects::EffectInfo, keyframes::Keyframes, sequences::Sequence},
    util::blending::BlendingMode,
};

/// Represents a single track on the timeline. Each track contains generic
/// info, such as how it should be blended with others, as well as what
/// contents the track holds. When tracks are played back, they will be
/// instantiated as `ActiveTrack`s in the sequence tree hierarchy.
///
/// Tracks can be one of three different types, depending on the variant of
/// `contents`. See `TrackContents` docs for more information.
#[derive(Debug)]
pub struct Track {
    pub info: TrackInfo,
    pub contents: TrackContents,
}

/// Holds generic information about a track. Currently contains blending
/// mode, factor, and keyframes that modify any supported track values.
#[derive(Debug)]
pub struct TrackInfo {
    pub blending_mode: BlendingMode,
    pub factor: f32,
    pub track_keyframes: Keyframes,
}

/// Tracks can be one of three different types, depending on the variant of
/// `TrackContents`:
///
/// - `EffectTrack`: Contains an effect that plays indefinitely. The contents
/// will store a reference to this effect, which will be used to instantiate
/// and initialize an `ActiveEffectTrack`.
///
/// - `SequenceTrack`: Contains a series of `Clip`s that allow for sequences to
/// be nested within each other. During playback, the current `Clip` will be
/// selected, and the corresponding sequence will be instantiated as an
/// `ActiveSequence` linked back to its corresponding `ActiveSequenceTrack`.
///
/// - `TriggerTrack`: Unimplemented. Will allow for sequences to be spawned in
/// on user input.
#[derive(Debug)]
pub enum TrackContents {
    /// Contains an effect that plays indefinitely. The contents
    /// will store a reference to this effect, which will be used to instantiate
    /// and initialize an `ActiveEffectTrack`.
    EffectTrack {
        effect_init_info: EffectInfo,
        effect_keyframes: Keyframes,
    },
    /// Contains a series of `Clip`s that allow for sequences to
    /// be nested within each other. During playback, the current `Clip` will be
    /// selected, and the corresponding sequence will be instantiated as an
    /// `ActiveSequence` linked back to its corresponding `ActiveSequenceTrack`.
    SequenceTrack { clips: Vec<Clip> },
    /// Unimplemented. Will allow for sequences to be spawned in on user input.
    TriggerTrack {
        sequence_handle: SimpleHandle<Sequence>,
    },
}

/// Represents a small window on the timeline of a sequence. The `TimeSegment`
/// allows for arbitrary start times, start offsets, and durations. If the
/// playback head falls within a clip's span, it will instantiate the
/// corresponding `Sequence` as an `ActiveSequence`.
#[derive(Debug)]
pub struct Clip {
    pub sequence_handle: SimpleHandle<Sequence>,
    pub time_segment: TimeSegment,
}

/// Helper trait to search a list of `Clip`s for the currently playing one.
pub trait ClipsExt {
    fn find_current(&self, time: f64) -> Option<&Clip>;
}

impl ClipsExt for [Clip] {
    /// Helper function to search a list of `Clip`s for the currently playing
    /// one. Simple linear scan; I can't imagine this becomes a significant
    /// bottleneck.
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

/// Represents a window of time that a sequence is played within a
/// sequence track. `start_time` represents the start time within that track,
/// and `duration` represents the length of time for which it will play.
/// `start_offset` represents where in the sequence the clip will start
/// playback from, in seconds from the start.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct TimeSegment {
    pub start_time: f64,
    pub duration: f64,
    pub start_offset: f64,
}

impl TimeSegment {
    /// Constructs a new `TimeSegment`.
    pub fn new(start_time: f64, duration: f64, start_offset: f64) -> Self {
        TimeSegment {
            start_time,
            duration,
            start_offset,
        }
    }
}

/// Single datastructure to refer to a track. Holds a handle to the `Sequence`
/// the track resides in, as well as an index to indicate which track within
/// that sequence it is. Should be invalidated whenever the static sequence
/// tree changes.
#[derive(Debug)]
pub struct TrackReference {
    pub sequence: SimpleHandle<Sequence>,
    pub index: usize,
}

impl TrackReference {
    /// Constructs a new `TrackReference`.
    pub fn new(sequence_handle: SimpleHandle<Sequence>, track_index: usize) -> Self {
        Self {
            sequence: sequence_handle,
            index: track_index,
        }
    }
}
