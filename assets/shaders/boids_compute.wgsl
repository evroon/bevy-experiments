const TEXTURE_SIZE: u32 = 128;
const TEXTURE_SIZE_F32: f32 = 128.0;
const BOX_SIZE: u32 = 100;
const BOX_SIZE_F32: f32 = 100.0;
const HALF_BOX_SIZE_F32: f32 = 50.0;
const MAX_SPEED = 1.0;

struct Config {
    align_range: f32,
    avoid_range: f32,
    centering_range: f32,
    matching_factor: f32,
    avoid_factor: f32,
    centering_factor: f32,
    bounds_margin: f32,
    bounds_turn_factor: f32,
};

@group(0) @binding(0) var<uniform> config: Config;

@group(1) @binding(0) var position_map: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(1) var rotation_map: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(2) var velocity_map: texture_storage_2d<rgba32float, read_write>;

fn loop_through_neighbors(position: vec3f, velocity: vec3f) -> vec3f {
    var avoid_velocity = vec3f();
    var result_velocity = velocity;
    var center = vec3f();

    var avg_velocity = vec3f();
    var averaging_neighbors = 0;
    var centering_neighbors = 0;

    // Loop through all boids for now even though it's slow
    for (var x: u32 = 0; x < TEXTURE_SIZE; x++) {
        for (var y: u32 = 0; y < TEXTURE_SIZE; y++) {
            let other_position = textureLoad(position_map, vec2u(x, y)).xyz;
            let distance = length(position - other_position);

            if distance == 0 {
                continue;
            }

            if distance < config.align_range {
                avg_velocity += textureLoad(velocity_map, vec2u(x, y)).xyz;
                averaging_neighbors++;
            }

            if distance < config.avoid_range {
                avoid_velocity += position - other_position;
            }

            if distance < config.centering_range {
                center += other_position;
                centering_neighbors++;
            }
        }
    }

    if averaging_neighbors > 0 {
        // Align velocity with other neighbors
        result_velocity += (avg_velocity / f32(averaging_neighbors) - position) * config.matching_factor;
    }

    if centering_neighbors > 0 {
        // Fly towards the center of mass of neighbors
        result_velocity += (center / f32(centering_neighbors) - position) * config.centering_factor;
    }

    result_velocity += avoid_velocity * config.avoid_factor;

    return result_velocity;
}

fn keep_boid_within_bounds(position: vec4f) -> vec4f {
    let margin = config.bounds_margin;
    let turn_factor = config.bounds_turn_factor;
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

fn limit_speed(velocity: vec4f) -> vec4f {
    let speed = length(velocity);
    if length(velocity) > MAX_SPEED {
        return velocity / speed * MAX_SPEED;
    }
    return velocity;
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

    let neighbors_interaction = loop_through_neighbors(position.xyz, velocity.xyz);

    velocity = vec4(neighbors_interaction, 0.0);
    velocity = limit_speed(velocity);
    velocity += keep_boid_within_bounds(position);

    storageBarrier();

    textureStore(position_map, location_i32, vec4(position + velocity));
    textureStore(velocity_map, location_i32, velocity);
}
