use std::borrow::Cow;

use bevy::{
    ecs::system::ResMut,
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel},
        render_resource::{
            binding_types::uniform_buffer, AsBindGroup, BindGroup, BindGroupEntries,
            BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId, CachedPipelineState,
            ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderStages,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::GpuImage,
        Extract, Render, RenderApp, RenderSet,
    },
};
use rand::{thread_rng, Rng};

use super::{
    images::IMAGE_SIZE,
    uniforms::{BoidsImage, BoidsUniform, TerrainUniformBuffer},
};

const WORKGROUP_SIZE: u32 = 8;

#[derive(Resource, Clone, Copy)]
pub struct BoidsConfig {
    pub volume_factor: f32,
    pub dt: f32,
    pub density: f32,
    pub evap_rate: f32,
    pub deposition_rate: f32,
    pub min_volume: f32,
    pub friction: f32,
    pub drops_per_frame_per_chunk: u32,
    pub drop_count: u32,
    pub max_drops: u32,
}

impl Default for BoidsConfig {
    fn default() -> Self {
        Self {
            volume_factor: 100.0,
            dt: 1.2,
            density: 1.0,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            friction: 0.05,
            min_volume: 0.05,
            drops_per_frame_per_chunk: 1000,
            drop_count: 0,
            max_drops: 200_000,
        }
    }
}

#[derive(Resource)]
pub struct BoidsUniformBindGroup(BindGroup);

#[derive(Resource)]
pub struct BoidsImageBindGroup(BindGroup);

pub(crate) fn prepare_uniforms_bind_group(
    mut commands: Commands,
    pipeline: Res<BoidsPipeline>,
    render_queue: Res<RenderQueue>,
    mut terrain_uniform_buffer: ResMut<TerrainUniformBuffer>,
    boids_config: Res<BoidsConfig>,
    render_device: Res<RenderDevice>,
) {
    let buffer = terrain_uniform_buffer.buffer.get_mut();
    let mut rng = thread_rng();

    buffer.time_seconds = rng.gen_range(0.0..1e6); // * time.elapsed_seconds_wrapped();
    buffer.volume_factor = boids_config.volume_factor;
    buffer.dt = boids_config.dt;
    buffer.density = boids_config.density;
    buffer.evap_rate = boids_config.evap_rate;
    buffer.deposition_rate = boids_config.deposition_rate;
    buffer.min_volume = boids_config.min_volume;
    buffer.friction = boids_config.friction;
    buffer.drops_per_frame_per_chunck = boids_config.drops_per_frame_per_chunk;
    buffer.drop_count = boids_config.drop_count;
    buffer.max_drops = boids_config.max_drops;

    terrain_uniform_buffer
        .buffer
        .write_buffer(&render_device, &render_queue);

    let bind_group_uniforms = render_device.create_bind_group(
        None,
        &pipeline.uniform_bind_group_layout,
        &BindGroupEntries::single(terrain_uniform_buffer.buffer.binding().unwrap().clone()),
    );
    commands.insert_resource(BoidsUniformBindGroup(bind_group_uniforms));
}

pub(crate) fn prepare_textures_bind_group(
    mut commands: Commands,
    pipeline: Res<BoidsPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    boids_image: Res<BoidsImage>,
    render_device: Res<RenderDevice>,
) {
    let position_map_view = gpu_images.get(&boids_image.position_map).unwrap();
    let rotation_map_view = gpu_images.get(&boids_image.rotation_map).unwrap();
    let velocity_map_view = gpu_images.get(&boids_image.velocity_map).unwrap();

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &position_map_view.texture_view,
            &rotation_map_view.texture_view,
            &velocity_map_view.texture_view,
        )),
    );
    commands.insert_resource(BoidsImageBindGroup(bind_group));
}

#[derive(Resource)]
pub struct BoidsPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub uniform_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for BoidsPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = BoidsImage::bind_group_layout(render_device);
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/boids_compute.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();

        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (uniform_buffer::<BoidsUniform>(false),),
        );

        let uniform_bind_group_layout =
            render_device.create_bind_group_layout("uniform_bind_group_layout", &entries);

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            zero_initialize_workgroup_memory: false,
            label: None,
            layout: vec![
                uniform_bind_group_layout.clone(),
                texture_bind_group_layout.clone(),
            ],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            zero_initialize_workgroup_memory: false,
            label: None,
            layout: vec![
                uniform_bind_group_layout.clone(),
                texture_bind_group_layout.clone(),
            ],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        BoidsPipeline {
            texture_bind_group_layout,
            uniform_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum BoidsState {
    Loading,
    Init,
    Update,
}

struct BoidsNode {
    state: BoidsState,
}

impl Default for BoidsNode {
    fn default() -> Self {
        Self {
            state: BoidsState::Loading,
        }
    }
}

impl Node for BoidsNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<BoidsPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            BoidsState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = BoidsState::Init;
                }
            }
            BoidsState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = BoidsState::Update;
                }
            }
            BoidsState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<BoidsImageBindGroup>().0;
        let uniform_bind_group = &world.resource::<BoidsUniformBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<BoidsPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, uniform_bind_group, &[]);
        pass.set_bind_group(1, texture_bind_group, &[]);

        match self.state {
            BoidsState::Loading => {}
            BoidsState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    IMAGE_SIZE / WORKGROUP_SIZE,
                    IMAGE_SIZE / WORKGROUP_SIZE,
                    1,
                );
            }
            BoidsState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    IMAGE_SIZE / WORKGROUP_SIZE,
                    IMAGE_SIZE / WORKGROUP_SIZE,
                    1,
                );
            }
        }
        Ok(())
    }
}

pub struct BoidsComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct BoidsLabel;

impl Plugin for BoidsComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<BoidsImage>::default());
        app.add_plugins(ExtractResourcePlugin::<BoidsUniform>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_textures_bind_group.in_set(RenderSet::PrepareResources),
        );
        render_app.add_systems(
            Render,
            prepare_uniforms_bind_group.in_set(RenderSet::PrepareResources),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(BoidsLabel, BoidsNode::default());
        render_graph.add_node_edge(BoidsLabel, bevy::render::graph::CameraDriverLabel);

        render_app.add_systems(ExtractSchedule, (extract_boids_config, extract_time));
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<BoidsPipeline>();
        render_app.init_resource::<TerrainUniformBuffer>();
    }
}

fn extract_boids_config(mut commands: Commands, config: Extract<Res<BoidsConfig>>) {
    commands.insert_resource(**config);
}

fn extract_time(mut commands: Commands, time: Extract<Res<Time>>) {
    commands.insert_resource(**time);
}
