use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{AsBindGroup, ShaderType, UniformBuffer},
    },
};

#[derive(Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource, Default)]
pub struct BoidsUniform {
    pub time_seconds: f32,
    pub volume_factor: f32,
    pub dt: f32,
    pub density: f32,
    pub evap_rate: f32,
    pub deposition_rate: f32,
    pub min_volume: f32,
    pub friction: f32,
    pub drops_per_frame_per_chunck: u32,
    pub drop_count: u32,
    pub max_drops: u32,
}

impl Default for BoidsUniform {
    fn default() -> Self {
        Self {
            volume_factor: 100.0,
            time_seconds: 0.0,
            dt: 1.2,
            density: 1.0,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            friction: 0.05,
            min_volume: 0.05,
            drops_per_frame_per_chunck: 1000,
            drop_count: 0,
            max_drops: 200_000,
        }
    }
}

/// The buffer containing the [`TerrainUniform`]
#[derive(Resource, Default)]
pub struct TerrainUniformBuffer {
    pub buffer: UniformBuffer<BoidsUniform>,
}

#[derive(Resource, Clone, ExtractResource, AsBindGroup)]
pub(crate) struct BoidsImage {
    #[storage_texture(0, image_format = Rgba32Float, access = ReadWrite)]
    pub(crate) position_map: Handle<Image>,

    #[storage_texture(1, image_format = Rgba32Float, access = ReadWrite)]
    pub(crate) rotation_map: Handle<Image>,

    #[storage_texture(2, image_format = Rgba32Float, access = ReadWrite)]
    pub(crate) velocity_map: Handle<Image>,
}
