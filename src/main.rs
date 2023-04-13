use bevy::{prelude::*, window::PresentMode};
mod chart_plugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_pkv::PkvStore;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_ui_borders::BordersPlugin;
pub use chart_plugin::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_startup_system(setup_background)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Velo".into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(ChartPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(BordersPlugin)
        .insert_resource(PkvStore::new("", "velo"))
        .run();
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("bg.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });
}
