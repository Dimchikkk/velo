use crate::components::MainCamera;
use bevy::prelude::*;
pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("bg.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });
}
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
