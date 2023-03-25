mod system;
use system::*;

use bevy::prelude::*;

pub struct Simple3DScenePlugin;

impl Plugin for Simple3DScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(simple_3d_scene);
    }
}
