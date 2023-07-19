#import bevy_sprite::mesh2d_functions as MeshFunctions

struct CustomShadowMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomShadowMaterial;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) size: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

fn sd_box(p: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2<f32>(0.))) + min(max(d.x, d.y), 0.);
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let d = sd_box(mesh.uv, 0.35 * mesh.size);
    let power = 12.0;
    let width = 0.4 * mesh.size.x;
    var a = (width - d) / width;
    a = clamp(a, 0.0, 1.0);
    a = pow(a, power);
    let color = material.color;
    return vec4<f32>(color.rgb, a * color.a);
}

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.size = 2.0 * abs(vertex_uv);
    out.position = MeshFunctions::mesh2d_position_world_to_clip(vec4(vertex_position, 1.0));;
    return out;
}