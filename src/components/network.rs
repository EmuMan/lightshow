use bevy::prelude::*;

#[derive(Component)]
pub struct ArtNetNode {
    pub ip: String,
    pub port: u16,
    pub universe: u16,
    pub channels: Vec<u16>,
}

impl Default for ArtNetNode {
    fn default() -> Self {
        ArtNetNode {
            ip: "0.0.0.0".into(),
            port: 6454,
            universe: 0,
            channels: vec![0; 512],
        }
    }
}
