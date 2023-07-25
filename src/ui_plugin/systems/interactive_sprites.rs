use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::MainCamera, utils::get_timestamp};

use std::time::Duration;

use super::{
    ui_helpers::{Drawing, InteractiveNode},
    NodeInteraction, NodeInteractionType,
};

#[derive(Default, Debug)]
pub struct HoldingState {
    duration: Duration,
    entity: Option<Entity>,
    is_holding: bool,
}

pub fn interactive_node(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    res_images: Res<Assets<Image>>,
    mut sprite_query: Query<
        (&Sprite, &Handle<Image>, &GlobalTransform, Entity),
        With<InteractiveNode>,
    >,
    drawing_query: Query<
        (&Drawing<(String, Color)>, &GlobalTransform, Entity),
        With<InteractiveNode>,
    >,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut node_interaction_events: EventWriter<NodeInteraction>,
    mut double_click: Local<(Duration, Option<Entity>)>,
    mut holding_state: Local<HoldingState>,
) {
    let (camera, camera_transform) = camera_q.single();
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor() as f32;
    let mut active_entity = None;
    for (drawing, node_transform, entity) in drawing_query.iter() {
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for point in &drawing.points {
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
        }
        x_min += node_transform.affine().translation.x;
        x_max += node_transform.affine().translation.x;
        x_min = x_min.min(x_max - 5.);
        y_min += node_transform.affine().translation.y;
        y_max += node_transform.affine().translation.y;
        y_min = y_min.min(y_max - 5.);
        let z_current = node_transform.affine().translation.z;

        if let Some(pos) = primary_window.cursor_position() {
            if let Some(pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
                    if let Some((_, z)) = active_entity {
                        if z < z_current {
                            active_entity = Some((entity, z_current));
                        }
                    } else {
                        active_entity = Some((entity, node_transform.affine().translation.z));
                    }
                }
            };
        }
    }
    for (sprite, handle, node_transform, entity) in &mut sprite_query.iter_mut() {
        let size = match sprite.custom_size {
            Some(size) => (size.x, size.y),
            None => {
                if let Some(sprite_image) = res_images.get(handle) {
                    (
                        sprite_image.size().x / scale_factor,
                        sprite_image.size().y / scale_factor,
                    )
                } else {
                    (1., 1.)
                }
            }
        };

        let x_min = node_transform.affine().translation.x - size.0 / 2.;
        let y_min = node_transform.affine().translation.y - size.1 / 2.;
        let x_max = node_transform.affine().translation.x + size.0 / 2.;
        let y_max = node_transform.affine().translation.y + size.1 / 2.;
        let z_current = node_transform.affine().translation.z;

        if let Some(pos) = primary_window.cursor_position() {
            if let Some(pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
                    if let Some((_, z)) = active_entity {
                        if z < z_current {
                            active_entity = Some((entity, z_current));
                        }
                    } else {
                        active_entity = Some((entity, node_transform.affine().translation.z));
                    }
                }
            };
        }
    }

    if let Some((active, _)) = active_entity {
        let now_ms = get_timestamp();
        let mut is_hover = true;
        if buttons.just_pressed(MouseButton::Left) {
            is_hover = false;
            if double_click.1 == Some(active)
                && Duration::from_millis(now_ms as u64) - double_click.0
                    < Duration::from_millis(500)
            {
                node_interaction_events.send(NodeInteraction {
                    entity: active,
                    node_interaction_type: NodeInteractionType::LeftDoubleClick,
                });
            } else {
                node_interaction_events.send(NodeInteraction {
                    entity: active,
                    node_interaction_type: NodeInteractionType::LeftClick,
                });
                *double_click = (Duration::from_millis(now_ms as u64), Some(active));
                *holding_state = HoldingState {
                    duration: Duration::from_millis(now_ms as u64),
                    entity: Some(active),
                    is_holding: false,
                };
            }
        }
        if buttons.just_pressed(MouseButton::Right) {
            is_hover = false;
            node_interaction_events.send(NodeInteraction {
                entity: active,
                node_interaction_type: NodeInteractionType::RightClick,
            });
        }

        if buttons.pressed(MouseButton::Left)
            && !holding_state.is_holding
            && Duration::from_millis(now_ms as u64) - holding_state.duration
                > Duration::from_millis(100)
            && holding_state.entity.is_some()
        {
            is_hover = false;
            holding_state.is_holding = true;
            node_interaction_events.send(NodeInteraction {
                entity: active,
                node_interaction_type: NodeInteractionType::LeftMouseHoldAndDrag,
            });
        }

        if buttons.just_released(MouseButton::Left) {
            *holding_state = HoldingState {
                is_holding: false,
                duration: Duration::ZERO,
                entity: None,
            };
            node_interaction_events.send(NodeInteraction {
                entity: active,
                node_interaction_type: NodeInteractionType::LeftMouseRelease,
            });
        }

        if is_hover {
            node_interaction_events.send(NodeInteraction {
                entity: active,
                node_interaction_type: NodeInteractionType::Hover,
            });
        }
    } else if buttons.just_released(MouseButton::Left) {
        *holding_state = HoldingState {
            is_holding: false,
            duration: Duration::ZERO,
            entity: None,
        };
        node_interaction_events.send(NodeInteraction {
            entity: Entity::PLACEHOLDER,
            node_interaction_type: NodeInteractionType::LeftMouseRelease,
        });
    }
}
