use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        system::{Query, ResMut},
    },
    prelude::Mesh3d,
    render::mesh::Mesh,
};
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use super::system::{rebuild_terrain, TerrainBuildConfig};

pub fn terrain_ui(
    meshes: ResMut<Assets<Mesh>>,
    mut terrain_query: Query<(Entity, &mut TerrainBuildConfig)>,
    ui: &mut Ui,
) {
    ui.add(egui::Slider::new(&mut terrain_query.single_mut().1.seed, 0..=120).text("Seed"));
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().1.base_amplitude,
            0.0..=120.0,
        )
        .text("Base amplitude"),
    );
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().1.base_frequency,
            0.0005..=0.05,
        )
        .text("Base frequency"),
    );
    ui.end_row();

    if ui.button("Rebuild terrain").clicked() {
        rebuild_terrain(meshes, terrain_query);
    };
    ui.end_row();
}

pub fn ui_system(
    meshes: ResMut<Assets<Mesh>>,
    terrain_query: Query<(Entity, &mut TerrainBuildConfig)>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Terrain")
        .current_pos(Pos2 { x: 10., y: 160. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    terrain_ui(meshes, terrain_query, ui);
                });
        });
}
