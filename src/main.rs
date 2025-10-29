use bevy::prelude::*;

use lightshow::LightshowPlugin;

fn main() {
    App::new().add_plugins(LightshowPlugin).run();
}
