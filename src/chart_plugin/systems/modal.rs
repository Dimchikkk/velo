use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_pkv::PkvStore;
use linkify::{LinkFinder, LinkKind};

use super::ui_helpers::{DocListItemContainer, ModalCancel, ModalConfirm, ModalTop};
use super::{add_list_item, CommChannels, DocList, EditableText, ModalAction};
use crate::components::Doc;
use crate::resources::{AppState, LoadRequest, SaveRequest};
use crate::utils::ReflectableUuid;
use crate::UiState;

pub fn cancel_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalCancel),
        (Changed<Interaction>, With<ModalCancel>),
    >,
    mut state: ResMut<UiState>,
    query: Query<(Entity, &ModalTop), With<ModalTop>>,
) {
    for (interaction, path_modal_cancel) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query.iter() {
                if path_modal_cancel.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                    state.modal_id = None;
                }
            }
        }
    }
}

fn delete_doc(
    app_state: &mut ResMut<AppState>,
    commands: &mut Commands,
    pkv: &mut ResMut<PkvStore>,
    query_container: &mut Query<(Entity, &DocListItemContainer), With<DocListItemContainer>>,
) {
    let current_document = app_state.current_document.unwrap();
    let id_to_remove = current_document;
    for (entity, button) in query_container.iter_mut() {
        if button.id == id_to_remove {
            commands.entity(entity).despawn_recursive();
        }
    }
    app_state.docs.remove(&current_document);
    for (_, button) in query_container.iter_mut() {
        if button.id != id_to_remove {
            app_state.current_document = Some(button.id);
            break;
        }
    }
    commands.insert_resource(LoadRequest {
        doc_id: None,
        drop_last_checkpoint: false,
    });
    remove_from_storage(pkv, id_to_remove, app_state.current_document.unwrap());
}

fn delete_tab(app_state: &mut ResMut<AppState>, commands: &mut Commands) {
    let current_document = app_state.current_document.unwrap();
    let index = app_state
        .docs
        .get_mut(&current_document)
        .unwrap()
        .tabs
        .iter()
        .position(|x| x.is_active)
        .unwrap();
    app_state
        .docs
        .get_mut(&current_document)
        .unwrap()
        .tabs
        .remove(index);
    let mut last_tab = app_state
        .docs
        .get_mut(&current_document)
        .unwrap()
        .tabs
        .last_mut()
        .unwrap();
    last_tab.is_active = true;
    commands.insert_resource(LoadRequest {
        doc_id: None,
        drop_last_checkpoint: false,
    });
}

pub fn load_doc_handler(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    doc_list_query: Query<Entity, With<DocList>>,
    comm_channels: Res<CommChannels>,
) {
    if comm_channels.rx.is_empty() {
        return;
    }
    let r = comm_channels
        .rx
        .try_recv()
        .expect("Failed to receive document string");
    let import_document: Doc = serde_json::from_str(&r).expect("Failed to deserialize document");
    if app_state.docs.contains_key(&import_document.id) {
        return;
    }
    commands.insert_resource(SaveRequest {
        doc_id: Some(app_state.current_document.unwrap()),
        tab_id: None,
        path: None,
    });
    let button = add_list_item(
        &mut commands,
        import_document.id,
        import_document.name.clone(),
    );
    commands.entity(doc_list_query.single()).add_child(button);

    app_state
        .docs
        .insert(import_document.id, import_document.clone());
    app_state.current_document = Some(import_document.id);
    commands.insert_resource(LoadRequest {
        drop_last_checkpoint: false,
        doc_id: Some(import_document.id),
    });
}

