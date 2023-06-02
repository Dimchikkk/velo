use bevy::prelude::*;

use bevy_cosmic_edit::CosmicTextPos;
use serde::{Deserialize, Serialize};

use crate::resources::AppState;
use crate::ui_plugin::TextPos;

use std::collections::HashMap;
use std::{fs, path::PathBuf};
use uuid::Uuid;

use bevy_pkv::PkvStore;

use crate::{components::Doc, ui_plugin::MAX_SAVED_DOCS_IN_MEMORY};

#[derive(Clone, Reflect, Default, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[reflect_value]
pub struct ReflectableUuid(pub Uuid);

#[derive( Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme_name: Option<String>,
}

impl ReflectableUuid {
    pub fn generate() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self(uuid)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_timestamp() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_timestamp() -> f64 {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis() as f64
}

pub fn load_doc_to_memory(
    doc_id: ReflectableUuid,
    app_state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
) {
    if app_state.docs.contains_key(&doc_id) {
        return;
    }
    if let Ok(docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
        if docs.contains_key(&doc_id) {
            let keys = app_state.docs.keys().cloned().collect::<Vec<_>>();
            while (app_state.docs.len() as i32) >= MAX_SAVED_DOCS_IN_MEMORY {
                app_state.docs.remove(&keys[0]);
            }
            app_state
                .docs
                .insert(doc_id, docs.get(&doc_id).unwrap().clone());
        } else {
            panic!("Document not found in pkv");
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub github_access_token: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_config_file() -> Option<Config> {
    let home_dir = std::env::var("HOME").ok()?;
    let config_file_path = PathBuf::from(&home_dir).join(".velo.toml");
    let config_str = fs::read_to_string(config_file_path).ok()?;
    let config_value: toml::Value = toml::from_str(&config_str).ok()?;
    let mut config = Config::default();
    if let Some(token) = config_value.get("github_access_token") {
        if let Some(token_str) = token.as_str() {
            config.github_access_token = Some(token_str.to_owned());
        }
    }
    Some(config)
}

pub fn convert_from_val_px(x: Val) -> f32 {
    match x {
        Val::Px(x) => x,
        _ => 0.,
    }
}

pub fn to_cosmic_text_pos(pos: TextPos) -> CosmicTextPos {
    match pos {
        TextPos::Center => CosmicTextPos::Center,
        TextPos::TopLeft => CosmicTextPos::TopLeft,
    }
}
