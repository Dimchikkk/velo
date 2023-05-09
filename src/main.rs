use bevy::prelude::*;
use velo::VeloPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    App::new().add_plugin(VeloPlugin).run();
}
