use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_cosmic_edit::{CosmicEdit, CosmicFont};
use cosmic_text::{Cursor, Edit};

use super::ui_helpers::{spawn_modal, AddTab, DeleteTab, TabButton};
use super::MainPanel;
use crate::components::Tab;
use crate::resources::{AppState, FontSystemState, LoadDocRequest, LoadTabRequest, SaveTabRequest};
use crate::themes::Theme;
use crate::utils::{bevy_color_to_cosmic, get_timestamp, ReflectableUuid};
use crate::UiState;

pub fn select_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &TabButton),
        (Changed<Interaction>, With<TabButton>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, selected_tab) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let current_document = state.current_document.unwrap();
                for tab in state
                    .docs
                    .get_mut(&current_document)
                    .unwrap()
                    .tabs
                    .iter_mut()
                {
                    if tab.is_active && tab.id == selected_tab.id {
                        return;
                    }
                    if tab.is_active {
                        commands.insert_resource(SaveTabRequest {
                            tab_id: tab.id,
                            doc_id: current_document,
                        });
                    }
                }
                for tab in state
                    .docs
                    .get_mut(&current_document)
                    .unwrap()
                    .tabs
                    .iter_mut()
                {
                    tab.is_active = tab.id == selected_tab.id;
                }

                commands.insert_resource(LoadTabRequest {
                    doc_id: current_document,
                    tab_id: selected_tab.id,
                    drop_last_checkpoint: false,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn add_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddTab>)>,
    mut app_state: ResMut<AppState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let tab_id = ReflectableUuid::generate();
                let current_document = app_state.current_document.unwrap();
                let tabs = &mut app_state.docs.get_mut(&current_document).unwrap().tabs;
                for tab in tabs.iter_mut() {
                    if tab.is_active {
                        commands.insert_resource(SaveTabRequest {
                            tab_id: tab.id,
                            doc_id: current_document,
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
                    z_index: 10.,
                });
                commands.insert_resource(LoadDocRequest {
                    doc_id: app_state.current_document.unwrap(),
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn rename_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &TabButton, Entity, &mut CosmicEdit),
        (Changed<Interaction>, With<TabButton>),
    >,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
    theme: Res<Theme>,
) {
    for (interaction, item, entity, mut cosmic_edit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let now_ms = get_timestamp();
                if double_click.1 == Some(item.id)
                    && Duration::from_millis(now_ms as u64) - double_click.0
                        < Duration::from_millis(500)
                {
                    *ui_state = UiState::default();
                    commands.insert_resource(bevy_cosmic_edit::ActiveEditor {
                        entity: Some(entity),
                    });
                    cosmic_edit.readonly = false;
                    let current_cursor = cosmic_edit.editor.cursor();
                    let new_cursor = Cursor::new_with_color(
                        current_cursor.line,
                        current_cursor.index,
                        bevy_color_to_cosmic(theme.font),
                    );
                    cosmic_edit.editor.set_cursor(new_cursor);
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
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single();
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let id: ReflectableUuid = ReflectableUuid::generate();
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                let current_document = app_state.current_document.unwrap();
                let tabs_len = app_state
                    .docs
                    .get_mut(&current_document)
                    .unwrap()
                    .tabs
                    .len();
                if tabs_len < 2 {
                    return;
                }
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    window,
                    id,
                    super::ModalAction::DeleteTab,
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
