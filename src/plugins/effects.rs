use bevy::prelude::*;

use crate::systems::effects::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, fill::update_fill_effect)
            .add_systems(Update, fill::apply_fill_effect)
            .add_systems(Update, shockwave::update_shockwave_effect)
            .add_systems(Update, shockwave::apply_shockwave_effect);
    }

}
