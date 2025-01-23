use bevy::prelude::*;

use crate::systems::effects::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, fill::update_fill_effect)
            .add_systems(FixedUpdate, fill::apply_fill_effect)
            .add_systems(FixedUpdate, shockwave::update_shockwave_effect)
            .add_systems(FixedUpdate, shockwave::apply_shockwave_effect);
    }

}
