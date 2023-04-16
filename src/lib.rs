mod canvas;
mod chart_plugin;
mod components;
mod resources;
mod systems;
mod utils;
use bevy::{prelude::*, window::PresentMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_pkv::PkvStore;
use canvas::CanvasPlugin;
use chart_plugin::*;
use systems::*;
pub struct VeloPlugin;
impl Plugin for VeloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
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
            .add_plugin(CanvasPlugin)
            .add_plugin(ChartPlugin)
            .insert_resource(PkvStore::new("", "velo"));
    }
}
