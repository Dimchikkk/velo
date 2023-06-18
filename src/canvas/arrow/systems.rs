use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{ArrowConnect, ArrowMeta};
use super::events::{CreateArrow, RedrawArrow};
use super::utils::{build_arrow, create_arrow};
use crate::themes::Theme;
use crate::ui_plugin::{NodeInteraction, UiState};
use bevy_prototype_lyon::prelude::Path;

pub fn create_arrow_start(
    mut node_interaction_events: EventReader<NodeInteraction>,
    arrow_connect_query: Query<&ArrowConnect, With<ArrowConnect>>,
    mut state: ResMut<UiState>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for event in node_interaction_events.iter() {
        if let Ok(arrow_connect) = arrow_connect_query.get(event.entity) {
            match event.node_interaction_type {
                crate::ui_plugin::NodeInteractionType::Hover => {
                    primary_window.cursor.icon = CursorIcon::Crosshair;
                }
                crate::ui_plugin::NodeInteractionType::LeftClick => {
                    match state.arrow_to_draw_start {
                        Some(start_arrow) => {
                            if start_arrow.id == arrow_connect.id {
                                continue;
                            }
                            state.arrow_to_draw_start = None;
                            create_arrow.send(CreateArrow {
                                start: start_arrow,
                                end: *arrow_connect,
                                arrow_type: state.arrow_type,
                            });
                        }
                        None => {
                            state.arrow_to_draw_start = Some(*arrow_connect);
                        }
                    }
                }
                crate::ui_plugin::NodeInteractionType::LeftDoubleClick => {}
                crate::ui_plugin::NodeInteractionType::LeftMouseHoldAndDrag => {}
                crate::ui_plugin::NodeInteractionType::RightClick => {}
                crate::ui_plugin::NodeInteractionType::LeftMouseRelease => {}
            }
        }
    }
}

pub fn create_arrow_end(
    mut commands: Commands,
    mut events: EventReader<CreateArrow>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    theme: Res<Theme>,
) {
    for event in events.iter() {
        let mut start = None;
        let mut end = None;
        for (arrow_connect, global_transform) in &mut arrow_markers.iter() {
            if *arrow_connect == event.start {
                start = Some(global_transform.affine().translation.truncate());
            }
            if *arrow_connect == event.end {
                end = Some(global_transform.affine().translation.truncate());
            }
            if let (Some(start), Some(end)) = (start, end) {
                create_arrow(
                    &mut commands,
                    &theme,
                    start,
                    end,
                    ArrowMeta {
                        start: event.start,
                        end: event.end,
                        arrow_type: event.arrow_type,
                    },
                );
                break;
            }
        }
    }
}
pub fn redraw_arrows(
    mut redraw_arrow: EventReader<RedrawArrow>,
    mut arrow_query: Query<(&mut Path, &mut ArrowMeta), With<ArrowMeta>>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
) {
    for event in redraw_arrow.iter() {
        for (mut path, mut arrow) in arrow_query.iter_mut() {
            if arrow.start.id == event.id || arrow.end.id == event.id {
                let (arrow_hold_vec, arrow_move_vec): (Vec<_>, Vec<_>) = arrow_markers
                    .iter()
                    .filter(|(x, _)| x.id == arrow.end.id || x.id == arrow.start.id)
                    .map(|(ac, gt)| (ac, gt.affine().translation.truncate()))
                    .partition(|(x, _)| x.id == arrow.end.id);
                let arrow_pos = arrow_hold_vec
                    .iter()
                    .flat_map(move |x| std::iter::repeat(*x).zip(arrow_move_vec.clone()))
                    .min_by_key(|(arrow_hold, arrow_move)| {
                        arrow_hold.1.distance(arrow_move.1) as u32
                    });
                if let Some((start_pos, end_pos)) = arrow_pos {
                    let ((start_pos, start), (end_pos, end)) = if start_pos.0.id == arrow.start.id {
                        (start_pos, end_pos)
                    } else {
                        (end_pos, start_pos)
                    };
                    arrow.start = *start_pos;
                    arrow.end = *end_pos;
                    *path = build_arrow(start, end, *arrow);
                }
            }
        }
    }
}
