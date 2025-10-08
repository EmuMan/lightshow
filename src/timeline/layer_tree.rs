use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Resource)]
pub struct LayerTree {
    pub all_active: HashMap<Vec<u64>, Entity>,
}
