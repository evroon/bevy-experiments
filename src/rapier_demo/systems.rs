use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::simple_3d_scene::BOX_SIZE;

pub const SPHERE_COUNT: u32 = 100;
pub const SPHERE_RADIUS: f32 = 0.2;

fn get_random_position_in_box(mut rng: ThreadRng) -> Transform {
    Transform::from_xyz(
        BOX_SIZE.x * (rng.gen::<f32>() - 0.5),
        BOX_SIZE.y * rng.gen::<f32>(),
        BOX_SIZE.z * (rng.gen::<f32>() - 0.5),
    )
}

pub fn spawn_rigid_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rng = thread_rng();

    for _ in 0..SPHERE_COUNT {
        commands
            .spawn((
                Mesh3d(meshes.add(Mesh::from(Sphere {
                    radius: SPHERE_RADIUS,
                }))),
                MeshMaterial3d(materials.add(Color::linear_rgb(0.8, 0.2, 0.2))),
                get_random_position_in_box(rng.clone()),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(SPHERE_RADIUS));
    }
}

pub fn spawn_on_mouseclick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let rng = thread_rng();

    if mouse_button_input.pressed(MouseButton::Left) {
        commands
            .spawn((
                Mesh3d(meshes.add(Mesh::from(Sphere {
                    radius: SPHERE_RADIUS,
                }))),
                MeshMaterial3d(materials.add(Color::linear_rgb(0.2, 0.7, 0.8))),
                get_random_position_in_box(rng.clone()),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(SPHERE_RADIUS));
    }
}
