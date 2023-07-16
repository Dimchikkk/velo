use super::{
    ui_helpers::{ResizeMarker, VeloShape},
    NodeInteraction, NodeType, RawText, RedrawArrow, VeloNode,
};
use crate::{canvas::arrow::components::ArrowConnect, components::MainCamera, UiState};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicEdit;
use bevy_prototype_lyon::prelude::Path;
use cosmic_text::Edit;

pub fn resize_entity_start(
    mut ui_state: ResMut<UiState>,
    mut node_interaction_events: EventReader<NodeInteraction>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    resize_marker_query: Query<(&ResizeMarker, &Parent, &mut Transform), With<ResizeMarker>>,
    velo_node_query: Query<&VeloNode, With<VeloNode>>,
) {
    let mut primary_window = windows.single_mut();

    for event in node_interaction_events.iter() {
        if let Ok((resize_marker, parent, _)) = resize_marker_query.get(event.entity) {
            match event.node_interaction_type {
                super::NodeInteractionType::Hover => match *resize_marker {
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
                super::NodeInteractionType::LeftClick => {}
                super::NodeInteractionType::LeftDoubleClick => {}
                super::NodeInteractionType::LeftMouseHoldAndDrag => {
                    let velo_node = velo_node_query.get(parent.get()).unwrap();
                    ui_state.entity_to_resize = Some(velo_node.id);
                }
                super::NodeInteractionType::RightClick => {}
                super::NodeInteractionType::LeftMouseRelease => {}
            }
        }
    }
}

pub fn resize_entity_end(
    mut ui_state: ResMut<UiState>,
    mut node_interaction_events: EventReader<NodeInteraction>,
) {
    for event in node_interaction_events.iter() {
        if event.node_interaction_type == super::NodeInteractionType::LeftMouseRelease
            && ui_state.entity_to_resize.is_some()
        {
            ui_state.entity_to_resize = None;
        }
    }
}

pub fn resize_entity_run(
    ui_state: ResMut<UiState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut events: EventWriter<RedrawArrow>,
    mut resize_marker_query: Query<
        (&ResizeMarker, &Parent, &mut Transform),
        (With<ResizeMarker>, Without<VeloNode>, Without<ArrowConnect>),
    >,
    mut arrow_connector_query: Query<
        (&ArrowConnect, &mut Transform),
        (With<ArrowConnect>, Without<VeloNode>, Without<ResizeMarker>),
    >,
    mut raw_text_query: Query<(&Parent, &RawText, &mut CosmicEdit, &mut Sprite), With<RawText>>,
    mut border_query: Query<(&Parent, &VeloShape, &mut Path), With<VeloShape>>,
    mut velo_node_query: Query<
        (&mut Transform, &Children),
        (With<VeloNode>, Without<ResizeMarker>, Without<ArrowConnect>),
    >,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();

    if let Some(id) = ui_state.entity_to_resize {
        for (raw_text_parent, raw_text, mut cosmic_edit, mut sprite) in
            &mut raw_text_query.iter_mut()
        {
            if id != raw_text.id {
                continue;
            }
            let event = cursor_moved_events.iter().last();
            if let Some(cursor_pos) = event
                .and_then(|event| camera.viewport_to_world_2d(camera_transform, event.position))
            {
                let (border_parent, velo_border, mut path) =
                    border_query.get_mut(raw_text_parent.get()).unwrap();
                let (velo_transform, children) =
                    velo_node_query.get_mut(border_parent.get()).unwrap();
                let pos = velo_transform.translation.truncate();
                let mut width = f32::max(((cursor_pos.x - pos.x).abs() * 2.).round(), 1.);
                let mut height = f32::max(((cursor_pos.y - pos.y).abs() * 2.).round(), 1.);
                if velo_border.node_type == NodeType::Circle {
                    width = f32::max(width, height);
                    height = f32::max(width, height);
                }
                if width % 2.0 != 0.0 {
                    width += 1.0;
                }
                if height % 2.0 != 0.0 {
                    height += 1.0;
                }

                cosmic_edit.width = width;
                cosmic_edit.height = height;
                sprite.custom_size = Some(Vec2::new(width, height));
                cosmic_edit.editor.buffer_mut().set_redraw(true);

                for child in children.iter() {
                    // update resize markers positions
                    if let Ok(resize) = resize_marker_query.get_mut(*child) {
                        let mut resize_transform = resize.2;
                        match resize.0 {
                            ResizeMarker::TopLeft => {
                                resize_transform.translation.x = -width / 2.;
                                resize_transform.translation.y = height / 2.;
                            }
                            ResizeMarker::TopRight => {
                                resize_transform.translation.x = width / 2.;
                                resize_transform.translation.y = height / 2.;
                            }
                            ResizeMarker::BottomLeft => {
                                resize_transform.translation.x = -width / 2.;
                                resize_transform.translation.y = -height / 2.;
                            }
                            ResizeMarker::BottomRight => {
                                resize_transform.translation.x = width / 2.;
                                resize_transform.translation.y = -height / 2.;
                            }
                        }
                    }
                    // update arrow connectors positions
                    if let Ok(arrow_connect) = arrow_connector_query.get_mut(*child) {
                        let mut arrow_transform = arrow_connect.1;
                        match arrow_connect.0.pos {
                            crate::canvas::arrow::components::ArrowConnectPos::Top => {
                                arrow_transform.translation.x = 0.;
                                arrow_transform.translation.y = height / 2.;
                            }
                            crate::canvas::arrow::components::ArrowConnectPos::Bottom => {
                                arrow_transform.translation.x = 0.;
                                arrow_transform.translation.y = -height / 2.;
                            }
                            crate::canvas::arrow::components::ArrowConnectPos::Left => {
                                arrow_transform.translation.x = -width / 2.;
                                arrow_transform.translation.y = 0.;
                            }
                            crate::canvas::arrow::components::ArrowConnectPos::Right => {
                                arrow_transform.translation.x = width / 2.;
                                arrow_transform.translation.y = 0.;
                            }
                        }
                    }
                }

                // update size of bevy_lyon node
                let points = [
                    Vec2::new(-width / 2., -height / 2.),
                    Vec2::new(-width / 2., height / 2.),
                    Vec2::new(width / 2., height / 2.),
                    Vec2::new(width / 2., -height / 2.),
                ];

                let new_path = match velo_border.node_type {
                    NodeType::Rect => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
                        &bevy_prototype_lyon::shapes::RoundedPolygon {
                            points: points.into_iter().collect(),
                            closed: true,
                            radius: 10.,
                        },
                    ),
                    NodeType::Paper => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
                        &bevy_prototype_lyon::shapes::Polygon {
                            points: points.into_iter().collect(),
                            closed: true,
                        },
                    ),
                    NodeType::Circle => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
                        &bevy_prototype_lyon::shapes::Circle {
                            radius: width / 2.,
                            center: Vec2::new(0., 0.),
                        },
                    ),
                };
                *path = new_path;
                events.send(RedrawArrow { id: raw_text.id });
            }
        }
    }
}
