use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    ui_helpers::{MainPanel, RawText},
    NodeInteractionEvent, UiState,
};

pub fn canvas_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainPanel>)>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut node_interaction_events: EventReader<NodeInteractionEvent>,
    raw_text: Query<With<RawText>>,
) {
    let mut primary_window = windows.single_mut();
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            for event in node_interaction_events.iter() {
                if raw_text.get(event.entity).is_ok() {
                    return;
                }
            }
            ui_state.entity_to_edit = None;
        }
        if *interaction == Interaction::Hovered {
            primary_window.cursor.icon = CursorIcon::default();
        }
    }
}
