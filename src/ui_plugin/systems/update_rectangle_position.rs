use std::os::macos::raw;

use bevy::prelude::*;
use bevy_cosmic_edit::CosmicEdit;
use cosmic_text::Edit;

use crate::{canvas::arrow::events::RedrawArrowEvent, components::MainCamera};

use super::{
    ui_helpers::{RawText, VeloNode},
    UiState,
};

pub fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut raw_text_query: Query<(&mut CosmicEdit, &RawText), With<RawText>>,
    mut velo_node_query: Query<(&mut Transform, &VeloNode), With<VeloNode>>,
    mut events: EventWriter<RedrawArrowEvent>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    state: Res<UiState>,
) {
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        for (mut cosmic_edit, raw_text) in &mut raw_text_query.iter_mut() {
            if Some(raw_text.id) == state.hold_entity && state.entity_to_edit.is_none() {
                if let Some(pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
                    for (mut transform, top) in velo_node_query.iter_mut() {
                        if top.id == raw_text.id {
                            transform.translation.x = pos.x;
                            transform.translation.y = pos.y;
                            break;
                        }
                    }
                    events.send(RedrawArrowEvent { id: raw_text.id });
                    cosmic_edit.editor.buffer_mut().set_redraw(true);
                }
            }
        }
    }
}
