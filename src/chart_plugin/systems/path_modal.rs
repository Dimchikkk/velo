use std::{fs::canonicalize, path::PathBuf};

use bevy::{prelude::*, window::PrimaryWindow};
use uuid::Uuid;

use crate::{AppState, LoadRequest, SaveRequest};

use super::ui_helpers::{
    spawn_path_modal, LoadState, PathModalCancel, PathModalConfirm, PathModalText,
    PathModalTextInput, PathModalTop, ReflectableUuid, SaveState,
};

pub fn cancel_path_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &PathModalCancel),
        (Changed<Interaction>, With<PathModalCancel>),
    >,
    mut state: ResMut<AppState>,
    query: Query<(Entity, &PathModalTop), With<PathModalTop>>,
) {
    for (interaction, path_modal_cancel) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query.iter() {
                if path_modal_cancel.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                    state.path_modal_id = None;
                }
            }
        }
    }
}

pub fn confirm_path_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &PathModalConfirm),
        (Changed<Interaction>, With<PathModalConfirm>),
    >,
    mut state: ResMut<AppState>,
    mut query_path: Query<(&Text, &PathModalTextInput), With<PathModalTextInput>>,
    query_top: Query<(Entity, &PathModalTop), With<PathModalTop>>,
) {
    for (interaction, path_modal_confirm) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (text, modal) in &mut query_path.iter_mut() {
                if Some(modal.id) == state.path_modal_id {
                    if modal.save {
                        commands.insert_resource(SaveRequest {
                            path: Some(PathBuf::from(text.sections[0].value.trim())),
                            tab_id: None,
                        });
                    } else if let Ok(path) =
                        canonicalize(PathBuf::from(text.sections[0].value.trim()))
                    {
                        commands.insert_resource(LoadRequest {
                            path: Some(path),
                            drop_last_checkpoint: false,
                        });
                    } else {
                        eprintln!("File not found: {}", text.sections[0].value);
                    }
                }
            }
            for (entity, path_modal_top) in query_top.iter() {
                if path_modal_confirm.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                    state.path_modal_id = None;
                }
            }
        }
    }
}

pub fn path_modal_keyboard_input_system(
    mut query: Query<(&mut Text, &PathModalTextInput), With<PathModalTextInput>>,
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    query_top: Query<(Entity, &PathModalTop), With<PathModalTop>>,
    mut commands: Commands,
) {
    for (mut text, modal) in &mut query.iter_mut() {
        if Some(modal.id) == state.path_modal_id {
            if input.just_pressed(KeyCode::Back) {
                let mut str = text.sections[0].value.clone();
                str.pop();
                text.sections[0].value = str;
            } else {
                for ev in char_evr.iter() {
                    text.sections[0].value = format!("{}{}", text.sections[0].value, ev.char);
                }
            }
        }
    }
    if input.just_pressed(KeyCode::Return) {
        for (text, modal) in &mut query.iter_mut() {
            if Some(modal.id) == state.path_modal_id {
                if modal.save {
                    commands.insert_resource(SaveRequest {
                        path: Some(PathBuf::from(text.sections[0].value.trim())),
                        tab_id: None,
                    });
                } else if let Ok(path) = canonicalize(PathBuf::from(text.sections[0].value.trim()))
                {
                    commands.insert_resource(LoadRequest {
                        path: Some(path),
                        drop_last_checkpoint: false,
                    });
                } else {
                    eprintln!("File not found: {}", text.sections[0].value);
                }
            }
        }
        for (entity, path_modal_top) in query_top.iter() {
            if Some(path_modal_top.id) == state.path_modal_id {
                commands.entity(entity).despawn_recursive();
                state.path_modal_id = None;
            }
        }
    }
}

pub fn set_focused_modal(
    mut interaction_query: Query<
        (&Interaction, &PathModalText),
        (Changed<Interaction>, With<PathModalText>),
    >,
    mut state: ResMut<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    for (interaction, modal) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            window.cursor.icon = CursorIcon::Text;
            state.path_modal_id = Some(modal.id);
            state.entity_to_edit = None;
        }
    }
}

pub fn open_path_modal(
    mut save_query: Query<&Interaction, (Changed<Interaction>, With<SaveState>)>,
    mut load_query: Query<&Interaction, (Changed<Interaction>, With<LoadState>)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut state: ResMut<AppState>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    for interaction in &mut save_query {
        if *interaction == Interaction::Clicked {
            let id = ReflectableUuid(Uuid::new_v4());
            state.path_modal_id = Some(id);
            state.entity_to_edit = None;
            let entity = spawn_path_modal(&mut commands, font.clone(), id, true);
            commands.entity(state.main_panel.unwrap()).add_child(entity);
        }
    }
    for interaction in &mut load_query {
        if *interaction == Interaction::Clicked {
            let id = ReflectableUuid(Uuid::new_v4());
            state.path_modal_id = Some(id);
            state.entity_to_edit = None;
            let entity = spawn_path_modal(&mut commands, font.clone(), id, false);
            commands.entity(state.main_panel.unwrap()).add_child(entity);
        }
    }
}
