use crate::canvas::arrow::components::{ArrowConnect, ArrowType};
use crate::chart_plugin::ResizeMarker;
use crate::components::Doc;
use crate::utils::ReflectableUuid;
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Resource, Default)]
pub struct AppState {
    pub font: Option<Handle<Font>>,
    pub modal_id: Option<ReflectableUuid>,
    pub main_panel: Option<Entity>,
    pub arrow_type: ArrowType,
    pub entity_to_edit: Option<ReflectableUuid>,
    pub tab_to_edit: Option<ReflectableUuid>,
    pub doc_to_edit: Option<ReflectableUuid>,
    pub hold_entity: Option<ReflectableUuid>,
    pub entity_to_resize: Option<(ReflectableUuid, ResizeMarker)>,
    pub arrow_to_draw_start: Option<ArrowConnect>,
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
