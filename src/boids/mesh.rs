use super::{
    boids_compute::BoidsConfig,
    images::{build_images, IMAGE_SIZE},
    uniforms::BoidsImage,
};
use bevy::{
    color::palettes::css::PURPLE,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, Face, ShaderRef},
};

const SHADER_PATH: &str = "shaders/boids_material.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct BoidsMaterial {
    #[texture(100, visibility(vertex))]
    #[sampler(101, visibility(vertex))]
    position_map: Handle<Image>,

    #[texture(102, visibility(vertex))]
    #[sampler(103, visibility(vertex))]
    rotation_map: Handle<Image>,

    #[texture(104, visibility(vertex))]
    #[sampler(105, visibility(vertex))]
    velocity_map: Handle<Image>,
}

impl MaterialExtension for BoidsMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn deferred_vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

pub fn spawn_boids(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, BoidsMaterial>>>,
) {
    let (position_map, rotation_map, velocity_map) = build_images(images);
    let mesh = meshes.add(Tetrahedron::default());
    let material = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: PURPLE.into(),
            // can be used in forward or deferred mode
            opaque_render_method: OpaqueRendererMethod::Auto,
            // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
            // in forward mode, the output can also be modified after lighting is applied.
            // see the fragment shader `extended_material.wgsl` for more info.
            // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
            // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
            ..Default::default()
        },
        extension: BoidsMaterial {
            position_map: position_map.clone(),
            rotation_map: rotation_map.clone(),
            velocity_map: velocity_map.clone(),
        },
    });

    // spawn 1000 boids
    for _ in 0..IMAGE_SIZE * IMAGE_SIZE {
        commands.spawn((
            // For automatic instancing to take effect you need to
            // use the same mesh handle and material handle for each instance
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
    commands.insert_resource(BoidsImage {
        position_map,
        rotation_map,
        velocity_map,
    });

    commands.insert_resource(BoidsConfig::default());
}

pub fn spawn_bbox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(100.0, 100.0, 100.0));

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::new(1.0, 1.0, 1.0, 0.2).into(),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: Some(Face::Front),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::new(1.0, 1.0, 1.0, 1.0).into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, -50.0, 0.0).with_scale(Vec3::new(1.0, 0.01, 1.0)),
    ));
}
