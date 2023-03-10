use bevy::prelude::*;
mod hello_plugin;
pub use hello_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}