pub fn confirm_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalConfirm),
        (Changed<Interaction>, With<ModalConfirm>),
    >,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    query_top: Query<(Entity, &ModalTop), With<ModalTop>>,
    mut query_container: Query<(Entity, &DocListItemContainer), With<DocListItemContainer>>,
    mut pkv: ResMut<PkvStore>,
    input: Res<Input<KeyCode>>,
    mut query_path: Query<(&Text, &EditableText), With<EditableText>>,
    comm_channels: Res<CommChannels>,
) {
    for (interaction, path_modal_confirm) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query_top.iter() {
                if path_modal_confirm.id == path_modal_top.id {
                    for (text, editable_text) in query_path.iter_mut() {
                        if editable_text.id == path_modal_top.id {
                            match path_modal_confirm.action {
                                ModalAction::SaveToFile => {
                                    commands.insert_resource(SaveRequest {
                                        doc_id: None,
                                        tab_id: None,
                                        path: Some(PathBuf::from(text.sections[0].value.trim())),
                                    });
                                    break;
                                }
                                ModalAction::LoadFromFile => {
                                    if let Ok(path) =
                                        canonicalize(PathBuf::from(text.sections[0].value.trim()))
                                    {
                                        let json = std::fs::read_to_string(path)
                                            .expect("Error reading document from file");
                                        let cc = comm_channels.tx.clone();
                                        cc.try_send(json).unwrap()
                                    }
                                }
                                ModalAction::LoadFromUrl => {
                                    let pool = IoTaskPool::get();
                                    let url = text.sections[0].value.trim();
                                    let mut finder = LinkFinder::new();
                                    finder.kinds(&[LinkKind::Url]);
                                    let links: Vec<_> = finder.links(url).collect();
                                    if links.len() == 1 {
                                        let url = links.first().unwrap().as_str().to_owned();
                                        let cc = comm_channels.tx.clone();
                                        let task = pool.spawn(async move {
                                            let request = ehttp::Request::get(url);
                                            ehttp::fetch(request, move |result| {
                                                let json_string = result.unwrap().text().unwrap();
                                                cc.try_send(json_string).unwrap();
                                            });
                                        });
                                        task.detach();
                                    }
                                }
                                ModalAction::DeleteDocument => {}
                                ModalAction::DeleteTab => {}
                            }
                        }
                    }
                    match path_modal_confirm.action {
                        ModalAction::SaveToFile => {}
                        ModalAction::LoadFromFile => {}
                        ModalAction::LoadFromUrl => {}
                        ModalAction::DeleteDocument => {
                            delete_doc(
                                &mut app_state,
                                &mut commands,
                                &mut pkv,
                                &mut query_container,
                            );
                        }
                        ModalAction::DeleteTab => delete_tab(&mut app_state, &mut commands),
                    }
                }
                commands.entity(entity).despawn_recursive();
                ui_state.modal_id = None;
            }
        }
    }
    if input.just_pressed(KeyCode::Return) {
        for (entity, path_modal_top) in query_top.iter() {
            if Some(path_modal_top.id) == ui_state.modal_id {
                match path_modal_top.action {
                    ModalAction::SaveToFile => todo!(),
                    ModalAction::LoadFromFile => todo!(),
                    ModalAction::LoadFromUrl => todo!(),
                    ModalAction::DeleteDocument => {
                        delete_doc(
                            &mut app_state,
                            &mut commands,
                            &mut pkv,
                            &mut query_container,
                        );
                    }
                    ModalAction::DeleteTab => delete_tab(&mut app_state, &mut commands),
                }
                commands.entity(entity).despawn_recursive();
                ui_state.modal_id = None;
            }
        }
    }
}

fn remove_from_storage(
    pkv: &mut ResMut<PkvStore>,
    id_to_remove: ReflectableUuid,
    new_id: ReflectableUuid,
) {
    if let Ok(mut docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
        if docs.remove(&id_to_remove).is_some() {
            pkv.set("docs", &docs).unwrap();
        }
    }
    if let Ok(mut tags) = pkv.get::<HashMap<ReflectableUuid, Vec<String>>>("tags") {
        if tags.remove(&id_to_remove).is_some() {
            pkv.set("tags", &tags).unwrap();
        }
    }
    if let Ok(mut tags) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
        if tags.remove(&id_to_remove).is_some() {
            pkv.set("names", &tags).unwrap();
        }
    }
    if let Ok(last_saved) = pkv.get::<ReflectableUuid>("last_saved") {
        if last_saved == id_to_remove {
            pkv.set("last_saved", &new_id).unwrap();
        }
    }
}
