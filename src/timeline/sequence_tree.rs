use crate::{
    audio::processing::fft::RecentFftData,
    fixtures::{FixtureRequest, FixtureResponse},
    simple_store::{SimpleHandle, SimpleStore},
    timeline::{
        effects::{ColorEffectLike, EffectInfo, EffectUpdateCommonInfo, PanTiltEffectLike},
        keyframes::Keyframes,
        playback::PlaybackInformation,
        sequences::{PrimarySequence, Sequence},
        tracks::{Clip, ClipsExt, TimeSegment, Track, TrackContents},
    },
    util::blending::BlendingMode,
};
use bevy::prelude::*;
use derive_more::From;

/// Bevy plugin for the sequence tree.
pub struct SequenceTreePlugin;

impl Plugin for SequenceTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SequenceTree>()
            .add_systems(FixedUpdate, update_sequence_tree)
            .add_observer(clear_sequence_tree);
    }
}

/// An `ActiveSequence` represents a currently-playing-back sequence within
/// the tree. Here, it simply points to many `ActiveTrack` children, which
/// handle any actual effects or subsequences.
///
/// `ActiveSequence` -> many `ActiveTrack` nodes.
#[derive(Debug)]
pub struct ActiveSequence {
    local_time: f64,
    children: Vec<ActiveTrack>,
}

impl Default for ActiveSequence {
    /// Creates a new `ActiveSequence`. Information will be brought in during
    /// the next update cycle.
    fn default() -> Self {
        Self {
            local_time: 0.0,
            children: Vec::new(),
        }
    }
}

/// Represents a single active effect track, i.e. an indexed child of an active
/// sequence that is currently being played back and defines a single effect to
/// be rendered. These represent leaves of the sequence tree and contain only
/// effect info. Keyframes are shared globally across all instances of the
/// effect and are therefore passed by reference in the effect update functions
/// to avoid recloning on every modification.
///
/// `ActiveSequenceTrack` -> nothing
#[derive(Debug)]
pub struct ActiveEffectTrack {
    current_info: EffectInfo,
}

/// Represents a single active sequence track, i.e. an indexed child of an
/// active sequence that is currently being played back and defines a series of
/// clips which reference other sequences. Holds a reference to the currently
/// playing clip if the playback head is currently on one, otherwise `None`.
///
/// `ActiveSequenceTrack` -> potentially one `ActiveSequence` node
#[derive(Debug)]
pub struct ActiveSequenceTrack {
    child: Option<(TimeSegment, ActiveSequence)>,
}

// ActiveTriggerTrack -> many ActiveSequence nodes
#[derive(Debug)]
pub struct ActiveTriggerTrack {
    // TODO: implement trigger tracks
}

/// Collection of all active track content types to be included as data for
/// active tracks (indexed children of active sequences in the sequence tree).
#[derive(Debug, From)]
pub enum ActiveTrackContents {
    ActiveEffectTrack(ActiveEffectTrack),
    ActiveSequenceTrack(ActiveSequenceTrack),
    ActiveTriggerTrack(ActiveTriggerTrack),
}

/// Represents a single active track, i.e. an indexed child of an active
/// sequence that is currently being played back. Contains basic track
/// metadata, as well as any variant-specific contents of the track, such as
/// the effect information or children, within `contents`.
#[derive(Debug)]
pub struct ActiveTrack {
    blending_mode: BlendingMode,
    factor: f32,
    local_time: f64,
    contents: ActiveTrackContents,
}

impl From<&Track> for ActiveTrack {
    /// Constructs an `ActiveTrack` from a static track playing back within a
    /// sequence defined in the timeline.
    fn from(value: &Track) -> Self {
        match &value.contents {
            TrackContents::EffectTrack {
                effect_init_info, ..
            } => Self {
                blending_mode: value.info.blending_mode,
                factor: value.info.factor,
                local_time: 0.0, // will be set later down the line
                contents: ActiveEffectTrack {
                    // Since an effect can be instantiated several times, data
                    // has to be cloned each time.
                    current_info: effect_init_info.clone(),
                }
                .into(),
            },
            TrackContents::SequenceTrack { .. } => Self {
                blending_mode: value.info.blending_mode,
                factor: value.info.factor,
                local_time: 0.0,
                contents: ActiveSequenceTrack {
                    child: None, // will be set later down the line
                }
                .into(),
            },
            TrackContents::TriggerTrack { .. } => Self {
                blending_mode: value.info.blending_mode,
                factor: value.info.factor,
                local_time: 0.0,
                // TODO: implement trigger tracks
                contents: ActiveTriggerTrack {}.into(),
            },
        }
    }
}

