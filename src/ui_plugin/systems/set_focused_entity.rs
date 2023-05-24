use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils::{get_timestamp, ReflectableUuid};

use std::time::Duration;

use super::{UiState, VeloNode};

pub fn set_focused_entity(
    mut interaction_query: Query<(&Interaction, &VeloNode), (Changed<Interaction>, With<VeloNode>)>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut holding_time: Local<(Duration, Option<ReflectableUuid>)>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, node) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                primary_window.cursor.icon = CursorIcon::Text;
                *state = UiState::default();
                state.entity_to_edit = Some(node.id);
                let now_ms = get_timestamp();
                *holding_time = (Duration::from_millis(now_ms as u64), Some(node.id));
            }
            Interaction::Hovered => {
                if state.hold_entity.is_none() && state.entity_to_edit.is_none() {
                    primary_window.cursor.icon = CursorIcon::Hand;
                }
                if state.entity_to_edit.is_some() {
                    primary_window.cursor.icon = CursorIcon::Text;
                }
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }

    if state.hold_entity.is_some() {
        primary_window.cursor.icon = CursorIcon::Move;
    }

    let now_ms = get_timestamp();
    // 250ms delay before re-positioning the rectangle
    if state.hold_entity.is_none()
        && Duration::from_millis(now_ms as u64) - holding_time.0 > Duration::from_millis(250)
        && holding_time.1.is_some()
    {
        state.hold_entity = holding_time.1;
    }

    if buttons.just_released(MouseButton::Left) {
        *holding_time = (Duration::new(0, 0), None);
        state.hold_entity = None;
        state.entity_to_resize = None;
    }
}
