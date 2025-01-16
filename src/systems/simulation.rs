use bevy::prelude::*;

use crate::resources::simulation::*;

pub fn increment_time(
    time: Res<Time>,
    mut simulation_time: ResMut<SimulationTime>,
) {
    simulation_time.time += time.delta_secs_f64();
}
