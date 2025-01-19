use bevy::prelude::*;

use crate::systems::ui::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ui_playback_system);
    }

}
