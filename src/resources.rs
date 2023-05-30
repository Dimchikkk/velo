use crate::components::Doc;
#[cfg(not(target_arch = "wasm32"))]
use crate::ui_plugin::SearchIndexState;
use crate::utils::ReflectableUuid;
use bevy::prelude::*;
use bevy_cosmic_edit::CosmicFont;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Resource, Default)]
pub struct AppState {
    pub current_document: Option<ReflectableUuid>,
    pub docs: HashMap<ReflectableUuid, Doc>,
    pub github_token: Option<String>,
    #[cfg(not(target_arch = "wasm32"))]
    pub search_index: Option<SearchIndexState>,
    pub doc_list_ui: HashSet<ReflectableUuid>,
}

#[derive(Resource, Debug)]
pub struct SaveDocRequest {
    pub doc_id: ReflectableUuid,
    pub path: Option<PathBuf>, // Save current document to file
}

#[derive(Resource, Debug)]
pub struct SaveTabRequest {
    pub doc_id: ReflectableUuid,
    pub tab_id: ReflectableUuid,
}

#[derive(Resource, Debug)]
pub struct LoadDocRequest {
    pub doc_id: ReflectableUuid,
}

#[derive(Resource, Debug)]
pub struct LoadTabRequest {
    pub doc_id: ReflectableUuid,
    pub tab_id: ReflectableUuid,
    pub drop_last_checkpoint: bool, // Useful for undo functionality
}

#[derive(Resource, Default)]
pub struct FontSystemState(pub Option<Handle<CosmicFont>>);
