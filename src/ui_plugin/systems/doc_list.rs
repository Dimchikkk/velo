use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_cosmic_edit::CosmicFont;

use super::ui_helpers::ScrollingList;
use crate::{resources::FontSystemState, themes::Theme, ui_plugin::ui_helpers::DocListItemButton};

use crate::resources::{AppState, LoadDocRequest, SaveDocRequest};

use std::collections::{HashMap, HashSet};

use bevy_pkv::PkvStore;

use crate::{ui_plugin::ui_helpers::add_list_item, utils::ReflectableUuid};

use super::{
    ui_helpers::{DeleteDoc, DocList, DocListItemContainer},
    UpdateDeleteDocBtn,
};

pub fn list_item_click(
    mut interaction_query: Query<
        (&Interaction, &DocListItemButton),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut state: ResMut<AppState>,
    mut commands: Commands,
) {
    for (interaction, doc_list_item) in &mut interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if Some(doc_list_item.id) != state.current_document {
                    commands.insert_resource(SaveDocRequest {
                        doc_id: state.current_document.unwrap(),
                        path: None,
                    });
                    state.current_document = Some(doc_list_item.id);
                    commands.insert_resource(LoadDocRequest {
                        doc_id: doc_list_item.id,
                    });
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn mouse_scroll_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub fn doc_list_del_button_update(
    app_state: Res<AppState>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
    mut event_reader: EventReader<UpdateDeleteDocBtn>,
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
    mut event_writer: EventWriter<UpdateDeleteDocBtn>,
    theme: Res<Theme>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor() as f32;
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
            let doc_list_item = add_list_item(
                &mut commands,
                &mut cosmic_fonts,
                font_system_state.0.clone().unwrap(),
                &theme,
                &asset_server,
                doc_id,
                doc_name,
                scale_factor,
            );
            commands.entity(doc_list).add_child(doc_list_item);
        }
        event_writer.send(UpdateDeleteDocBtn);
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
