pub mod mesh;
use mesh::{spawn_bbox, spawn_boids, update_visibility, BoidsMaterial};
mod boids_compute;
mod images;
mod ui;
mod uniforms;

use bevy::{pbr::ExtendedMaterial, prelude::*};

use self::{boids_compute::BoidsComputePlugin, ui::ui_system};

pub const BOX_SIZE: f32 = 1000.0;

pub struct LowPolyTerrainPlugin;

impl Plugin for LowPolyTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, BoidsMaterial>,
        >::default())
            .add_plugins(BoidsComputePlugin)
            .add_systems(Startup, spawn_boids)
            .add_systems(Startup, spawn_bbox)
            .add_systems(Update, ui_system)
            .add_systems(Update, update_visibility);
    }
}
