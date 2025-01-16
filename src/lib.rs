use bevy::prelude::*;
use bevy::window::WindowResized;

pub fn on_resize_system(
    mut mesh: Single<(&Mesh2d, &mut Transform)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        mesh.1.scale = Vec3::new(e.width, e.height, 1.0);
    }
}
