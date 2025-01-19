use bevy::prelude::*;

use crate::resources::simulation::PlaybackInformation;
use crate::systems::simulation::*;
use crate::systems::tests::pulse_test_startup;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlaybackInformation>()
            .add_systems(Update, increment_playback_time)
            .add_systems(Startup, pulse_test_startup);
    }

}
