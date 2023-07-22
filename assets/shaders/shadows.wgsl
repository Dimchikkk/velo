//! https://www.w3.org/TR/WGSL/#builtin-functions

// #import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

// taken from https://github.com/bevyengine/bevy/blob/264195ed772e1fc964c34b6f1e64a476805e1766/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
struct MeshVertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output 
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    // #ifdef VERTEX_TANGENTS
    // @location(3) world_tangent: vec4<f32>,
    // #endif
    // #ifdef VERTEX_COLORS
    // @location(4) color: vec4<f32>,
    // #endif
}

struct CustomShadowMaterial {
    color: vec4<f32>,
    flat_size: vec2<f32>,
    edge_size: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomShadowMaterial;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let color = material.color;
    let flat_size = material.flat_size;
    let edge_size = material.edge_size;
    let full_size = flat_size + edge_size;

    // the size matters because we want the shadow blurred edge to be in physical pixels
    // but uv is relative to the mesh size
    // we ensure, outside, in Bevy-land, that
    // the mesh size is the "shadow-casting object"'s size plus the blurred edge
    // let's imagine that we want 100px of blurred edge, for an 800px object, so that's 1000px
    // if mesh size = 1000px, then that's 10%
    // or more generally, the blurred edge size in 0..1 scale is the blurred edge size in pixels / total mesh size in pixels
    // here, edge_x/edge_y is the blurred edge size in the 0..1 scale

    let edge = edge_size / full_size;
    let ax = smoothstep(0.0, 1.0, symmetric_top(mesh.uv.x) / edge.x / 2.0);
    let ay = smoothstep(0.0, 1.0, symmetric_top(mesh.uv.y) / edge.y / 2.0);

    // `min(ax, ay)` gives sharp corners
    // `ax * ay` gives rounded corners
    // blending the two seems to give the nicest-looking results
    var a = mix(ax * ay, min(ax, ay), 0.3);

    return vec4<f32>(color.rgb, a * color.a);
}

// `v` is a value in the range between 0..0.5..1, and we want it to be a value between 1..0..1
// e.g. 0.2 becomes 0.6, 0.5 becomes 0, 0.6 becomes 0.2
fn symmetric_bottom(v: f32) -> f32 {
    return 2.0 * abs(v - 0.5);
}

// `v` is a value in the range between 0..0.5..1, and we want it to be a value between 0..1..0
// e.g. 0.2 becomes 0.4, 0.5 becomes 1, 0.6 becomes 0.8
fn symmetric_top(v: f32) -> f32 {
    return invert(symmetric_bottom(v));
}

// converts a value `v` from 0..1 to 1..0
// e.g. 0.3 becomes 0.7
fn invert(v: f32) -> f32 {
    return 1.0 - v;
}