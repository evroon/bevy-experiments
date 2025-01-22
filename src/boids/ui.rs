use bevy::ecs::system::ResMut;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use super::boids_compute::BoidsConfig;

pub fn boids_ui(config: &mut BoidsConfig, ui: &mut Ui) {
    ui.add(egui::Slider::new(&mut config.align_range, 1.0..=10.0).text("Align range"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.avoid_range, 0.1..=10.0).text("Avoid range"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.centering_range, 0.01..=10.0).text("Centering range"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.matching_factor, 0.000..=1.0).text("Matching factor"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.avoid_factor, 0.000..=0.1).text("Avoid factor"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.centering_factor, 0.0005..=0.05).text("Centering factor"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.bounds_margin, 0.0..=20.0).text("Bounds margin"));
    ui.end_row();
    ui.add(
        egui::Slider::new(&mut config.bounds_turn_factor, 0.001..=2.0).text("Bounds turn factor"),
    );
    ui.end_row();

    if ui.button("Reset to defaults").clicked() {
        let default = BoidsConfig::default();
        config.align_range = default.align_range;
        config.avoid_range = default.avoid_range;
        config.centering_range = default.centering_range;
        config.matching_factor = default.matching_factor;
        config.avoid_factor = default.avoid_factor;
        config.centering_factor = default.centering_factor;
        config.bounds_margin = default.bounds_margin;
        config.bounds_turn_factor = default.bounds_turn_factor;
    };
}

pub fn ui_system(mut boids_config: ResMut<BoidsConfig>, mut contexts: EguiContexts) {
    egui::Window::new("Boids")
        .current_pos(Pos2 { x: 10., y: 320. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    boids_ui(boids_config.as_mut(), ui);
                });
        });
}
