use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct SimulationTime {
    pub time: f64,
}
