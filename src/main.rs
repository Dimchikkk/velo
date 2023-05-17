use bevy::prelude::*;
use velo::VeloPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(not(target_arch = "wasm32"))]
    // std::env::set_var("RUST_LOG", "error,info,warn");
    App::new().add_plugin(VeloPlugin).run();
}
