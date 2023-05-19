use bevy::{log::LogPlugin, prelude::*};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

fn setup(mut pkv: ResMut<PkvStore>) {
    // strings
    if let Ok(username) = pkv.get::<String>(PkvKeys::UserName) {
        info!("Welcome back {username}");
    } else {
        pkv.set_string(PkvKeys::UserName, "alice")
            .expect("failed to store username");

        // alternatively, using the slightly less efficient generic api:
        pkv.set(PkvKeys::UserName, &"alice".to_string())
            .expect("failed to store username");
    }

    // serde types
    if let Ok(user) = pkv.get::<User>(PkvKeys::User) {
        info!("Welcome back {}", user.name);
    } else {
        let user = User {
            name: "bob".to_string(),
        };
        pkv.set(PkvKeys::User, &user)
            .expect("failed to store User struct");
    }
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(PkvStore::new("BevyPkv", "EnumExample"))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(strum_macros::AsRefStr)]
enum PkvKeys {
    User,
    UserName,
}
