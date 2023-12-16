use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    pbr::StandardMaterial,
    render::mesh::Mesh,
};
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use super::system::{rebuild_terrain, TerrainBuildConfig};

pub fn terrain_ui(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_query: Query<(Entity, &Handle<Mesh>, &mut TerrainBuildConfig)>,
    ui: &mut Ui,
) {
    ui.add(egui::Slider::new(&mut terrain_query.single_mut().2.seed, 0..=120).text("Seed"));
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().2.base_amplitude,
            0.0..=120.0,
        )
        .text("Base amplitude"),
    );
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().2.base_frequency,
            0.0005..=0.05,
        )
        .text("Base frequency"),
    );
    ui.end_row();

    if ui.button("Rebuild terrain").clicked() {
        rebuild_terrain(commands, meshes, materials, terrain_query);
    };
    ui.end_row();
}

pub fn ui_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    terrain_query: Query<(Entity, &Handle<Mesh>, &mut TerrainBuildConfig)>,
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
                    terrain_ui(commands, meshes, materials, terrain_query, ui);
                });
        });
}
