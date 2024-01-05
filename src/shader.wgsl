@group(0) @binding(0) var<uniform> camera: mat4x4<f32>;
@group(0) @binding(1) var<uniform> model: mat4x4<f32>;
@group(0) @binding(2) var<uniform> normalMatrix: mat4x4<f32>;
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) worldPos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) texCoords: vec2<f32>,
}
@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) texCoords: vec2<f32>
) -> VertexOut {
    var out: VertexOut;

    out.worldPos = (model * vec4(position, 1.0)).xyz;
    out.normal = (normalMatrix * vec4(normal, 1.0)).xyz;
    out.texCoords = texCoords;
    out.position = camera * model * vec4(position, 1.0);
    return out;
}
//fn hash12(p: vec2<f32>) -> f32 {
//    return fract(sin(p.x*957.15912 + p.y*23.46767) * 4357.926);
//}
fn hash12(p: vec2<f32>) -> f32 {
	var p3 = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise(pos: vec2<f32>) -> f32 {
    var p = floor(pos);
    var f = smoothstep(vec2(0.0), vec2(1.0), fract(pos));

    var bl = hash12(p);
    var br = hash12(p + vec2(1.0, 0.0));
    var tl = hash12(p + vec2(0.0, 1.0));
    var tr = hash12(p + vec2(1.0, 1.0));
    var b = mix(bl, br, f.x);
    var t = mix(tl, tr, f.x);
    var n = mix(b, t, f.y);
    return n;
}
fn fbm(pos: vec2<f32>) -> f32 {
    var frequency = 1.0;
    var amplitude = 0.5;
    var total_added = 0.0;
    var noise = 0.0;
    for (var i = 0; i < 3; i++) {
        noise += noise(pos * frequency) * amplitude;
        total_added += amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    return noise / total_added;
}
fn average_noise(pos: vec2<f32>) -> f32 {
    var n1 = fbm(pos + vec2(1.0, 0.0));
    var n2 = fbm(pos + vec2(0.0, 1.0));
    var n3 = fbm(pos + vec2(-1.0, 0.0));
    var n4 = fbm(pos + vec2(0.0, -1.0));
    return (n1 + n2 + n3 + n4) / 4.0;
}
fn getNormalFromMap(tangentNormal: vec3<f32>, input: VertexOut) -> vec3<f32> {
    var Q1  = dpdx(input.worldPos);
    var Q2  = dpdy(input.worldPos);
    var st1 = dpdx(input.texCoords);
    var st2 = dpdy(input.texCoords);

    var N   = normalize(input.normal);
    var T  = normalize(Q1*st2.y - Q2*st1.y);
    var B  = -normalize(cross(N, T));
    var TBN = mat3x3(T, B, N);

    return normalize(TBN * tangentNormal);
}
@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    var pos = input.texCoords * 10.0;
    var noise = average_noise(pos);
    var theta = 2.0;
    var dx = theta * (average_noise(noise + vec2(1.0, 0.0)) - average_noise(pos - vec2(1.0, 0.0)));
    var dy = theta * (average_noise(noise + vec2(0.0, 1.0)) - average_noise(pos - vec2(0.0, 1.0)));
    var normal = normalize(vec3(dx, dy, 1.0));
    var n = getNormalFromMap(normal, input);
    var sunDir = normalize(vec3(0.4, 0.8, 0.2));
    var sunCol = vec3(1.5, 1.2, 1.1);
    var skyCol = vec3(0.5, 0.7, 1.0);
    var lig = sunCol * max(0., dot(sunDir, n));
    var amb = skyCol * 0.1;
    var total = vec3(0.8) * lig + amb;
    total /= (total + 1.0);
    total = sqrt(total);
    return vec4<f32>(total, 1.0);
}
// vim: ts=4 sts=4 sw=4
