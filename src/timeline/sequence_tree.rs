use crate::{
    fixtures::{FixtureInput, FixtureInputVec, FixtureType},
    simple_store::{SimpleHandle, SimpleStore},
    timeline::{
        effects::{ColorEffectLike, Effect, EffectInfo},
        playback::PlaybackInformation,
        sequences::{PrimarySequence, Sequence},
        tracks::{Clip, ClipsExt, TimeSegment, Track, TrackContents},
    },
    util::blending::colors::ColorBlendingMode,
};
use bevy::prelude::*;
use derive_more::From;

pub struct SequenceTreePlugin;

impl Plugin for SequenceTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SequenceTree>()
            .add_systems(FixedUpdate, update_sequence_tree)
            .add_observer(clear_sequence_tree);
    }
}

// ActiveSequence -> many ActiveTrack nodes
#[derive(Debug)]
pub struct ActiveSequence {
    local_time: f64,
    original: SimpleHandle<Sequence>,
    children: Vec<ActiveTrack>,
}

impl ActiveSequence {
    fn from_sequence(sequence_handle: SimpleHandle<Sequence>) -> Self {
        Self {
            local_time: 0.0,
            original: sequence_handle,
            children: Vec::new(),
        }
    }
}

// ActiveSequenceTrack -> nothing
#[derive(Debug)]
pub struct ActiveEffectTrack {
    color_blending_mode: ColorBlendingMode,
    factor: f32,
    local_time: f64,
    original: SimpleHandle<Effect>,
    current_info: EffectInfo,
}

// ActiveSequenceTrack -> potentially one ActiveSequence node
#[derive(Debug)]
pub struct ActiveSequenceTrack {
    color_blending_mode: ColorBlendingMode,
    factor: f32,
    local_time: f64,
    child: Option<(TimeSegment, ActiveSequence)>,
}

// ActiveTriggerTrack -> many ActiveSequence nodes
#[derive(Debug)]
pub struct ActiveTriggerTrack {
    // TODO: implement trigger tracks
}

#[derive(Debug, From)]
pub enum ActiveTrack {
    ActiveEffectTrack(ActiveEffectTrack),
    ActiveSequenceTrack(ActiveSequenceTrack),
    ActiveTriggerTrack(ActiveTriggerTrack),
}

impl ActiveTrack {
    fn from_track(track: &Track, effect_store: &SimpleStore<Effect>) -> Self {
        match &track.contents {
            TrackContents::EffectTrack { effect_handle } => ActiveEffectTrack {
                color_blending_mode: track.info.color_blending_mode,
                // TODO: align types
                factor: track.info.opacity as f32,
                local_time: 0.0, // will be set later down the line
                original: *effect_handle,
                current_info: effect_store
                    .get(*effect_handle)
                    .expect("attempted to get invalid effect while constructing EffectActiveTrack")
                    .info
                    .clone(),
            }
            .into(),
            TrackContents::SequenceTrack { clips } => ActiveSequenceTrack {
                color_blending_mode: track.info.color_blending_mode,
                // TODO: align types
                factor: track.info.opacity as f32,
                local_time: 0.0,
                child: None, // will be set later down the line
            }
            .into(),
            TrackContents::TriggerTrack { sequence_handle } => ActiveTriggerTrack {}.into(), // TODO: implement trigger tracks
        }
    }

    fn as_active_effect_track(&mut self) -> &mut ActiveEffectTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(active_effect_track) => active_effect_track,
            ActiveTrack::ActiveSequenceTrack(_) => {
                panic!("attempted to unwrap ActiveSequenceTrack as ActiveEffectTrack")
            }
            ActiveTrack::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveEffectTrack")
            }
        }
    }

    fn as_active_sequence_track(&mut self) -> &mut ActiveSequenceTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveSequenceTrack")
            }
            ActiveTrack::ActiveSequenceTrack(active_sequence_track) => active_sequence_track,
            ActiveTrack::ActiveTriggerTrack(_) => {
                panic!("attempted to unwrap ActiveTriggerTrack as ActiveSequenceTrack")
            }
        }
    }

    fn as_active_trigger_track(&mut self) -> &mut ActiveTriggerTrack {
        match self {
            ActiveTrack::ActiveEffectTrack(_) => {
                panic!("attempted to unwrap ActiveEffectTrack as ActiveTriggerTrack")
            }
            ActiveTrack::ActiveSequenceTrack(_) => {
                panic!("attempted to unwrap ActiveSequenceTrack as ActiveTriggerTrack")
            }
            ActiveTrack::ActiveTriggerTrack(active_trigger_track) => active_trigger_track,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct SequenceTree {
    primary_node: Option<ActiveSequence>,
}

impl SequenceTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.primary_node = None;
    }
}

