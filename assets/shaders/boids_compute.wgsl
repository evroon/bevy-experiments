const TEXTURE_SIZE: u32 = 128;
const TEXTURE_SIZE_F32: f32 = f32(TEXTURE_SIZE);
const BOX_SIZE: u32 = 1000;
const BOX_SIZE_F32: f32 = f32(BOX_SIZE);
const HALF_BOX_SIZE_F32: f32 = BOX_SIZE_F32 * 0.5;
const EPSILON = 0.0001;

struct Config {
    boids_count: u32,
    align_range: f32,
    avoid_range: f32,
    centering_range: f32,
    align_factor: f32,
    avoid_factor: f32,
    centering_factor: f32,
    bounds_margin: f32,
    bounds_turn_factor: f32,
    max_speed: f32,
};

@group(0) @binding(0) var<uniform> config: Config;

@group(1) @binding(0) var position_map: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(1) var velocity_map: texture_storage_2d<rgba32float, read_write>;

fn steer_towards(velocity_self: vec3f, velocity_towards: vec3f) -> vec3f {
    let max_steer_force = 0.01;
    let v = normalize(velocity_towards) * config.max_speed - velocity_self;
    let v_dist_sq = v.x * v.x + v.y * v.y + v.z * v.z;

    if v_dist_sq > max_steer_force * max_steer_force {
        return normalize(v) * max_steer_force;
    }
    return v;
}

fn loop_through_neighbors(position: vec3f, velocity: vec3f) -> vec3f {
    let delta_time = 0.033;
    var avoid_velocity = vec3f();
    var result_velocity = velocity;
    var center = vec3f();
    var acceleration = vec3f();

    var avg_velocity = vec3f();
    var averaging_neighbors = 0;
    var centering_neighbors = 0;
    var i: u32 = 0;

    // Loop through all boids for now even though it's slow
    for (var x: u32 = 0; x < TEXTURE_SIZE; x++) {
        for (var y: u32 = 0; y < TEXTURE_SIZE; y++) {
            if i > config.boids_count { break; }

            let other_position = textureLoad(position_map, vec2u(x, y)).xyz;
            let offset = position - other_position;
            let distance_squared = offset.x * offset.x + offset.y * offset.y + offset.z * offset.z;
            i++;

            if distance_squared < EPSILON * EPSILON {
                continue;
            }

            if distance_squared < config.align_range * config.align_range {
                avg_velocity += textureLoad(velocity_map, vec2u(x, y)).xyz;
                averaging_neighbors++;
            }

            if distance_squared < config.avoid_range * config.avoid_range {
                avoid_velocity += offset;
            }

            if distance_squared < config.centering_range * config.centering_range {
                center += other_position;
                centering_neighbors++;
            }
        }
    }

    if centering_neighbors > 0 {
        acceleration += steer_towards(velocity, (center / f32(centering_neighbors) - position)) * config.centering_factor;
    }

    let avoid_velocity_length_sqrd = avoid_velocity.x * avoid_velocity.x + avoid_velocity.y * avoid_velocity.y + avoid_velocity.z * avoid_velocity.z;
    if abs(config.avoid_factor) > EPSILON && avoid_velocity_length_sqrd > EPSILON {
        acceleration += steer_towards(velocity, avoid_velocity) * config.avoid_factor;
    }

    let avg_velocity_length_sqrd = avg_velocity.x * avg_velocity.x + avg_velocity.y * avg_velocity.y + avg_velocity.z * avg_velocity.z;
    if averaging_neighbors > 0 && avg_velocity_length_sqrd > EPSILON {
        acceleration += steer_towards(velocity, avg_velocity / f32(averaging_neighbors)) * config.align_factor;
    }

    return velocity + acceleration * delta_time;
}

fn keep_boid_within_bounds(position: vec3f) -> vec3f {
    let margin = config.bounds_margin;
    let turn_factor = config.bounds_turn_factor;
    var velocity_diff = vec3f();

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

fn limit_speed(velocity: vec3f) -> vec3f {
    let speed_sqrd = velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z;
    if speed_sqrd > config.max_speed * config.max_speed {
        return normalize(velocity) * config.max_speed;
    }
    return velocity;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let index = vec2f(f32(invocation_id.x), f32(invocation_id.y));
    let max_init_speed = 1.0;

    let location_f32 = vec2f(BOX_SIZE_F32 * (index.x / TEXTURE_SIZE_F32 - 0.5), BOX_SIZE_F32 * (index.y / TEXTURE_SIZE_F32 - 0.5));

    storageBarrier();

    let z = sin(f32(index.x * 960.2 + index.y * 2.0)) * HALF_BOX_SIZE_F32;
    let vx = sin(f32(index.x * 672.2 + index.y * 1.0)) * HALF_BOX_SIZE_F32;
    let vy = sin(f32(index.x * 44.2 + index.y * 3.0)) * max_init_speed;
    let vz = sin(f32(index.x * 123.2 + index.y * 4.0)) * max_init_speed;

    textureStore(position_map, invocation_id.xy, vec4f(location_f32, z, 0.0));
    textureStore(velocity_map, invocation_id.xy, vec4f(vx, vy, vz, 0.0));
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let index = invocation_id.x * TEXTURE_SIZE + invocation_id.y;
    if index > config.boids_count {
        storageBarrier();
        return;
    }

    var location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    var position = textureLoad(position_map, location_i32).xyz;
    var velocity = textureLoad(velocity_map, location_i32).xyz;

    let neighbors_interaction = loop_through_neighbors(position, velocity);

    velocity = neighbors_interaction;
    velocity += keep_boid_within_bounds(position);
    velocity = limit_speed(velocity);

    storageBarrier();

    textureStore(position_map, location_i32, vec4(position + velocity, 0.0));
    textureStore(velocity_map, location_i32, vec4(velocity, 0.0));
}
