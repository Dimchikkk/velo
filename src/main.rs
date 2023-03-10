use bevy::prelude::*;
mod hello_plugin;
pub use hello_plugin::*;

fn main() {
    App::new()
        .add_startup_system(setup_background)
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("bg.png");
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });
}
