use super::{
    ui_helpers::ResizeMarker, BevyMarkdownView, RawText, RedrawArrowEvent, VeloNode,
    VeloNodeContainer,
};
use crate::UiState;
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_markdown::BevyMarkdownNode;

pub fn resize_entity_start(
    mut interaction_query: Query<
        (&Interaction, &Parent, &ResizeMarker),
        (Changed<Interaction>, With<ResizeMarker>),
    >,
    mut button_query: Query<&VeloNode, With<VeloNode>>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, parent, resize_marker) in &mut interaction_query {
        let rectangle = button_query.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Clicked => {
                state.entity_to_resize = Some((rectangle.id, *resize_marker));
            }
            Interaction::Hovered => match *resize_marker {
                ResizeMarker::TopLeft => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
                ResizeMarker::TopRight => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomLeft => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomRight => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
            },
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn resize_entity_end(
    mut mouse_motion_events: EventReader<MouseMotion>,
    state: Res<UiState>,
    mut rectangle_query: Query<
        (&VeloNodeContainer, &mut Style),
        (
            With<VeloNodeContainer>,
            Without<RawText>,
            Without<BevyMarkdownNode>,
        ),
    >,
    mut raw_text_input_query: Query<
        (&RawText, &mut Style),
        (
            With<RawText>,
            Without<VeloNodeContainer>,
            Without<BevyMarkdownNode>,
        ),
    >,
    mut markdown_text_input_query: Query<
        (&Parent, &mut Style),
        (
            With<BevyMarkdownNode>,
            Without<VeloNodeContainer>,
            Without<RawText>,
        ),
    >,
    markdown_view_query: Query<(&BevyMarkdownView, Entity), With<BevyMarkdownView>>,
    mut events: EventWriter<RedrawArrowEvent>,
) {
    for event in mouse_motion_events.iter() {
        if let Some((id, resize_marker)) = state.entity_to_resize {
            for (rectangle, mut button_style) in &mut rectangle_query {
                if id == rectangle.id {
                    events.send(RedrawArrowEvent { id });
                    #[allow(unused)]
                    let mut delta = event.delta;
                    #[cfg(target_arch = "wasm32")]
                    {
                        // MouseMotion returns different values depending on platform
                        delta = Vec2::new(delta.x / 2., delta.y / 2.);
                    }
                    match resize_marker {
                        ResizeMarker::TopLeft => {
                            if let Val::Px(width) = button_style.width {
                                button_style.width = Val::Px(width - delta.x);
                            }

                            if let Val::Px(height) = button_style.height {
                                button_style.height = Val::Px(height - delta.y);
                            }

                            if let Val::Px(x) = button_style.left {
                                button_style.left = Val::Px(x + delta.x);
                            }
                        }
                        ResizeMarker::TopRight => {
                            if let Val::Px(width) = button_style.width {
                                button_style.width = Val::Px(width + delta.x);
                            }

                            if let Val::Px(height) = button_style.height {
                                button_style.height = Val::Px(height - delta.y);
                            }
                        }
                        ResizeMarker::BottomLeft => {
                            if let Val::Px(width) = button_style.width {
                                button_style.width = Val::Px(width - delta.x);
                            }

                            if let Val::Px(height) = button_style.height {
                                button_style.height = Val::Px(height + delta.y);
                            }

                            if let Val::Px(x) = button_style.left {
                                button_style.left = Val::Px(x + delta.x);
                            }

                            if let Val::Px(y) = button_style.bottom {
                                button_style.bottom = Val::Px(y - delta.y);
                            }
                        }
                        ResizeMarker::BottomRight => {
                            if let Val::Px(width) = button_style.width {
                                button_style.width = Val::Px(width + delta.x);
                            }

                            if let Val::Px(height) = button_style.height {
                                button_style.height = Val::Px(height + delta.y);
                            }

                            if let Val::Px(y) = button_style.bottom {
                                button_style.bottom = Val::Px(y - delta.y);
                            }
                        }
                    }
                    for (text, mut text_style) in &mut raw_text_input_query {
                        if text.id == id {
                            text_style.max_width = button_style.width;
                            text_style.max_height = button_style.height;
                        }
                    }
                    for (node, entity) in markdown_view_query.iter() {
                        if node.id == id {
                            for (parent, mut text_style) in &mut markdown_text_input_query {
                                if parent.get() == entity {
                                    text_style.max_width = button_style.width;
                                    text_style.max_height = button_style.height;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_resize_entity_end() {
    // Set up a test app with the necessary resources and entities
    let mut app = App::new();
    let entity_id = crate::utils::ReflectableUuid::generate();

    // Test all ResizeMarkers
    for &marker in &[
        ResizeMarker::TopLeft,
        ResizeMarker::TopRight,
        ResizeMarker::BottomLeft,
        ResizeMarker::BottomRight,
    ] {
        app.insert_resource(UiState {
            entity_to_resize: Some((entity_id, marker)),
            ..default()
        });

        app.add_event::<MouseMotion>();
        app.add_event::<RedrawArrowEvent>();
        app.world
            .resource_mut::<Events<MouseMotion>>()
            .send(MouseMotion {
                delta: Vec2::new(10.0, 5.0),
            });

        app.add_system(resize_entity_end);

        app.world
            .spawn(NodeBundle {
                style: Style {
                    width:Val::Px(100.0),

                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(VeloNodeContainer { id: entity_id });

        // Run the app
        app.update();

        // Check that the size and position of the rectangle have been updated correctly
        let (_velo_node_container, style) = app
            .world
            .query::<(&VeloNodeContainer, &mut Style)>()
            .iter_mut(&mut app.world)
            .last()
            .unwrap();

        match marker {
            ResizeMarker::TopLeft => {
                assert_eq!(style.width, Val::Px(90.0));
                assert_eq!(style.height, Val::Px(95.0));
                assert_eq!(style.left, Val::Px(10.0));
            }
            ResizeMarker::TopRight => {
                assert_eq!(style.width, Val::Px(120.0));
                assert_eq!(style.height, Val::Px(90.0));
            }
            ResizeMarker::BottomLeft => {
                assert_eq!(style.width, Val::Px(70.0));
                assert_eq!(style.height, Val::Px(115.0));
                assert_eq!(style.left, Val::Px(30.0));
                assert_eq!(style.bottom, Val::Px(-15.0));
            }
            ResizeMarker::BottomRight => {
                assert_eq!(style.width, Val::Px(140.0));
                assert_eq!(style.height, Val::Px(120.0));
                assert_eq!(style.bottom, Val::Px(-20.0));
            }
        }
    }
}
