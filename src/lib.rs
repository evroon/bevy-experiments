use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use boids::LowPolyTerrainPlugin;
use simple_3d_scene::Simple3DScenePlugin;

pub mod boids;
pub mod simple_3d_scene;

pub fn on_resize_system(
    mut mesh: Single<(&Mesh2d, &mut Transform)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        mesh.1.scale = Vec3::new(e.width, e.height, 1.0);
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            Simple3DScenePlugin,
            PanOrbitCameraPlugin,
            LowPolyTerrainPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
