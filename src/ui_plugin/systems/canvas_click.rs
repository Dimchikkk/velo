use bevy::prelude::*;

use super::{ui_helpers::MainPanel, UiState};

pub fn canvas_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainPanel>)>,
    mut ui_state: ResMut<UiState>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            ui_state.entity_to_edit = None;
        }
    }
}
