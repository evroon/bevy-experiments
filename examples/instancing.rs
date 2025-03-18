//! Shows that multiple instances of a cube are automatically instanced in one draw call
//! Try running this example in a graphics profiler and all the cubes should be only a single draw call.

use bevy::{
    color::palettes::css::PURPLE,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const SHADER_ASSET_PATH: &str = "shaders/instancing.wgsl";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, BoidsMaterial>,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct BoidsMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for BoidsMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, BoidsMaterial>>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        // Transform::from_xyz(-200.0, 250.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate,
    ));

    let mesh = meshes.add(Cone::new(0.3, 1.0));

    // spawn 1000 boids
    for _ in 0..1000 {
        commands.spawn((
            // For automatic instancing to take effect you need to
            // use the same mesh handle and material handle for each instance
            Mesh3d(mesh.clone()),
            MeshMaterial3d(materials.add(ExtendedMaterial {
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
                extension: BoidsMaterial { quantize_steps: 3 },
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}
