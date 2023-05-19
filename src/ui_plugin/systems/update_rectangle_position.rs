use bevy::{prelude::*, window::PrimaryWindow};

use crate::canvas::arrow::events::RedrawArrowEvent;

use super::{LeftPanel, UiState, VeloNodeContainer};

pub fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut node_position: Query<(&mut Style, &VeloNodeContainer), With<VeloNodeContainer>>,
    state: Res<UiState>,
    mut query: Query<(&Style, &LeftPanel), Without<VeloNodeContainer>>,
    mut events: EventWriter<RedrawArrowEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    for event in cursor_moved_events.iter() {
        for (mut style, top) in &mut node_position.iter_mut() {
            if Some(top.id) == state.hold_entity {
                let width = query.single_mut().0.width;
                if let (Val::Percent(x), Val::Px(element_width)) = (width, style.width) {
                    let width = (primary_window.width() * x) / 100.;
                    style.left = Val::Px(event.position.x - width - element_width / 2.);
                }
                if let Val::Px(element_height) = style.height {
                    style.bottom = Val::Px(event.position.y - element_height / 2.);
                }
                events.send(RedrawArrowEvent { id: top.id });
            }
        }
    }
}
