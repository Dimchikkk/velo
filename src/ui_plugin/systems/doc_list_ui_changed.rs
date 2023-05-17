use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{resources::AppState, ui_plugin::ui_helpers::add_list_item, utils::ReflectableUuid};

use super::{
    ui_helpers::{DeleteDoc, DocList, DocListItemContainer},
    UpdateDeleteDocBtnEvent,
};

pub fn doc_list_del_button_update(
    app_state: Res<AppState>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
    mut event_reader: EventReader<UpdateDeleteDocBtnEvent>,
) {
    for _ in event_reader.iter() {
        for (mut visibility, doc) in delete_doc.iter_mut() {
            if Some(doc.id) == app_state.current_document {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn doc_list_ui_changed(
    mut commands: Commands,
    app_state: Res<AppState>,
    mut last_doc_list: Local<HashSet<ReflectableUuid>>,
    mut doc_list_query: Query<Entity, With<DocList>>,
    asset_server: Res<AssetServer>,
    pkv: Res<PkvStore>,
    mut query_container: Query<Entity, With<DocListItemContainer>>,
    mut event_writer: EventWriter<UpdateDeleteDocBtnEvent>,
) {
    if app_state.is_changed() && app_state.doc_list_ui != *last_doc_list {
        // Think about re-using UI elements instead of destroying and re-creating them
        for entity in query_container.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        let doc_list = doc_list_query.single_mut();
        let mut doc_tuples: Vec<(String, ReflectableUuid)> = app_state
            .doc_list_ui
            .iter()
            .map(|doc_id| {
                let doc_name = get_doc_name(*doc_id, &pkv, &app_state);
                (doc_name, *doc_id)
            })
            .collect();
        // Sort the tuples alphabetically based on doc_name
        doc_tuples.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));
        for (doc_name, doc_id) in doc_tuples {
            let doc_list_item = add_list_item(&mut commands, &asset_server, doc_id, doc_name);
            commands.entity(doc_list).add_child(doc_list_item);
        }
        event_writer.send(UpdateDeleteDocBtnEvent);
        *last_doc_list = app_state.doc_list_ui.clone();
    }
}

pub fn get_doc_name(
    doc_id: ReflectableUuid,
    pkv: &Res<PkvStore>,
    app_state: &Res<AppState>,
) -> String {
    if let Some(doc) = app_state.docs.get(&doc_id) {
        return doc.name.clone();
    }
    if let Ok(names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
        if let Some(name) = names.get(&doc_id) {
            return name.clone();
        }
    }

    "Unknown".to_string()
}
