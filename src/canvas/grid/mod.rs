use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
    transform::TransformSystem,
};

pub mod systems;
use systems::*;

pub struct GridPlugin;

#[derive(Resource, Default)]
pub struct CanvasInserted(pub bool);

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomGridMaterial>::default())
            .add_systems(Startup, grid)
            .add_systems(
                PostUpdate,
                (update_grid, grid_follows_camera)
                    .chain()
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "CC3772B3-5282-4F1F-92B5-4F2D864B4441"]
pub struct CustomGridMaterial {
    #[uniform(0)]
    line_color: Color,
    #[uniform(0)]
    grid_size: Vec2,
    #[uniform(0)]
    cell_size: Vec2,
    #[uniform(0)]
    major: f32,
}

impl Material2d for CustomGridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}
