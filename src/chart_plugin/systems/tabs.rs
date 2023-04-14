use std::collections::VecDeque;

use bevy::prelude::*;

use uuid::Uuid;

use crate::{AppState, LoadRequest, SaveRequest, Tab};

use super::ui_helpers::{
    spawn_modal, AddTab, DeleteTab, ModalEntity, ReflectableUuid, RenameTab, SelectedTab,
    SelectedTabTextInput,
};

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
                    if selected_tab.id == tab.id {
                        if tab.is_active {
                            bg_color.0 = Color::ALICE_BLUE;
                        } else {
                            bg_color.0 =
                                Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                        }
                    }
                }
            }
            Interaction::None => {
                let current_document = state.current_document.unwrap();
                for tab in state.docs.get_mut(&current_document).unwrap().tabs.iter() {
                    if selected_tab.id == tab.id {
                        if tab.is_active {
                            bg_color.0 = Color::ALICE_BLUE;
                        } else {
                            bg_color.0 =
                                Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
                        }
                    }
                }
            }
        }
    }
}

pub fn add_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AddTab>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
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
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn tab_keyboard_input_system(
    mut query: Query<(&mut Text, &SelectedTabTextInput), With<SelectedTabTextInput>>,
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut deleting: Local<bool>,
) {
    for (mut text, tab_input) in &mut query.iter_mut() {
        if Some(tab_input.id) == state.tab_to_edit {
            if input.just_pressed(KeyCode::Return) {
                state.tab_to_edit = None;
                continue;
            }
            let mut str = text.sections[0].value.clone();
            if input.just_pressed(KeyCode::Back) {
                *deleting = true;
                str.pop();
            } else if input.just_released(KeyCode::Back) {
                *deleting = false;
            } else {
                for ev in char_evr.iter() {
                    if *deleting {
                        str.pop();
                    } else {
                        str = format!("{}{}", text.sections[0].value, ev.char);
                    }
                }
            }
            text.sections[0].value = str;
            let current_document = state.current_document.unwrap();
            let tab = state
                .docs
                .get_mut(&current_document)
                .unwrap()
                .tabs
                .iter_mut()
                .find(|x| x.id == tab_input.id)
                .unwrap();
            tab.name = text.sections[0].value.clone();
        }
    }
}

pub fn rename_tab_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RenameTab>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
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
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn delete_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DeleteTab>),
    >,
    mut state: ResMut<AppState>,
    _asset_server: Res<AssetServer>,
) {
    let font = state.font.as_ref().unwrap().clone();
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                state.modal_id = Some(id);
                state.entity_to_edit = None;
                let entity = spawn_modal(&mut commands, font.clone(), id, ModalEntity::Tab);
                commands.entity(state.main_panel.unwrap()).add_child(entity);
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}
