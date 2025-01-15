fn opSmoothUnion(d1: f32, d2: f32, k: f32) -> f32 {
    var h: f32 = clamp(0.5 + 0.5 * (d2 - d1) / k, 0., 1.);
    return mix(d2, d1, h) - k * h * (1. - h);
}

fn sdSphere(p: vec3<f32>, s: f32) -> f32 {
    return length(p) - s;
}

fn map(p: vec3<f32>) -> f32 {
    var d: f32 = 2.;

    for (var i: i32 = 0; i < 16; i = i + 1) {
        let fi: f32 = f32(i);
        let time: f32 = uni.iTime * (fract(fi * 412.531 + 0.513) - 0.5) * 2.;
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
fn fragment(
    let r = 1200;
    let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    var fragColor: vec4<f32>;
    var fragCoord = vec2<f32>(f32(location.x), f32(location.y));

    let uv: vec2<f32> = fragCoord / uni.iResolution.xy;
    let rayOri: vec3<f32> = vec3<f32>((uv - 0.5) * vec2<f32>(uni.iResolution.x / uni.iResolution.y, 1.) * 6., 3.);
    let rayDir: vec3<f32> = vec3<f32>(0., 0., -1.);
    var depth = 0.0;
    var p: vec3<f32>;

    // for (var i: i32 = 0; i < 64; i = i + 1) {
    //     p = rayOri + rayDir * depth;
    //     let dist: f32 = map(p);
    //     depth = depth + (dist);
    //     if dist < 0.000001 {
	// 		break;
    //     }
    // }

    depth = min(6., depth);
    let n: vec3<f32> = calcNormal(p);
    let b: f32 = max(0., dot(n, vec3<f32>(0.577)));
    var col: vec3<f32> = (0.5 + 0.5 * cos(b + uni.iTime * 3. + uv.xyx * 2. + vec3<f32>(0., 2., 4.))) * (0.85 + b * 0.35);
    col = col * (exp(-depth * 0.15));
    fragColor = vec4<f32>(col, 1. - (depth - 0.5) / 2.);
}
