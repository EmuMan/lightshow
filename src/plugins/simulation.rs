use bevy::prelude::*;

use crate::resources::simulation::SimulationTime;
use crate::systems::simulation::increment_time;
use crate::systems::tests::pulse_test_startup;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SimulationTime>()
            .add_systems(Update, increment_time)
            .add_systems(Startup, pulse_test_startup);
    }

}
