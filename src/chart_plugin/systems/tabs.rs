use std::collections::VecDeque;

use bevy::prelude::*;

use uuid::Uuid;

use super::ui_helpers::{spawn_modal, AddTab, DeleteTab, ModalEntity, RenameTab, SelectedTab};
use crate::components::Tab;
use crate::resources::{AppState, LoadRequest, SaveRequest};
use crate::utils::ReflectableUuid;

pub fn selected_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SelectedTab),
        (Changed<Interaction>, With<SelectedTab>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, mut bg_color, selected_tab) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let current_document = state.current_document.unwrap();
                let tabs = state
                    .docs
                    .get_mut(&current_document)
                    .unwrap()
                    .tabs
                    .iter_mut();
                for tab in tabs {
                    if tab.is_active {
                        commands.insert_resource(SaveRequest {
                            doc_id: None,
                            tab_id: Some(tab.id),
                        });
                    }
                    tab.is_active = tab.id == selected_tab.id;
                }

                commands.insert_resource(LoadRequest {
                    doc_id: None,
                    drop_last_checkpoint: false,
                });
            }
            Interaction::Hovered => {
                let current_document = state.current_document.unwrap();
                for tab in state.docs.get_mut(&current_document).unwrap().tabs.iter() {
                    if selected_tab.id == tab.id && tab.is_active {
                        bg_color.0 = Color::ALICE_BLUE;
                    }
                }
            }
            Interaction::None => {
                let current_document = state.current_document.unwrap();
                for tab in state.docs.get_mut(&current_document).unwrap().tabs.iter() {
                    if selected_tab.id == tab.id && tab.is_active {
                        bg_color.0 = Color::ALICE_BLUE;
                    }
                }
            }
        }
    }
}

pub fn add_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddTab>)>,
    mut state: ResMut<AppState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let tab_id = ReflectableUuid(Uuid::new_v4());
                let current_document = state.current_document.unwrap();
                let tabs = &mut state.docs.get_mut(&current_document).unwrap().tabs;
                for tab in tabs.iter_mut() {
                    if tab.is_active {
                        commands.insert_resource(SaveRequest {
                            doc_id: None,
                            tab_id: Some(tab.id),
                        });
                    }
                    tab.is_active = false;
                }
                let tabs_len = tabs.len();
                tabs.push(Tab {
                    id: tab_id,
                    name: "Tab ".to_string() + &(tabs_len + 1).to_string(),
                    checkpoints: VecDeque::new(),
                    is_active: true,
                });
                commands.insert_resource(LoadRequest {
                    doc_id: None,
                    drop_last_checkpoint: false,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn rename_tab_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RenameTab>)>,
    mut state: ResMut<AppState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.entity_to_edit = None;
                state.doc_to_edit = None;
                let current_document = state.current_document.unwrap();
                let tab = state
                    .docs
                    .get_mut(&current_document)
                    .unwrap()
                    .tabs
                    .iter()
                    .find(|x| x.is_active)
                    .unwrap();
                state.tab_to_edit = Some(tab.id);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn delete_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeleteTab>)>,
    mut state: ResMut<AppState>,
) {
    let font = state.font.as_ref().unwrap().clone();
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                state.modal_id = Some(id);
                state.entity_to_edit = None;
                let entity = spawn_modal(&mut commands, font.clone(), id, ModalEntity::Tab);
                commands.entity(state.main_panel.unwrap()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
