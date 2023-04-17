use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use uuid::Uuid;

use crate::{get_timestamp, AppState, LoadRequest, SaveRequest, StaticState, Tab, UiState};

use super::ui_helpers::{
    spawn_modal, AddTab, DeleteTab, ModalEntity, ReflectableUuid, SelectedTab, SelectedTabContainer,
};

pub fn selected_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SelectedTab, &Parent),
        (Changed<Interaction>, With<SelectedTab>),
    >,
    mut selected_tab_query: Query<&mut BackgroundColor, With<SelectedTabContainer>>,
    mut state: ResMut<AppState>,
) {
    for (interaction, selected_tab, parent) in &mut interaction_query {
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
                        if tab.id == selected_tab.id {
                            return;
                        }
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
                        let mut bg_color = selected_tab_query.get_mut(parent.get()).unwrap();
                        bg_color.0 = Color::ALICE_BLUE;
                    }
                }
            }
            Interaction::None => {
                let current_document = state.current_document.unwrap();
                for tab in state.docs.get_mut(&current_document).unwrap().tabs.iter() {
                    if selected_tab.id == tab.id && tab.is_active {
                        let mut bg_color = selected_tab_query.get_mut(parent.get()).unwrap();
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
    mut interaction_query: Query<
        (&Interaction, &SelectedTab),
        (Changed<Interaction>, With<SelectedTab>),
    >,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
) {
    for (interaction, item) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let now_ms = get_timestamp();
                if double_click.1 == Some(item.id)
                    && Duration::from_millis(now_ms as u64) - double_click.0
                        < Duration::from_millis(500)
                {
                    *ui_state = UiState::default();
                    let current_document = app_state.current_document.unwrap();
                    let tab = app_state
                        .docs
                        .get_mut(&current_document)
                        .unwrap()
                        .tabs
                        .iter()
                        .find(|x| x.is_active)
                        .unwrap();
                    ui_state.tab_to_edit = Some(tab.id);
                    *double_click = (Duration::from_secs(0), None);
                } else {
                    *double_click = (Duration::from_millis(now_ms as u64), Some(item.id));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn delete_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeleteTab>)>,
    static_state: ResMut<StaticState>,
    mut ui_state: ResMut<UiState>,
) {
    let font = static_state.font.as_ref().unwrap().clone();
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                *ui_state = UiState::default();
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(&mut commands, font.clone(), id, ModalEntity::Tab);
                commands
                    .entity(static_state.main_panel.unwrap())
                    .add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
