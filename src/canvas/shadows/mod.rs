use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

use self::systems::synchronise_shadow_sizes;

pub mod systems;
pub struct ShadowsPlugin;

impl Plugin for ShadowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomShadowMaterial>::default())
            .add_systems(PostUpdate, synchronise_shadow_sizes);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomShadowMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    flat_size: Vec2,
    #[uniform(0)]
    edge_size: Vec2,
}

impl Material2d for CustomShadowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shadows.wgsl".into()
    }
}
