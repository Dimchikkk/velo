use bevy::prelude::*;

/// Bevy plugin to add to your application that will insert a custom [`AssetServer`] embedding
/// your assets instead of the default added by the [`AssetPlugin`](bevy::asset::AssetPlugin).
/// If you are using the [`DefaultPlugins`] group from Bevy, it can be added this way:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_embedded_assets::EmbeddedAssetPlugin;
/// # fn main() {
/// App::new().add_plugins(
///     DefaultPlugins
///         .build()
///         .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
/// );
/// # }
/// ```
#[allow(
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::module_name_repetitions
)]
#[derive(Default)]
pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        if app.is_plugin_added::<AssetPlugin>() {
            error!("plugin EmbeddedAssetPlugin must be added before plugin AssetPlugin");
        }
        app.insert_resource(AssetServer::new(crate::EmbeddedAssetIo::preloaded()));
    }
}
