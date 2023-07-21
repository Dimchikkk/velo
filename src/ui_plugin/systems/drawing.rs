use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::{Path, PathBuilder, ShapeBundle, Stroke};

use crate::{
    components::MainCamera,
    resources::AppState,
    themes::Theme,
    utils::{get_timestamp, ReflectableUuid},
};

use super::{
    ui_helpers::{Drawing, MainPanel},
    UiState,
};

#[path = "../../macros.rs"]
#[macro_use]
mod macros;

pub fn drawing(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainPanel>)>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut holding_state: Local<Option<Duration>>,
    buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    theme: Res<Theme>,
    mut drawing_line_q: Query<
        (&mut Path, &mut Drawing<(String, Color)>),
        With<Drawing<(String, Color)>>,
    >,
    app_state: Res<AppState>,
    mut z_index_local: Local<f32>,
) {
    let (camera, camera_transform) = camera_q.single();
    let mut primary_window = windows.single_mut();
    let now_ms = get_timestamp();
    let now = Duration::from_millis(now_ms as u64);

    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            *holding_state = Some(now);
        }
    }

    if ui_state.drawing_mode {
        let current_document_id = app_state.current_document.unwrap();
        let current_document = app_state.docs.get(&current_document_id);
        if current_document.is_none() {
            return;
        }
        let tab = app_state
            .docs
            .get(&current_document_id)
            .unwrap()
            .tabs
            .iter()
            .find(|x| x.is_active)
            .unwrap();

        if buttons.just_released(MouseButton::Left) {
            *holding_state = None;
            primary_window.cursor.icon = CursorIcon::Default;
            ui_state.entity_to_draw = None;
        }

        if let Some(holding_time) = *holding_state {
            primary_window.cursor.icon = CursorIcon::Crosshair;
            if now - holding_time > Duration::from_millis(60) {
                if let Some(pos) = primary_window.cursor_position() {
                    if let Some(pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                        if let Some(entity_to_draw) = ui_state.entity_to_draw {
                            for (mut path, mut drawing_line) in &mut drawing_line_q.iter_mut() {
                                if entity_to_draw == drawing_line.id {
                                    if drawing_line.points.last() == Some(&pos) {
                                        continue;
                                    }
                                    drawing_line.points.push(pos);
                                    let mut path_builder = PathBuilder::new();
                                    let mut points_iter = drawing_line.points.iter();
                                    let start = points_iter.next().unwrap();
                                    path_builder.move_to(*start);
                                    path_builder.line_to(*start);
                                    for point in points_iter {
                                        path_builder.line_to(*point);
                                    }
                                    *path = path_builder.build();
                                }
                            }
                        } else {
                            let id = ReflectableUuid::generate();
                            let pair_color = ui_state
                                .draw_color_pair
                                .clone()
                                .unwrap_or(pair_struct!(theme.drawing_pencil_btn));
                            *z_index_local += 0.01 % f32::MAX;
                            commands.spawn((
                                ShapeBundle {
                                    path: PathBuilder::new().build(),
                                    transform: Transform::from_xyz(
                                        0.,
                                        0.,
                                        tab.z_index + *z_index_local,
                                    ),
                                    ..Default::default()
                                },
                                Stroke::new(pair_color.1, 2.),
                                Drawing {
                                    points: vec![pos],
                                    drawing_color: pair_color,
                                    id,
                                },
                            ));
                            ui_state.entity_to_draw = Some(id);
                        }
                    }
                }
            }
        }
    }
}
