pub mod components;
mod systems;
pub mod resources;

use bevy::prelude::*;

use systems::*;
use resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveEffects>()
            .init_resource::<View>()
            .init_resource::<SimulationTime>()
            .add_systems(Startup, initialize_view)
            .add_systems(Startup, pulse_test_startup)
            .add_systems(Update, increment_time)
            .add_systems(Update, update_light_positions)
            .add_systems(Update, update_light_colors);
    }

}
