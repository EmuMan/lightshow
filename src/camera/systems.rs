use bevy::prelude::*;
use super::resources::OriginalCameraTransform;

pub fn spawn_camera(
    mut commands: Commands,
) {
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let camera_transform = Transform::from_translation(camera_pos);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera2d::default(), camera_transform));
}
