use crate::{components::MainCamera, resources::FontHandle};
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
pub fn set_default_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    font_handle: Res<FontHandle>,
) {
    if let Some(font) = fonts.remove(&font_handle.0) {
        let _ = fonts.set(TextStyle::default().font, font);
        commands.remove_resource::<FontHandle>();
    }
}
pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    commands.insert_resource(FontHandle(font));
}