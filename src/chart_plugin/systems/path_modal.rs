use bevy::prelude::*;

use crate::{AppState, LoadRequest, UpdateListHighlight};

use super::ui_helpers::{DocListItemButton, ModalCancel, ModalConfirm, ModalEntity, ModalTop};

pub fn cancel_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalCancel),
        (Changed<Interaction>, With<ModalCancel>),
    >,
    mut state: ResMut<AppState>,
    query: Query<(Entity, &ModalTop), With<ModalTop>>,
) {
    for (interaction, path_modal_cancel) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query.iter() {
                if path_modal_cancel.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                    state.path_modal_id = None;
                }
            }
        }
    }
}

pub fn confirm_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalConfirm),
        (Changed<Interaction>, With<ModalConfirm>),
    >,
    mut state: ResMut<AppState>,
    query_top: Query<(Entity, &ModalTop), With<ModalTop>>,
    mut query_button: Query<(Entity, &DocListItemButton), With<DocListItemButton>>,
    mut events: EventWriter<UpdateListHighlight>,
) {
    for (interaction, path_modal_confirm) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query_top.iter() {
                if path_modal_confirm.id == path_modal_top.id {
                    let current_document = state.current_document.unwrap();
                    if path_modal_confirm.delete == ModalEntity::Tab
                        && state.docs.get_mut(&current_document).unwrap().tabs.len() > 1
                    {
                        let index = state
                            .docs
                            .get_mut(&current_document)
                            .unwrap()
                            .tabs
                            .iter()
                            .position(|x| x.is_active)
                            .unwrap();
                        state
                            .docs
                            .get_mut(&current_document)
                            .unwrap()
                            .tabs
                            .remove(index);
                        let mut last_tab = state
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
                    if path_modal_confirm.delete == ModalEntity::Document && state.docs.len() > 1 {
                        let id_to_remove = current_document;
                        for (entity, button) in query_button.iter_mut() {
                            if button.id == id_to_remove {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        state.docs.remove(&current_document);
                        for (_, button) in query_button.iter_mut() {
                            if button.id != id_to_remove {
                                state.current_document = Some(button.id);
                                break;
                            }
                        }
                        commands.insert_resource(LoadRequest {
                            doc_id: None,
                            drop_last_checkpoint: false,
                        });
                        events.send(UpdateListHighlight);
                    }
                    commands.entity(entity).despawn_recursive();
                    state.path_modal_id = None;
                }
            }
        }
    }
}

pub fn modal_keyboard_input_system(
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    query_top: Query<(Entity, &ModalTop), With<ModalTop>>,
    mut commands: Commands,
    mut query_button: Query<(Entity, &DocListItemButton), With<DocListItemButton>>,
    mut events: EventWriter<UpdateListHighlight>,
) {
    if input.just_pressed(KeyCode::Return) {
        for (entity, path_modal_top) in query_top.iter() {
            if Some(path_modal_top.id) == state.path_modal_id {
                let current_document = state.current_document.unwrap();
                if path_modal_top.delete == ModalEntity::Tab
                    && state.docs.get_mut(&current_document).unwrap().tabs.len() > 1
                {
                    let index = state
                        .docs
                        .get_mut(&current_document)
                        .unwrap()
                        .tabs
                        .iter()
                        .position(|x| x.is_active)
                        .unwrap();
                    state
                        .docs
                        .get_mut(&current_document)
                        .unwrap()
                        .tabs
                        .remove(index);
                    let mut last_tab = state
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
                if path_modal_top.delete == ModalEntity::Document && state.docs.len() > 1 {
                    let id_to_remove = current_document;
                    for (entity, button) in query_button.iter_mut() {
                        if button.id == id_to_remove {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                    state.docs.remove(&current_document);
                    for (_, button) in query_button.iter_mut() {
                        if button.id != id_to_remove {
                            state.current_document = Some(button.id);
                            break;
                        }
                    }
                    commands.insert_resource(LoadRequest {
                        doc_id: None,
                        drop_last_checkpoint: false,
                    });
                    events.send(UpdateListHighlight);
                }
                commands.entity(entity).despawn_recursive();
                state.path_modal_id = None;
            }
        }
    }
}
