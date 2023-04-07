use std::collections::{HashMap, VecDeque};

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_pkv::PkvStore;
use uuid::Uuid;

use crate::{
    chart_plugin::ui_helpers::{
        add_list_item, add_tab, DocList, DocListItemButton, ReflectableUuid,
    },
    AppState, Doc, LoadRequest, Tab, UpdateListHighlight,
};

use super::ui_helpers::ScrollingList;

pub fn add_list(
    bottom_panel: Entity,
    commands: &mut Commands,
    state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
    font: Handle<Font>,
) -> Entity {
    if let Ok(last_saved) = pkv.get::<ReflectableUuid>("last_saved") {
        state.current_document = Some(last_saved);
        commands.insert_resource(LoadRequest {
            doc_id: Some(last_saved),
            drop_last_checkpoint: false,
        });
    }

    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                size: Size::new(Val::Percent(80.), Val::Percent(80.)),
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.5).into(),
            ..default()
        })
        .id();
    let node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    max_size: Size::UNDEFINED,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            DocList,
            ScrollingList::default(),
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .id();

    if let Ok(names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
        names.into_iter().for_each(|(id, name)| {
            let button = add_list_item(commands, font.clone(), id, name);
            commands.entity(node).add_child(button);
        });
    } else {
        let tab_id = ReflectableUuid(Uuid::new_v4());
        let tab_name: String = "Tab 1".to_string();
        let tabs = vec![Tab {
            id: tab_id,
            name: tab_name.clone(),
            checkpoints: VecDeque::new(),
            is_active: true,
        }];
        let doc_id = ReflectableUuid(Uuid::new_v4());
        let mut docs = HashMap::new();
        let name = "Untitled".to_string();
        docs.insert(
            doc_id,
            Doc {
                id: doc_id,
                name: name.clone(),
                tabs,
                tags: vec![],
            },
        );
        let button = add_list_item(commands, font.clone(), doc_id, name);
        state.docs = docs;
        state.current_document = Some(doc_id);
        let tab_view = add_tab(commands, font, tab_name, tab_id);

        commands.entity(bottom_panel).add_child(tab_view);
        commands.entity(node).add_child(button);
    }
    commands.entity(top).add_child(node);
    top
}

pub fn list_selected_highlight(
    mut query: Query<(&DocListItemButton, &mut BackgroundColor), With<DocListItemButton>>,
    state: Res<AppState>,
    mut events: EventReader<UpdateListHighlight>,
) {
    for _ in events.iter() {
        for (doc_list_item, mut bg_color) in &mut query.iter_mut() {
            if doc_list_item.id == state.current_document.unwrap() {
                bg_color.0 = Color::ALICE_BLUE;
            } else {
                bg_color.0 = Color::rgba(1.0, 1.0, 1.0, 1.0);
            }
        }
    }
}

pub fn list_item_click(
    mut interaction_query: Query<
        (&Interaction, &DocListItemButton, &mut BackgroundColor),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    mut events: EventWriter<UpdateListHighlight>,
) {
    for (interaction, doc_list_item, mut bg_color) in &mut interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.current_document = Some(doc_list_item.id);
                commands.insert_resource(LoadRequest {
                    doc_id: Some(doc_list_item.id),
                    drop_last_checkpoint: false,
                });
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(1.0, 1.0, 1.0, 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(1.0, 1.0, 1.0, 0.5);
            }
        }
        events.send(UpdateListHighlight);
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
