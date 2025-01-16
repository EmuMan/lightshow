use bevy::prelude::*;

use crate::systems::fixtures::*;

pub struct FixturesPlugin;

impl Plugin for FixturesPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, color_light::apply_color_light_color_queues);
    }

}
