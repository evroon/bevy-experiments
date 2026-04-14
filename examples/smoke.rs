use std::f32::consts::PI;

use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_experiments::math::smooth_stop;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::{rng, Rng};

const SHADER_ASSET_PATH: &str = "shaders/smoke.wgsl";
const PARTICLE_SCALE_START: f32 = 0.03;
const PARTICLE_SCALE_END: f32 = 0.15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, ParticleMaterial>,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, particle_system)
        .add_systems(Update, particle_spawn)
        .run();
}

#[derive(Component)]
struct Particle {
    pub age: f32, // in range [0.0, 1.0]
    pub start_position: Vec3,
    pub start_velocity: Vec3,
    pub start_rotation: f32,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct ParticleMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    alpha: u32,
}

impl MaterialExtension for ParticleMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

fn get_billboard_quat(camera_translation: Vec3, entity_translation: Vec3) -> Quat {
    let x_dir = (camera_translation - entity_translation).normalize();
    let z_dir = x_dir.cross(Vec3::Y);
    let y_dir = z_dir.cross(x_dir);
    let rot_mat = Mat3::from_cols(z_dir, x_dir, y_dir);
    Quat::from_mat3(&rot_mat)
}

type ExtMaterial = ExtendedMaterial<StandardMaterial, ParticleMaterial>;
type ExtendedMaterial3D = MeshMaterial3d<ExtMaterial>;

fn particle_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut Particle, &ExtendedMaterial3D), Without<Camera3d>>,
    c: Single<&Transform, (With<Camera3d>, Without<Particle>)>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ParticleMaterial>>>,
) {
    let mut mat_id: Option<AssetId<ExtMaterial>> = None;
    for (entity, mut transform, mut particle, material) in &mut q {
        transform.rotation = get_billboard_quat(c.translation, transform.translation);
        transform.rotate_local_y(particle.start_rotation);

        let t_scale = smooth_stop(particle.age, 5);

        transform.translation = particle.start_position + particle.start_velocity * t_scale;
        transform.scale = Vec3::splat(
            PARTICLE_SCALE_START + (PARTICLE_SCALE_END - PARTICLE_SCALE_START) * t_scale,
        );
        particle.age += 0.01;

        if particle.age > 1.0 {
            commands.entity(entity).despawn();
        }
        mat_id = Some(material.id());
    }
    if let Some(mat_id) = mat_id {
        let instance_count = q.iter().len() as u32;
        println!("{}", instance_count);
        extended_materials.get_mut(mat_id).unwrap().extension.alpha = instance_count;
    }
}

/// set up a simple 3D scene
fn particle_spawn(
    mut commands: Commands,
    query: Query<(&Mesh3d, &ExtendedMaterial3D), With<Particle>>,
    c: Single<&Transform, (With<Camera3d>, Without<Particle>)>,
) {
    let (mesh, material) = query.iter().next().unwrap();
    let mut rng = rng();
    let spread = 0.3;

    for _ in 0..2 {
        commands.spawn((
            Mesh3d(mesh.0.clone()),
            MeshMaterial3d(material.0.clone()),
            Transform::IDENTITY
                .with_scale(Vec3::splat(PARTICLE_SCALE_START))
                .with_rotation(get_billboard_quat(c.translation, Vec3::ZERO))
                .rotate_local_z(rng.random_range(0.0..2.0 * PI)),
            Particle {
                age: 0.0,
                start_velocity: Vec3::new(
                    rng.random_range(-spread..spread),
                    1.0,
                    rng.random_range(-spread..spread),
                )
                .normalize()
                    * Vec3::splat(rng.random_range(1.0 - spread..1.0 + spread)),
                start_position: Vec3::ZERO,
                start_rotation: rng.random_range(0.0..2.0 * PI),
            },
        ));
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ParticleMaterial>>>,
) {
    let texture_handle = asset_server.load("textures/smoke.png");

    let mut rng = rng();
    let camera_height = 2.0;

    // camera
    commands.spawn((
        Camera3d::default(),
        // Transform::from_xyz(-200.0, 250.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        Transform::from_xyz(-2.0, 2.5 + camera_height, 5.0)
            .looking_at(Vec3::new(0.0, camera_height, 0.0), Vec3::Y),
        PanOrbitCamera {
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Left,
            ..Default::default()
        },
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::IDENTITY.looking_at(-Vec3::Y, Vec3::Z),
    ));

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE));

    // Spawn plane
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(standard_materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::IDENTITY,
    ));

    // Spawn particle
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(extended_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::linear_rgba(0.3, 0.3, 0.3, 0.3),
                base_color_texture: Some(texture_handle),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: ParticleMaterial { alpha: 1 },
        })),
        Transform::IDENTITY.with_scale(Vec3::splat(PARTICLE_SCALE_START)),
        Particle {
            age: 0.0,
            start_velocity: Vec3::Y,
            start_position: Vec3::ZERO,
            start_rotation: rng.random_range(0.0..2.0 * PI),
        },
    ));
}