impl ActiveTrack {
    fn as_active_effect_track(&mut self) -> &mut ActiveEffectTrack {
        match &mut self.contents {
            ActiveTrackContents::ActiveEffectTrack(active_effect_track) => active_effect_track,
            ActiveTrackContents::ActiveSequenceTrack(_) => {
                panic!("attempted to unwrap ActiveSequenceTrack as ActiveEffectTrack")
            }
            ActiveTrackContents::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveEffectTrack")
            }
        }
    }

    fn as_active_sequence_track(&mut self) -> &mut ActiveSequenceTrack {
        match &mut self.contents {
            ActiveTrackContents::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveSequenceTrack")
            }
            ActiveTrackContents::ActiveSequenceTrack(active_sequence_track) => {
                active_sequence_track
            }
            ActiveTrackContents::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveSequenceTrack")
            }
        }
    }

    fn as_active_trigger_track(&mut self) -> &mut ActiveTriggerTrack {
        match &mut self.contents {
            ActiveTrackContents::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveTriggerTrack")
            }
            ActiveTrackContents::ActiveSequenceTrack(_) => {
                panic!("attempted to unwrap ActiveSequenceTrack as ActiveTriggerTrack")
            }
            ActiveTrackContents::ActiveTriggerTrack(active_trigger_track) => active_trigger_track,
        }
    }
}

/// The almighty sequence tree, which drives how the effects and sub-sequences
/// of the current active sequence are built out into the actual, concrete
/// effects being played back and rendered out to the fixtures. If a track,
/// effect, or sequence of any sort is currently a factor in playback, it will
/// be included in this tree in the form of its `Active...` variant.
///
/// Stored as a global Bevy resource.
#[derive(Resource, Debug, Default)]
pub struct SequenceTree {
    primary_node: Option<ActiveSequence>,
}

impl SequenceTree {
    /// Constructs a new, empty `SequenceTree`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears out the sequence tree. By setting the primary node to `None`, a
    /// full regeneration is forced during the next update cycle. Should be
    /// called whenever changes are made to the order or major contents of any
    /// sequence to avoid stale data from causing issues.
    pub fn clear(&mut self) {
        self.primary_node = None;
    }

    /// Updates the sequence store based on the primary sequence open in the
    /// program. Creates and removes branches as necessary, as well as updating
    /// any active effects within the tree. This should be called every upate
    /// cycle to keep the active tree up to date in playback so it can be
    /// accurately sampled by any fixtures.
    pub fn update_recursive(
        &mut self,
        sequence_store: &SimpleStore<Sequence>,
        primary_sequence_handle: SimpleHandle<Sequence>,
        primary_sequence_time: f64,
        common_info: &EffectUpdateCommonInfo,
    ) {
        SequenceTree::update_recursive_sequence(
            sequence_store,
            primary_sequence_handle,
            // create the primary node if it does not exist
            self.primary_node.get_or_insert(ActiveSequence::default()),
            primary_sequence_time,
            common_info,
        )
    }

