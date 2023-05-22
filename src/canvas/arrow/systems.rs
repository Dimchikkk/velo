use bevy::{prelude::*, window::PrimaryWindow};

// use super::utils::{build_arrow, create_arrow};
use super::components::{ArrowConnect, ArrowMeta};
// use crate::states::{AppState, MainCamera, RedrawArrow};
use super::events::{CreateArrowEvent, RedrawArrowEvent};
use super::utils::{build_arrow, create_arrow, get_pos};
use crate::components::MainCamera;
use crate::ui_plugin::UiState;

pub fn create_arrow_start(
    mut interaction_query: Query<
        (&Interaction, &ArrowConnect),
        (Changed<Interaction>, With<ArrowConnect>),
    >,
    mut state: ResMut<UiState>,
    mut create_arrow: EventWriter<CreateArrowEvent>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, arrow_connect) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => match state.arrow_to_draw_start {
                Some(start_arrow) => {
                    if start_arrow.id == arrow_connect.id {
                        continue;
                    }
                    state.arrow_to_draw_start = None;
                    create_arrow.send(CreateArrowEvent {
                        start: start_arrow,
                        end: *arrow_connect,
                        arrow_type: state.arrow_type,
                    });
                }
                None => {
                    state.arrow_to_draw_start = Some(*arrow_connect);
                }
            },
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Crosshair;
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn create_arrow_end(
    mut gizmos: Gizmos,
    _commands: Commands,
    mut events: EventReader<CreateArrowEvent>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    for event in events.iter() {
        let mut start = None;
        let mut end = None;
        for (arrow_connect, global_transform) in &mut arrow_markers.iter() {
            if *arrow_connect == event.start {
                start = get_pos(global_transform, primary_window, camera, camera_transform);
            }
            if *arrow_connect == event.end {
                end = get_pos(global_transform, primary_window, camera, camera_transform);
            }
            if let (Some(start), Some(end)) = (start, end) {
                create_arrow(
                    &mut gizmos,
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
    mut gizmos: Gizmos,
    mut redraw_arrow: EventReader<RedrawArrowEvent>,
    mut arrow_query: Query<&mut ArrowMeta, With<ArrowMeta>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    for event in redraw_arrow.iter() {
        for mut arrow in arrow_query.iter_mut() {
            if arrow.start.id == event.id || arrow.end.id == event.id {
                let (arrow_hold_vec, arrow_move_vec): (Vec<_>, Vec<_>) = arrow_markers
                    .iter()
                    .filter(|(x, _)| x.id == arrow.end.id || x.id == arrow.start.id)
                    .filter_map(|(ac, gt)| {
                        Some((ac, get_pos(gt, primary_window, camera, camera_transform)?))
                    })
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
                    build_arrow(&mut gizmos, start, end, *arrow);
                }
            }
        }
    }
}
