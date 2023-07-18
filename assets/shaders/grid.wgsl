#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct CustomGridMaterial {
    color: vec4<f32>,
    line_color: vec4<f32>,
    grid_size: vec2<f32>,
    cell_size: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomGridMaterial;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let color: vec4<f32> = material.color;
    let line_color: vec4<f32> = material.line_color;
    let grid_size: vec2<f32> = material.grid_size;
    let cell_size: vec2<f32> = material.cell_size;

    // Calculate the relative position of the current pixel to the origin of the grid
    let relative_pos = ceil((mesh.uv - vec2(0.5)) * grid_size);

    // Check if the relative position is on a grid line
    let epsilon: f32 = 0.01;
    let on_line_x = fract(relative_pos.x / cell_size.x) < epsilon || fract(relative_pos.x / cell_size.x) > (1.0 - epsilon);
    let on_line_y = fract(relative_pos.y / cell_size.y) < epsilon || fract(relative_pos.y / cell_size.y) > (1.0 - epsilon);

    // If the relative position is on a grid line, return the line color
    if on_line_x || on_line_y {
        return line_color;
    }

    // Otherwise, return the color
    return color;
}
