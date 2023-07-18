#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct CustomGridMaterial {
    color: vec4<f32>,
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

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    // let color: vec4<f32> = material.color;
    let line_color: vec4<f32> = material.line_color;
    let grid_size: vec2<f32> = material.grid_size;
    let cell_size: vec2<f32> = material.cell_size;

    // Calculate the relative position of the current pixel to the origin of the grid
    let point = floor((mesh.uv - vec2(0.5)) * grid_size);

    // Check if the relative position is on a grid line
    let t = grid(point, cell_size, 0.1);
    let u = grid(point, cell_size * 10.,   1.0);
    let g = min(t, u);
    let alpha =  1.0 - smoothstep(0.0, fwidth(g), g);
    
    
    var color = line_color;
    if abs(point.x) < 0.1 || abs(point.y) < 0.1 {
        color = vec4(1.0, 0.,0., 1.);
    } 

    return vec4(color.rgb, alpha * color.a);
}
