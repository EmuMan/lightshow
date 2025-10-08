use bevy::prelude::*;

use crate::{
    simple_store::{SimpleHandle, SimpleStore},
    timeline::tracks::Track,
};

pub struct LayersPlugin;

impl Plugin for LayersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimpleStore<Layer>>()
            .init_resource::<PrimaryLayer>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct PrimaryLayer(pub Option<SimpleHandle<Layer>>);

#[derive(Debug)]
pub struct Layer {
    pub name: String,
    pub length: f64,
    pub tracks: Vec<Track>,
    pub info: LayerInfo,
}

#[derive(Debug)]
pub struct LayerInfo {
    pub strength: f64,
}

#[derive(Debug, Component)]
pub struct ActiveLayer {
    pub original: SimpleHandle<Layer>,
    pub current_info: LayerInfo,
    pub children: Vec<Entity>,
}
