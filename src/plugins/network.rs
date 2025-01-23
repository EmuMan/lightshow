use bevy::prelude::*;

use crate::resources::network::*;
use crate::systems::network::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<ArtNetConnections>()
            .init_resource::<ActiveSocket>()
            .add_systems(FixedUpdate, send_and_erase_buffers);
    }

}
