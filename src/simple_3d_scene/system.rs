use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

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

pub fn camera_ui(light: &mut PointLight, ui: &mut Ui) {
    ui.label("Intensity");
    ui.add(egui::Slider::new(&mut light.intensity, 100.0..=2500.0));
    ui.end_row();

    ui.label("Radius");
    ui.add(egui::Slider::new(&mut light.radius, 0.0..=10.0));
    ui.end_row()
}

pub fn ui_system(mut light_query: Query<&mut PointLight>, mut contexts: EguiContexts) {
    egui::Window::new("3D world")
        .current_pos(Pos2 { x: 10., y: 60. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    light_query.for_each_mut(|mut light| camera_ui(&mut light, ui));
                });
        });
}