impl SequenceTree {
    pub fn update_recursive(
        &mut self,
        sequence_store: &SimpleStore<Sequence>,
        effect_store: &SimpleStore<Effect>,
        primary_sequence_handle: SimpleHandle<Sequence>,
        primary_sequence_time: f64,
    ) {
        SequenceTree::update_recursive_sequence(
            sequence_store,
            effect_store,
            primary_sequence_handle,
            // create the primary node if it does not exist
            self.primary_node
                .get_or_insert(ActiveSequence::from_sequence(primary_sequence_handle)),
            primary_sequence_time,
        )
    }

    fn update_recursive_sequence(
        sequence_store: &SimpleStore<Sequence>,
        effect_store: &SimpleStore<Effect>,
        current_sequence_handle: SimpleHandle<Sequence>,
        current_active_sequence: &mut ActiveSequence,
        current_time: f64,
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
                    .push(ActiveTrack::from_track(track, effect_store));
            }

            let active_child_element = current_active_sequence
                .children
                .get_mut(track_i)
                .expect("sequence and active sequence track counts don't match");

            match &track.contents {
                TrackContents::EffectTrack { effect_handle } => {
                    SequenceTree::update_recursive_effect_track(
                        effect_store,
                        *effect_handle,
                        active_child_element.as_active_effect_track(),
                        current_time,
                    );
                }
                TrackContents::SequenceTrack { clips } => {
                    SequenceTree::update_recursive_sequence_track(
                        sequence_store,
                        effect_store,
                        clips,
                        active_child_element.as_active_sequence_track(),
                        current_time,
                    );
                }
                TrackContents::TriggerTrack { sequence_handle } => {
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
            .expect("encountered effect that does not exist while updating sequence tree");

        current_active_track.local_time = current_time;

        current_active_track
            .current_info
            .update(&current_effect.keyframes, current_time);

        // No need to recurse!
    }

    fn update_recursive_sequence_track(
        sequence_store: &SimpleStore<Sequence>,
        effect_store: &SimpleStore<Effect>,
        clips: &Vec<Clip>,
        current_active_track: &mut ActiveSequenceTrack,
        current_time: f64,
    ) {
        current_active_track.local_time = current_time;

        let current_clip = clips.find_current(current_time);
        match current_clip {
            Some(current_clip) => {
                match &mut current_active_track.child {
                    Some((time_segment, active_sequence)) => {
                        // the clips are already guaranteed to be on the same
                        // track, so only the time segment needs to be checked
                        if *time_segment != current_clip.time_segment {
                            // there is already a clip but it is the wrong one, so swap it out
                            *active_sequence =
                                ActiveSequence::from_sequence(current_clip.sequence_handle);
                        }
                    }
                    None => {
                        // there was no clip, so make a new one
                        current_active_track.child = Some((
                            current_clip.time_segment,
                            ActiveSequence::from_sequence(current_clip.sequence_handle),
                        ));
                    }
                }

                // now that the child has been updated, recurse on it
                let (time_segment, next_active_sequence) = current_active_track
                    .child
                    .as_mut()
                    .expect("ActiveSequenceTrack child somehow does not exist after creation");
                SequenceTree::update_recursive_sequence(
                    sequence_store,
                    effect_store,
                    current_clip.sequence_handle,
                    next_active_sequence,
                    current_time - time_segment.start_time + time_segment.start_offset,
                );
            }
            // no further checks needed
            None => current_active_track.child = None,
        }
    }
}

pub struct FixtureInfo {
    pub groups: Vec<u32>,
    pub input_type: FixtureType,
    pub position: Vec3,
}

trait FixtureInfoVec {
    fn get_default_inputs(&self) -> Vec<FixtureInput>;
}

impl FixtureInfoVec for &[FixtureInfo] {
    fn get_default_inputs(&self) -> Vec<FixtureInput> {
        self.iter()
            .map(|fixture_info| fixture_info.input_type.get_default_input())
            .collect()
    }
}

impl SequenceTree {
    pub fn get_values_recursive(&self, fixtures: &[FixtureInfo]) -> Vec<FixtureInput> {
        // can be ignored if there is no set primary node
        if let Some(primary_node) = &self.primary_node {
            SequenceTree::get_values_recursive_sequence(primary_node, fixtures)
        } else {
            fixtures.get_default_inputs()
        }
    }

