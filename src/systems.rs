use crate::components::{EffectsCamera, MainCamera};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("bg.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });
}

pub fn setup_camera(mut commands: Commands) {
    let main_camera = Camera2dBundle::default();
    commands.spawn((main_camera, MainCamera));
    let mut effects_camera = Camera2dBundle {
        camera: Camera {
            order: 2,
            is_active: false,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..default()
    };
    effects_camera.projection.scale = 1.0;
    effects_camera.projection.scaling_mode = ScalingMode::FixedVertical(1.);
    commands.spawn((
        effects_camera,
        EffectsCamera,
        RenderLayers::from_layers(&[2]),
    ));
}
