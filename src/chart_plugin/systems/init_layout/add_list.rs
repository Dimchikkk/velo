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
    chart_plugin::ui_helpers::{DocListItem, ReflectableUuid},
    AppState, Doc, SaveRequest, Tab,
};

use super::ui_helpers::ScrollingList;

pub fn add_list(
    commands: &mut Commands,
    state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
    font: Handle<Font>,
) -> Entity {
    let tab_id = ReflectableUuid(Uuid::new_v4());
    let tabs = vec![Tab {
        id: tab_id,
        name: "Tab 1".to_string(),
        checkpoints: VecDeque::new(),
        is_active: true,
    }];
    let doc_id = ReflectableUuid(Uuid::new_v4());
    let mut docs = HashMap::new();
    docs.insert(
        doc_id,
        Doc {
            id: doc_id,
            name: "Untitled".to_string(),
            tabs,
            tags: vec![],
        },
    );
    state.docs = docs;
    state.current_document = Some(doc_id);
    commands.insert_resource(SaveRequest {
        path: None,
        tab_id: Some(tab_id),
    });

    let names = pkv
        .get::<HashMap<ReflectableUuid, String>>("names")
        .unwrap();
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
            ScrollingList::default(),
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .id();
    commands.entity(top).add_child(node);
    names.into_iter().for_each(|(id, name)| {
        let button = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        justify_content: JustifyContent::Center,
                        padding: UiRect {
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                            top: Val::Px(5.),
                            bottom: Val::Px(5.),
                        },
                        ..default()
                    },
                    ..default()
                },
                DocListItem { id },
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ))
            .id();
        let text_bundle = commands
            .spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: name,
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 18.,
                                color: Color::BLACK,
                            },
                        }],
                        ..default()
                    },
                    style: Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    ..default()
                },
                Label,
            ))
            .id();
        commands.entity(button).add_child(text_bundle);
        commands.entity(node).add_child(button);
    });
    top
}

pub fn list_item_click(
    mut interaction_query: Query<
        (&Interaction, &DocListItem, &mut BackgroundColor),
        (Changed<Interaction>, With<DocListItem>),
    >,
) {
    for (interaction, doc_list_item, mut bg_color) in &mut interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {}
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(1.0, 1.0, 1.0, 0.8).into();
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(1.0, 1.0, 1.0, 0.5).into();
            }
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
