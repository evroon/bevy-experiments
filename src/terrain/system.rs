use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};

use super::{CELL_SIZE, TERRAIN_SIZE};

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell_count = usize::try_from(TERRAIN_SIZE.x * TERRAIN_SIZE.y).unwrap();
    let triangle_count = cell_count * 4;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut positions = vec![[0., 0., 0.]; triangle_count];
    let mut indices = vec![0u32; cell_count * 6];

    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let x_pos = (x as f32) * CELL_SIZE;
            let z_pos = (y as f32) * CELL_SIZE;

            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();
            positions[i * 4 + 0] = [x_pos, 0., z_pos];
            positions[i * 4 + 1] = [x_pos + CELL_SIZE, 0., z_pos + CELL_SIZE];
            positions[i * 4 + 2] = [x_pos, 0., z_pos + CELL_SIZE];
            positions[i * 4 + 3] = [x_pos + CELL_SIZE, 0., 0.];

            let i_index = i_32 * 6;
            let i_idx_usize = usize::try_from(i_index).unwrap();

            let slice = &[
                i_32 * 4 + 0,
                i_32 * 4 + 2,
                i_32 * 4 + 1,
                i_32 * 4 + 0,
                i_32 * 4 + 1,
                i_32 * 4 + 3,
            ];
            indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; triangle_count]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; triangle_count]);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    create_mesh(commands, meshes, materials);
}
