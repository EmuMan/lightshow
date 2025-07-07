use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct CurrentTime {
    pub time: f64,
}
