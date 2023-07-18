#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct CustomGridMaterial {
    line_color: vec4<f32>,
    grid_size: vec2<f32>,
    cell_size: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomGridMaterial;

fn grid(point: vec2<f32>, cell_size: vec2<f32>, thickness: f32) -> f32 {
  let x = abs(fract(point.x / cell_size.x)) * cell_size.x - thickness;
  let y = abs(fract(point.y / cell_size.y)) * cell_size.y - thickness;
  return min(x, y);
}

fn origin(point: vec2<f32>, thickness: f32) -> f32 {
  return min(abs(point.x), abs(point.y)) - thickness;
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let line_color: vec4<f32> = material.line_color;
    let grid_size: vec2<f32> = material.grid_size;
    let cell_size: vec2<f32> = material.cell_size;

    let point = (mesh.uv - vec2(0.5)) * grid_size;

    let t = grid(point, cell_size, 1.);
    let u = grid(point, cell_size, 2.);
    let g = min(t, u);
    let alpha =  1.0 - smoothstep(0.0, fwidth(g), g);
    
    // let o = origin(point, 1.);
    // let fwo = fwidth(o);
    // if o < 0.0 {
    //     let alpha = 1.0 - smoothstep(0.0, fwo, o);
    //     return vec4(1.0, 0., 0., alpha);
    // }
    return vec4(line_color.rgb, alpha * line_color.a);
}
