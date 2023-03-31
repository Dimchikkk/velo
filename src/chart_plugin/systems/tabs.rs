use std::collections::VecDeque;

use bevy::prelude::*;

use serde_json::json;
use uuid::Uuid;

use crate::{AppState, LoadRequest, Tab};

use super::ui_helpers::{AddTab, ReflectableUuid, SelectedTab};

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
                    tab.is_active = tab.id == selected_tab.id;
                }
                commands.insert_resource(LoadRequest {
                    path: None,
                    drop_last: false,
                });
            }
            Interaction::Hovered => {
                for tab in state.tabs.iter() {
                    if selected_tab.id == tab.id  {
                        if tab.is_active {
                            bg_color.0= Color::ALICE_BLUE;
                        } else {
                            bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                        }
                    }
                }
            }
            Interaction::None => {
                for tab in state.tabs.iter() {
                    if selected_tab.id == tab.id  {
                        if tab.is_active {
                            bg_color.0= Color::ALICE_BLUE;
                        } else {
                            bg_color.0= Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
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
                    drop_last: false,
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
