use bevy::prelude::*;

use crate::camera_control::CameraController;

use super::BOX_SIZE;

pub fn simple_3d_scene(mut commands: Commands) {
    let camera_controller = CameraController::default();

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.5, 1.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-BOX_SIZE.x * 0.9, BOX_SIZE.y * 1.5, BOX_SIZE.z)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera_controller,
    ));
}
