use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::ui_helpers::ScrollingList;
use crate::chart_plugin::ui_helpers::DocListItemButton;

use crate::resources::{AppState, LoadRequest, SaveRequest};

pub fn list_item_click(
    mut interaction_query: Query<
        (&Interaction, &DocListItemButton),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut state: ResMut<AppState>,
    mut commands: Commands,
) {
    for (interaction, doc_list_item) in &mut interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if Some(doc_list_item.id) != state.current_document {
                    commands.insert_resource(SaveRequest {
                        doc_id: Some(state.current_document.unwrap()),
                        tab_id: None,
                    });
                    state.current_document = Some(doc_list_item.id);
                    commands.insert_resource(LoadRequest {
                        doc_id: Some(doc_list_item.id),
                        drop_last_checkpoint: false,
                    });
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn mouse_scroll_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
