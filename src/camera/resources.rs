use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);