    fn get_values_recursive_sequence(
        current_active_sequence: &ActiveSequence,
        fixtures: &[FixtureInfo],
    ) -> Vec<FixtureInput> {
        let mut final_inputs = fixtures.get_default_inputs();
        for active_track in &current_active_sequence.children {
            match active_track {
                ActiveTrack::ActiveEffectTrack(active_effect_track) => {
                    let new_values = SequenceTree::get_values_recursive_effect_track(
                        active_effect_track,
                        fixtures,
                    );
                    (&mut final_inputs).merge_all(
                        &new_values,
                        active_effect_track.factor,
                        active_effect_track.color_blending_mode,
                    );
                }
                ActiveTrack::ActiveSequenceTrack(active_sequence_track) => {
                    let new_values = SequenceTree::get_values_recursive_sequence_track(
                        active_sequence_track,
                        fixtures,
                    );
                    (&mut final_inputs).merge_all(
                        &new_values,
                        active_sequence_track.factor,
                        active_sequence_track.color_blending_mode,
                    );
                }
                ActiveTrack::ActiveTriggerTrack(active_trigger_track) => {}
            }
        }
        final_inputs
    }

    fn get_values_recursive_effect_track(
        current_active_track: &ActiveEffectTrack,
        fixtures: &[FixtureInfo],
    ) -> Vec<FixtureInput> {
        let mut final_inputs: Vec<FixtureInput> = Vec::new();

        for fixture in fixtures {
            // Because FixtureInput supports merging single values into combined values,
            // we don't have to worry about that here!!!! this is so cool!!!!
            match &current_active_track.current_info {
                EffectInfo::ColorEffectInfo(color_effect) => {
                    let output = color_effect.get_value(fixture.position);
                    final_inputs.push(output.into());
                }
                EffectInfo::Vec3EffectInfo(vec3_effect) => {
                    // TODO: once vec3 effects are fixed replace this
                    let output = Vec3::ZERO;
                    final_inputs.push(output.into());
                }
            }
        }

        final_inputs
        // No need to recurse!
    }

    fn get_values_recursive_sequence_track(
        current_active_track: &ActiveSequenceTrack,
        fixtures: &[FixtureInfo],
    ) -> Vec<FixtureInput> {
        match &current_active_track.child {
            Some((_time_segment, active_sequence)) => {
                SequenceTree::get_values_recursive_sequence(active_sequence, fixtures)
            }
            None => fixtures.get_default_inputs(),
        }
    }
}

#[derive(Event)]
pub struct ClearSequenceTree {}

fn clear_sequence_tree(_reset: On<ClearSequenceTree>, mut sequence_tree: ResMut<SequenceTree>) {
    sequence_tree.clear();
}

fn update_sequence_tree(
    sequence_store: Res<SimpleStore<Sequence>>,
    effect_store: Res<SimpleStore<Effect>>,
    primary_sequence: Res<PrimarySequence>,
    mut sequence_tree: ResMut<SequenceTree>,
    playback_info: Res<PlaybackInformation>,
) {
    let Some(primary_sequence_handle) = primary_sequence.0 else {
        return;
    };

    sequence_tree.update_recursive(
        &sequence_store,
        &effect_store,
        primary_sequence_handle,
        playback_info.current_time,
    );
}
