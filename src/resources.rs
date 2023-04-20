use crate::components::Doc;
use crate::utils::ReflectableUuid;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AppState {
    pub current_document: Option<ReflectableUuid>,
    pub docs: HashMap<ReflectableUuid, Doc>,
}

#[derive(Resource, Debug)]
pub struct SaveRequest {
    pub doc_id: Option<ReflectableUuid>, // None means current doc
    pub tab_id: Option<ReflectableUuid>, // None means save to active tab
}

#[derive(Resource, Debug)]
pub struct LoadRequest {
    pub doc_id: Option<ReflectableUuid>, // None means current doc
    pub drop_last_checkpoint: bool,      // Useful for undo functionality
}

#[derive(Resource)]
pub struct FontHandle(pub Handle<Font>);