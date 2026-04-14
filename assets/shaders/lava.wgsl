#import bevy_sprite::{
    mesh2d_view_bindings::view,
    mesh2d_view_bindings::globals,
    mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip},
}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Based on https://www.shadertoy.com/view/3sySRK by edankwan

fn opSmoothUnion(d1: f32, d2: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
    return mix(d2, d1, h) - k * h * (1.0 - h);
}

fn sdSphere(p: vec3<f32>, s: f32) -> f32 {
    return length(p) - s;
}

fn map(p: vec3<f32>) -> f32 {
    var d: f32 = 2.;
    let iTime = globals.time;

    for (var i: i32 = 0; i < 16; i = i + 1) {
        let fi: f32 = f32(i);
        let time: f32 = iTime * (fract(fi * 412.531 + 0.513) - 0.5) * 2.;
        d = opSmoothUnion(sdSphere(p + sin(time + fi * vec3<f32>(52.5126, 64.62744, 632.25)) * vec3<f32>(2., 2., 0.8), mix(0.5, 1., fract(fi * 412.531 + 0.5124))), d, 0.4);
    }

    return d;
}

fn calcNormal(p: vec3<f32>) -> vec3<f32> {
    let h: f32 = 0.00001;
    let k: vec2<f32> = vec2<f32>(1., -1.);
    return normalize(k.xyy * map(p + k.xyy * h) + k.yyx * map(p + k.yyx * h) + k.yxy * map(p + k.yxy * h) + k.xxx * map(p + k.xxx * h));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let fragCoord = mesh.uv;
    let iResolution = view.viewport.zw;
    let iTime = globals.time;
    let uv = fragCoord;

    // screen size is 6m x 6m
    let rayOri = vec3((uv - 0.5) * vec2(iResolution.x / iResolution.y, 1.0) * 6.0, 3.0);
    let rayDir = vec3(0.0, 0.0, -1.0);

    var depth = 0.0;
    var p = vec3<f32>();

    for (var i = 0; i < 64; i++) {
        p = rayOri + rayDir * depth;
        let dist = map(p);
        depth += dist;
        if dist < 1e-6 {
			break;
        }
    }

    depth = min(6.0, depth);
    let n = calcNormal(p);
    let b = max(0.0, dot(n, vec3(0.577)));
    var col = (0.5 + 0.5 * cos((b + iTime * 3.0) + uv.xyx * 2.0 + vec3(0, 2, 4))) * (0.85 + b * 0.35);
    col *= exp(-depth * 0.15);

    // maximum thickness is 2m in alpha channel
    return vec4(col, 1.0 - (depth - 0.5) / 2.0);
}
