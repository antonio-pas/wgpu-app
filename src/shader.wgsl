
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> model: mat4x4<f32>;
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
    out.normal = (model * vec4(normal, 1.0)).xyz;
    out.texCoords = texCoords;
    out.position = mvp * vec4(position, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(sqrt(input.texCoords), 0.44, 1.0);
}