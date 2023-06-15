use bevy::{prelude::*, window::PrimaryWindow};

use super::{ui_helpers::MainPanel, UiState};

pub fn canvas_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainPanel>)>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            ui_state.entity_to_edit = None;
        }
        if *interaction == Interaction::Hovered {
            primary_window.cursor.icon = CursorIcon::default();
        }
    }
}
