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
    pub boids_count: u32,
    pub align_range: f32,
    pub avoid_range: f32,
    pub centering_range: f32,
    pub align_factor: f32,
    pub avoid_factor: f32,
    pub centering_factor: f32,
    pub bounds_margin: f32,
    pub bounds_turn_factor: f32,
    pub max_speed: f32,
}

impl Default for BoidsUniform {
    fn default() -> Self {
        Self {
            boids_count: 2000,
            align_range: 100.0,
            avoid_range: 50.0,
            centering_range: 50.0,
            align_factor: 5.0,
            avoid_factor: 5.0,
            centering_factor: 5.0,
            bounds_margin: 2.0,
            bounds_turn_factor: 0.5,
            max_speed: 1.0,
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
    pub(crate) velocity_map: Handle<Image>,
}
