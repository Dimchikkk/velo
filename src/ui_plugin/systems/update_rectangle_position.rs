use bevy::prelude::*;

use crate::{canvas::arrow::events::RedrawArrow, components::MainCamera};

use super::{
    ui_helpers::{RawText, VeloBorder, VeloNode},
    UiState,
};

pub fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    raw_text_query: Query<(&RawText, &Parent), With<RawText>>,
    border_query: Query<&Parent, With<VeloBorder>>,
    mut velo_node_query: Query<&mut Transform, With<VeloNode>>,
    mut events: EventWriter<RedrawArrow>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    state: Res<UiState>,
) {
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        for (raw_text, parent) in &mut raw_text_query.iter() {
            if Some(raw_text.id) == state.hold_entity && state.entity_to_edit.is_none() {
                if let Some(pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
                    let border = border_query.get(parent.get()).unwrap();
                    let mut top = velo_node_query.get_mut(border.get()).unwrap();
                    top.translation.x = pos.x;
                    top.translation.y = pos.y;
                    events.send(RedrawArrow { id: raw_text.id });
                }
            }
        }
    }
}
