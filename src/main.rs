use bevy::prelude::*;
use velo::VeloPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(not(target_arch = "wasm32"))]
    std::env::set_var("RUST_LOG", "warn,velo=info,tantivy=warn,bevy_render=off");
    // bevy_render=off until https://github.com/johanhelsing/bevy_smud/issues/26 is fixed ^^
    App::new().add_plugin(VeloPlugin).run();
}