    /// Helper function for `SequenceTree::update_recursive`. Recursively
    /// updates the sequence subtree and effects within an active sequence.
    fn update_recursive_sequence(
        sequence_store: &SimpleStore<Sequence>,
        current_sequence_handle: SimpleHandle<Sequence>,
        current_active_sequence: &mut ActiveSequence,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    ) {
        let Some(current_sequence) = sequence_store.get(current_sequence_handle) else {
            panic!("encountered sequence that does not exist while updating sequence tree");
        };

        current_active_sequence.local_time = current_time;

        let needs_initialization = current_active_sequence.children.is_empty();

        for (track_i, track) in current_sequence.tracks.iter().enumerate() {
            if needs_initialization {
                current_active_sequence
                    .children
                    .push(ActiveTrack::from(track));
            }

            let active_child_element = current_active_sequence
                .children
                .get_mut(track_i)
                .expect("sequence and active sequence track counts don't match");

            active_child_element.local_time = current_time;
            // TODO: Update factor using keyframes. Probably through a helper within `ActiveTrack`.

            match &track.contents {
                TrackContents::EffectTrack {
                    effect_keyframes, ..
                } => {
                    SequenceTree::update_recursive_effect_track(
                        active_child_element.as_active_effect_track(),
                        current_time,
                        common_info,
                        effect_keyframes,
                    );
                }
                TrackContents::SequenceTrack { clips } => {
                    SequenceTree::update_recursive_sequence_track(
                        sequence_store,
                        clips,
                        active_child_element.as_active_sequence_track(),
                        current_time,
                        common_info,
                    );
                }
                TrackContents::TriggerTrack { sequence_handle } => {
                    // TODO: recurse on trigger tracks
                }
            }
        }
    }

    /// Helper function for `SequenceTree::update_recursive`. Applies updates
    /// to an active effect track within the sequence tree, as dictated by the
    /// specific effect implementation. Does not recurse.
    fn update_recursive_effect_track(
        current_active_track: &mut ActiveEffectTrack,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
        effect_keyframes: &Keyframes,
    ) {
        // Let the effect implementation itself decide how to update.
        current_active_track
            .current_info
            .update(effect_keyframes, current_time, common_info);

        // No need to recurse!
    }

    /// Helper function for `SequenceTree::update_recursive`. Recursively
    /// updates the sequence subtree and effects within an active sequence
    /// track, if the playhead is currently sitting over a clip.
    fn update_recursive_sequence_track(
        sequence_store: &SimpleStore<Sequence>,
        clips: &Vec<Clip>,
        current_active_track: &mut ActiveSequenceTrack,
        current_time: f64,
        common_info: &EffectUpdateCommonInfo,
    ) {
        let current_clip = clips.find_current(current_time);
        match current_clip {
            Some(current_clip) => {
                match &mut current_active_track.child {
                    Some((time_segment, active_sequence)) => {
                        // the clips are already guaranteed to be on the same
                        // track, so only the time segment needs to be checked
                        if *time_segment != current_clip.time_segment {
                            // there is already a clip but it is the wrong one, so swap it out
                            *active_sequence = ActiveSequence::default();
                        }
                    }
                    None => {
                        // there was no clip, so make a new one
                        current_active_track.child =
                            Some((current_clip.time_segment, ActiveSequence::default()));
                    }
                }

                // now that the child has been updated, recurse on it
                let (time_segment, next_active_sequence) = current_active_track
                    .child
                    .as_mut()
                    .expect("ActiveSequenceTrack child somehow does not exist after creation");
                SequenceTree::update_recursive_sequence(
                    sequence_store,
                    current_clip.sequence_handle,
                    next_active_sequence,
                    current_time - time_segment.start_time + time_segment.start_offset,
                    common_info,
                );
            }
            // no further checks needed
            None => current_active_track.child = None,
        }
    }

    /// Recursively retreives a set of values for fixtures. Batches the whole
    /// set together for performance improvement. Does not ask for unrelated
    /// information if a fixture does not need it. Assumes that the sequence
    /// tree is up to date (`SequenceTree::update_recursive` has been called).
    /// The number of responses returned will equal the number of requests
    /// passed in.
    pub fn get_values_recursive(&self, fixtures: &[FixtureRequest]) -> Vec<FixtureResponse> {
        // can be ignored if there is no set primary node
        if let Some(primary_node) = &self.primary_node {
            SequenceTree::get_values_recursive_sequence(primary_node, fixtures)
        } else {
            fixtures
                .iter()
                .map(FixtureRequest::default_response)
                .collect()
        }
    }

