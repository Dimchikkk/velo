use bevy::prelude::*;
mod HelloPlugin;
pub use HelloPlugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}