use bevy::prelude::*;

use crate::systems::fixtures::*;

pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, color_light::apply_color_light_color_queues)
            .add_systems(FixedUpdate, color_light::add_data_to_buffer);
    }

}
