const TEXTURE_SIZE: u32 = 128;
const TEXTURE_SIZE_F32: f32 = 128.0;
const BOX_SIZE: u32 = 100;
const BOX_SIZE_F32: f32 = 100.0;
const HALF_BOX_SIZE_F32: f32 = 50.0;

struct Config {
    time_seconds: f32,
    volume_factor: f32,
    dt: f32,
    density: f32,
    evap_rate: f32,
    deposition_rate: f32,
    min_volume: f32,
    friction: f32,
    drops_per_frame_per_chunck: u32,
    drop_count: u32,
    max_drops: u32,
};

@group(0) @binding(0) var<uniform> config: Config;

@group(1) @binding(0) var position_map: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(1) var rotation_map: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(2) var velocity_map: texture_storage_2d<rgba32float, read_write>;

fn keep_boid_within_bounds(position: vec4f) -> vec4f {
    let margin = 2.0;
    let turn_factor = 0.5;
    var velocity_diff = vec4f();

    if position.x < -HALF_BOX_SIZE_F32 + margin {
        velocity_diff.x += turn_factor;
    }
    if position.x > HALF_BOX_SIZE_F32 - margin {
        velocity_diff.x -= turn_factor;
    }
    if position.y < -HALF_BOX_SIZE_F32 + margin {
        velocity_diff.y += turn_factor;
    }
    if position.y > HALF_BOX_SIZE_F32 - margin {
        velocity_diff.y -= turn_factor;
    }
    if position.z < -HALF_BOX_SIZE_F32 + margin {
        velocity_diff.z += turn_factor;
    }
    if position.z > HALF_BOX_SIZE_F32 - margin {
        velocity_diff.z -= turn_factor;
    }

    return velocity_diff;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let index = invocation_id.x * TEXTURE_SIZE + invocation_id.y;
    let location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    let location_f32 = vec2f(
        f32(location_i32.x) * BOX_SIZE_F32 / TEXTURE_SIZE_F32 - 0.5 * BOX_SIZE_F32,
        f32(location_i32.y) * BOX_SIZE_F32 / TEXTURE_SIZE_F32 - 0.5 * BOX_SIZE_F32
    );

    storageBarrier();

    textureStore(position_map, location_i32, vec4f(location_f32, sin(f32(index)) * BOX_SIZE_F32, 0.0));
    textureStore(velocity_map, location_i32, vec4f(0.1, 1.0, 0.5, 0.0));
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    let position = textureLoad(position_map, location_i32);
    var velocity = textureLoad(velocity_map, location_i32);

    velocity += keep_boid_within_bounds(position);
    velocity = normalize(velocity);

    storageBarrier();

    textureStore(position_map, location_i32, vec4(position + velocity));
    textureStore(velocity_map, location_i32, velocity);
}
