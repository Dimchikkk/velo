//! this example just shows how you can use serde aliases if you rename fields
//!
//! Guess it's more like a serde crash course than an intro to this crate.
//!
//! And it's also a test to show that aliases do work

use bevy::{log::LogPlugin, prelude::*};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserV1 {
    nick: String,
    favorite_color: String,
}

#[derive(Serialize, Deserialize)]
struct UserV2 {
    #[serde(alias = "nick")]
    name: String,
    #[serde(default = "existing_user_default_quest")]
    quest: Quest,
    // notice we no longer care about favorite colors
}

fn existing_user_default_quest() -> Quest {
    // Assume existing users have already played the tutorial
    // and go straight for the holy grail
    Quest::SeekHolyGrail
}

#[derive(Serialize, Deserialize, Debug)]
enum Quest {
    Tutorial,
    SeekHolyGrail,
}

impl Default for Quest {
    fn default() -> Self {
        Quest::Tutorial
    }
}

fn setup(mut pkv: ResMut<PkvStore>) {
    // Let's pretend someone has registered with the UserV1 definition
    let user_v1 = UserV1 {
        nick: "old bob".to_string(),
        favorite_color: "beige".to_string(),
    };
    pkv.set("user", &user_v1)
        .expect("failed to store User struct");

    // When we serialize with the updated struct with the serde annotations,
    // the renamed fields work, and the new fields are assigned values accordingly
    let user_v2 = pkv.get::<UserV2>("user").unwrap();
    info!("Welcome back {}", user_v2.name);
    info!("Current quest: {:?}", user_v2.quest);
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(PkvStore::new("BevyPkv", "MigrationExample"))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_systems(Startup, setup)
        .run();
}
