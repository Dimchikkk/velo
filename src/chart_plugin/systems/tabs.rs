use std::collections::VecDeque;

use bevy::prelude::*;

use serde_json::json;
use uuid::Uuid;

use crate::{AppState, LoadRequest, SaveRequest, Tab};

use super::ui_helpers::{
    AddTab, DeleteTab, ReflectableUuid, RenameTab, SelectedTab, SelectedTabTextInput,
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
                for tab in state.tabs.iter_mut() {
                    if tab.is_active {
                        commands.insert_resource(SaveRequest {
                            path: None,
                            tab_id: Some(tab.id),
                        });
                    }
                    tab.is_active = tab.id == selected_tab.id;
                }

                commands.insert_resource(LoadRequest {
                    path: None,
                    drop_last_checkpoint: false,
                });
            }
            Interaction::Hovered => {
                for tab in state.tabs.iter() {
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
                for tab in state.tabs.iter() {
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
                let tabs_len = state.tabs.len();
                for tab in state.tabs.iter_mut() {
                    if tab.is_active {
                        commands.insert_resource(SaveRequest {
                            path: None,
                            tab_id: Some(tab.id),
                        });
                    }
                    tab.is_active = false;
                }
                let mut checkpoints = VecDeque::new();
                checkpoints.push_back(
                    json!({
                        "nodes": [],
                        "arrows": [],
                        "images": {},
                    })
                    .to_string(),
                );
                state.tabs.push(Tab {
                    id: tab_id,
                    name: "Tab ".to_string() + &(tabs_len + 1).to_string(),
                    checkpoints,
                    is_active: true,
                });
                commands.insert_resource(LoadRequest {
                    path: None,
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
) {
    for (mut text, tab_input) in &mut query.iter_mut() {
        if Some(tab_input.id) == state.tab_to_edit {
            if input.just_pressed(KeyCode::Back) {
                let mut str = text.sections[0].value.clone();
                str.pop();
                text.sections[0].value = str;
            } else {
                for ev in char_evr.iter() {
                    text.sections[0].value = format!("{}{}", text.sections[0].value, ev.char);
                }
            }
            let tab = state
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
                let tab = state.tabs.iter().find(|x| x.is_active).unwrap();
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
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if state.tab_to_edit.is_some() {
                    return;
                }
                if state.tabs.len() > 1 {
                    let index = state.tabs.iter().position(|x| x.is_active).unwrap();
                    state.tabs.remove(index);
                    let mut last_tab = state.tabs.last_mut().unwrap();
                    last_tab.is_active = true;
                    commands.insert_resource(LoadRequest {
                        path: None,
                        drop_last_checkpoint: false,
                    });
                }
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
