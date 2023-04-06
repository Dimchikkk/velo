use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::ui_helpers::ScrollingList;

pub fn add_list(commands: &mut Commands, font: Handle<Font>) -> Entity {
    // List with hidden overflow
    commands
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
        .with_children(|parent| {
            // Moving panel
            parent
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
                .with_children(|parent| {
                    // List items
                    for i in 0..30 {
                        parent.spawn((
                            TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: format!("Item {i}"),
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
                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        ));
                    }
                });
        })
        .id()
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
