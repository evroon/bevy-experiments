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


struct ParticleMaterial {
    alpha: u32,
}

@group(2) @binding(100)
var<uniform> particle_material: ParticleMaterial;


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // Based on: https://github.com/bevyengine/bevy/blob/286bc8cce52add44e6f6f9c8cd778d26eaa1a761/crates/bevy_pbr/src/render/mesh.wgsl
    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);

    out.instance_index = vertex.instance_index;
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
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
    let max_instances = f32(particle_material.alpha);
    var out: FragmentOutput;
    out.color = vec4(0.3, 0.3, 0.3, (max_instances - f32(in.instance_index)) / max_instances * 0.5);
    return out;
}