    /// Helper function for `SequenceTree::get_values_recursive` that recurses
    /// over all tracks within a sequence, collects the data they provide for
    /// each request, and merges them together according to factor and blend
    /// mode.
    fn get_values_recursive_sequence(
        current_active_sequence: &ActiveSequence,
        fixtures: &[FixtureRequest],
    ) -> Vec<FixtureResponse> {
        let mut final_inputs: Vec<FixtureResponse> = fixtures
            .iter()
            .map(FixtureRequest::default_response)
            .collect();

        for active_track in &current_active_sequence.children {
            let new_values = match &active_track.contents {
                ActiveTrackContents::ActiveEffectTrack(active_effect_track) => {
                    SequenceTree::get_values_recursive_effect_track(active_effect_track, fixtures)
                }
                ActiveTrackContents::ActiveSequenceTrack(active_sequence_track) => {
                    SequenceTree::get_values_recursive_sequence_track(
                        active_sequence_track,
                        fixtures,
                    )
                }
                ActiveTrackContents::ActiveTriggerTrack(active_trigger_track) => {
                    // TODO: implement trigger tracks
                    fixtures
                        .iter()
                        .map(FixtureRequest::default_response)
                        .collect()
                }
            };
            for (existing_val, new_val) in final_inputs.iter_mut().zip(new_values.iter()) {
                existing_val.merge_in_place(
                    &new_val,
                    active_track.factor,
                    active_track.blending_mode,
                );
            }
        }

        final_inputs
    }

    /// Helper function for `SequenceTree::get_values_recursive` that retrieves
    /// the values for a set of requests from an effect track, based on what
    /// information the individual requests need.
    fn get_values_recursive_effect_track(
        current_active_track: &ActiveEffectTrack,
        fixtures: &[FixtureRequest],
    ) -> Vec<FixtureResponse> {
        let mut final_inputs: Vec<FixtureResponse> = Vec::new();

        match &current_active_track.current_info {
            EffectInfo::ColorEffectInfo(color_effect) => {
                for fixture in fixtures {
                    if fixture.has_color {
                        let output = color_effect.get_value(fixture.position);
                        final_inputs.push(FixtureResponse::color_only(output));
                    } else {
                        final_inputs.push(FixtureResponse::default())
                    }
                }
            }
            EffectInfo::PanTiltEffectInfo(pan_tilt_effect) => {
                for fixture in fixtures {
                    if fixture.has_pan_tilt {
                        let output = pan_tilt_effect.get_value(fixture.position);
                        final_inputs.push(FixtureResponse::pan_tilt_only(output));
                    } else {
                        final_inputs.push(FixtureResponse::default())
                    }
                }
            }
        }

        final_inputs
        // No need to recurse!
    }

    /// Helper function for `SequenceTree::get_values_recursive` that retrieves
    /// the value from inside a sequence track if there is currently an active
    /// clip playing inside of it, default otherwise.
    fn get_values_recursive_sequence_track(
        current_active_track: &ActiveSequenceTrack,
        fixtures: &[FixtureRequest],
    ) -> Vec<FixtureResponse> {
        match &current_active_track.child {
            Some((_time_segment, active_sequence)) => {
                SequenceTree::get_values_recursive_sequence(active_sequence, fixtures)
            }
            None => fixtures
                .iter()
                .map(FixtureRequest::default_response)
                .collect(),
        }
    }
}

/// Bevy event that clears the current sequence tree.
#[derive(Event)]
pub struct ClearSequenceTree {}

/// Bevy observer that listens for `ClearSequenceTree` events and clears the
/// sequence tree upon receiving one.
fn clear_sequence_tree(_reset: On<ClearSequenceTree>, mut sequence_tree: ResMut<SequenceTree>) {
    sequence_tree.clear();
}

/// Bevy system that updates the sequence tree, keeping both structure and
/// effect values up to date. See `SequenceTree::update_recursive` for more
/// information; this is basically just a wrapper around that.
fn update_sequence_tree(
    time: Res<Time>,
    sequence_store: Res<SimpleStore<Sequence>>,
    primary_sequence: Res<PrimarySequence>,
    mut sequence_tree: ResMut<SequenceTree>,
    playback_info: Res<PlaybackInformation>,
    recent_fft_data: Res<RecentFftData>,
) {
    let Some(primary_sequence_handle) = primary_sequence.0 else {
        return;
    };

    let common_info = EffectUpdateCommonInfo {
        recent_fft_data: &recent_fft_data,
        global_time: time.elapsed_secs_f64(),
    };

    sequence_tree.update_recursive(
        &sequence_store,
        primary_sequence_handle,
        playback_info.current_time,
        &common_info,
    );
}
