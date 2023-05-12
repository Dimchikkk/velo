mod canvas;
mod components;
mod resources;
mod systems;
mod ui_plugin;
mod utils;
use bevy::{prelude::*, window::PresentMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(not(target_arch = "wasm32"))]
use bevy_hanabi::HanabiPlugin;
use bevy_pkv::PkvStore;
use bevy_ui_borders::BordersPlugin;
use canvas::CanvasPlugin;
use resources::FontHandle;
use systems::*;
use ui_plugin::*;
pub struct VeloPlugin;
impl Plugin for VeloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_background)
            .add_startup_system(setup_font)
            .add_system(set_default_font.run_if(resource_exists::<FontHandle>()))
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
            .add_plugin(UiPlugin)
            .add_plugin(BordersPlugin)
            .insert_resource(PkvStore::new("", "velo"));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugin(HanabiPlugin);
    }
}
