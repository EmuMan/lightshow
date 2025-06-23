use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);

pub fn spawn_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let camera_transform = Transform::from_translation(camera_pos);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera2d::default(), camera_transform));
}
