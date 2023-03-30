use bevy::{prelude::*, window::PrimaryWindow};

use std::collections::HashSet;

use crate::{AppState, MainCamera, RedrawArrow};

use super::ui_helpers::{create_arrow, ArrowConnect, ArrowMeta, CreateArrow};

pub fn create_arrow_start(
    mut interaction_query: Query<
        (&Interaction, &ArrowConnect),
        (Changed<Interaction>, With<ArrowConnect>),
    >,
    mut state: ResMut<AppState>,
    mut create_arrow: EventWriter<CreateArrow>,
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
                    create_arrow.send(CreateArrow {
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
    mut commands: Commands,
    mut events: EventReader<CreateArrow>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();
    for event in events.iter() {
        let mut start = None;
        let mut end = None;
        for (arrow_connect, global_transform) in &mut arrow_markers.iter_mut() {
            if *arrow_connect == event.start {
                let world_position = global_transform.affine().translation;
                start = Some(Vec2::new(
                    world_position.x,
                    primary_window.height() - world_position.y,
                ));
            }
            if *arrow_connect == event.end {
                let world_position = global_transform.affine().translation;
                end = Some(Vec2::new(
                    world_position.x,
                    primary_window.height() - world_position.y,
                ));
            }
        }

        if let (Some(start), Some(end)) = (start, end) {
            let start = camera.viewport_to_world_2d(camera_transform, start);
            let end = camera.viewport_to_world_2d(camera_transform, end);
            if let (Some(start), Some(end)) = (start, end) {
                create_arrow(
                    &mut commands,
                    start,
                    end,
                    ArrowMeta {
                        start: event.start,
                        end: event.end,
                        arrow_type: event.arrow_type,
                    },
                );
            }
        }
    }
}

pub fn redraw_arrows(
    mut redraw_arrow: EventReader<RedrawArrow>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut arrow_query: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut commands: Commands,
) {
    let mut despawned: HashSet<ArrowMeta> = HashSet::new();

    for event in redraw_arrow.iter() {
        for (entity, arrow) in &mut arrow_query.iter_mut() {
            if despawned.contains(arrow) {
                continue;
            }
            if arrow.start.id == event.id || arrow.end.id == event.id {
                if let Some(entity) = commands.get_entity(entity) {
                    despawned.insert(*arrow);
                    entity.despawn_recursive();
                }
            }
        }
    }

    for arrow_meta in despawned {
        create_arrow.send(CreateArrow {
            start: arrow_meta.start,
            end: arrow_meta.end,
            arrow_type: arrow_meta.arrow_type,
        });
    }
}