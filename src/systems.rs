use crate::{
    components::{EffectsCamera, MainCamera},
    themes::{get_theme_by_name, Theme},
    ui_plugin::ui_helpers::{Background, InteractiveNode},
    utils::UserPreferences,
};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};
use bevy_pkv::PkvStore;

pub fn setup_velo_theme(mut commands: Commands, pkv: Res<PkvStore>) {
    let theme_name = if let Ok(user_preferences) = pkv.get::<UserPreferences>("user_preferences") {
        if let Some(theme_name) = user_preferences.theme_name {
            theme_name
        } else {
            "light".to_string()
        }
    } else {
        "light".to_string()
    };
    let theme = get_theme_by_name(&theme_name);
    commands.insert_resource(theme);
}

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>, theme: Res<Theme>) {
    let mut sprite_bundle = SpriteBundle::default();
    if let Some(bg_img) = theme.canvas_bg_img.clone() {
        sprite_bundle.texture = asset_server.load(bg_img);
    }
    if let Some(bg_color) = theme.canvas_bg_color {
        sprite_bundle.sprite.color = bg_color;
    }
    commands.spawn((sprite_bundle, Background, InteractiveNode));
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
