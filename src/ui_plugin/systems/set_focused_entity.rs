use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils::{get_timestamp, ReflectableUuid};

use std::time::Duration;

use super::{UiState, VeloNode};

pub fn set_focused_entity(
    mut interaction_query: Query<(&Interaction, &VeloNode), (Changed<Interaction>, With<VeloNode>)>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut holding_time: Local<(Duration, Option<ReflectableUuid>)>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, node) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                primary_window.cursor.icon = CursorIcon::Text;
                let now_ms = get_timestamp();
                if double_click.1 == Some(node.id)
                    && Duration::from_millis(now_ms as u64) - double_click.0
                        < Duration::from_millis(500)
                {
                    *ui_state = UiState::default();
                    ui_state.entity_to_edit = Some(node.id);
                } else {
                    *double_click = (Duration::from_millis(now_ms as u64), Some(node.id));
                }
                *holding_time = (Duration::from_millis(now_ms as u64), Some(node.id));
            }
            Interaction::Hovered => {
                if ui_state.hold_entity.is_none() && ui_state.entity_to_edit.is_none() {
                    primary_window.cursor.icon = CursorIcon::Hand;
                }
                if ui_state.entity_to_edit.is_some() {
                    primary_window.cursor.icon = CursorIcon::Text;
                }
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }

    if ui_state.hold_entity.is_some() {
        primary_window.cursor.icon = CursorIcon::Move;
    }

    let now_ms = get_timestamp();
    // 150ms delay before re-positioning the rectangle
    if ui_state.hold_entity.is_none()
        && Duration::from_millis(now_ms as u64) - holding_time.0 > Duration::from_millis(150)
        && holding_time.1.is_some()
    {
        ui_state.hold_entity = holding_time.1;
    }

    if buttons.just_released(MouseButton::Left) {
        *holding_time = (Duration::new(0, 0), None);
        ui_state.hold_entity = None;
        ui_state.entity_to_resize = None;
    }
}
