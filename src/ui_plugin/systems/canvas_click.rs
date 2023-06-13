use bevy::prelude::*;

use super::{ui_helpers::Background, NodeInteractionEvent, UiState};

pub fn canvas_click(
    mut ui_state: ResMut<UiState>,
    background_query: Query<With<Background>>,
    mut node_interaction_events: EventReader<NodeInteractionEvent>,
) {
    for event in node_interaction_events.iter() {
        if let Ok(_) = background_query.get(event.entity) {
            if event.node_interaction_type == crate::ui_plugin::NodeInteractionType::LeftClick {
                ui_state.entity_to_edit = None;
            }
        }
    }
}
