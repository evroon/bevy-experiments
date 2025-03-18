#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_world, mesh_normal_local_to_world}
#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@group(2) @binding(100) var heightmap_texture: texture_2d<f32>;
@group(2) @binding(101) var heightmap_sampler: sampler;
@group(2) @binding(102) var normalmap_topleft_texture: texture_2d<f32>;
@group(2) @binding(103) var normalmap_topleft_sampler: sampler;
@group(2) @binding(104) var normalmap_bottomright_texture: texture_2d<f32>;
@group(2) @binding(105) var normalmap_bottomright_sampler: sampler;


struct BoidsMaterial {
    quantize_steps: u32,
}

@group(2) @binding(100)
var<uniform> boids_material: BoidsMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // Based on: https://github.com/bevyengine/bevy/blob/286bc8cce52add44e6f6f9c8cd778d26eaa1a761/crates/bevy_pbr/src/render/mesh.wgsl
    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);

    out.instance_index = vertex.instance_index;
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0) + vec4<f32>(f32(vertex.instance_index), 0.0, 0.0, 0.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex.instance_index
    );
    return out;
}


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    out.color = out.color * 2.0;
#endif

    return out;
}