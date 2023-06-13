use bevy::{prelude::*, window::PrimaryWindow};

use crate::canvas::arrow::components::ArrowConnect;

use super::{
    ui_helpers::{RawText, ResizeMarker},
    NodeInteractionEvent, UiState,
};

pub fn my_test_system(
    mut node_interaction_events: EventReader<NodeInteractionEvent>,
    resize_marker_query: Query<&ResizeMarker, With<ResizeMarker>>,
    arrow_connect_query: Query<&ArrowConnect, With<ArrowConnect>>,
    raw_text_query: Query<With<RawText>>,
) {
    for event in node_interaction_events.iter() {
        // eprintln!("NodeInteractionType: {:?}", event.node_interaction_type);
        // if let Ok(marker) = resize_marker_query.get(event.entity) {
        //     println!("ResizeMarker: {:?}", marker);
        // }
        // if let Ok(arrow) = arrow_connect_query.get(event.entity) {
        //     println!("ArrowConnect: {:?}", arrow);
        // }
        // if let Ok(_) = raw_text_query.get(event.entity) {
        //     println!("RawText");
        // }
    }
}

pub fn set_focused_entity(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut node_interaction_events: EventReader<NodeInteractionEvent>,
    mut ui_state: ResMut<UiState>,
    velo: Query<&RawText, With<RawText>>,
) {
    let mut primary_window = windows.single_mut();
    for event in node_interaction_events.iter() {
        if let Ok(velo_node) = velo.get(event.entity) {
            match event.node_interaction_type {
                crate::ui_plugin::NodeInteractionType::Hover => {
                    if ui_state.hold_entity.is_none() && ui_state.entity_to_edit.is_none() {
                        primary_window.cursor.icon = CursorIcon::Hand;
                    }
                    if ui_state.entity_to_edit.is_some() {
                        primary_window.cursor.icon = CursorIcon::Text;
                    }
                }
                crate::ui_plugin::NodeInteractionType::LeftClick => {}
                crate::ui_plugin::NodeInteractionType::LeftDoubleClick => {
                    *ui_state = UiState::default();
                    ui_state.entity_to_edit = Some(velo_node.id);
                }
                crate::ui_plugin::NodeInteractionType::LeftMouseHoldAndDrag => {
                    if ui_state.entity_to_edit.is_none() {
                        ui_state.hold_entity = Some(velo_node.id);
                        primary_window.cursor.icon = CursorIcon::Move;
                    }
                }
                crate::ui_plugin::NodeInteractionType::LeftMouseRelease => {
                    ui_state.hold_entity = None;
                    ui_state.entity_to_resize = None;
                }
                crate::ui_plugin::NodeInteractionType::RightClick => {}
            }
        }
    }
}
