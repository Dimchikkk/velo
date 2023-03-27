use std::{fs::canonicalize, path::PathBuf};

use bevy::{prelude::*, window::PrimaryWindow};
pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;

use crate::{AppState, LoadRequest, SaveRequest};

use super::ui_helpers::{
    PathModalCancel, PathModalConfirm, PathModalText, PathModalTextInput, PathModalTop,
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
    for (interaction, path_modal_cancel) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (text, modal) in &mut query_path.iter_mut() {
                if Some(modal.id) == state.path_modal_id {
                    if modal.save {
                        commands.insert_resource(SaveRequest {
                            path: Some(PathBuf::from(text.sections[0].value.clone())),
                        });
                    } else if let Ok(path) =
                        canonicalize(PathBuf::from(text.sections[0].value.trim()))
                    {
                        commands.insert_resource(LoadRequest { path: Some(path) });
                    } else {
                        eprintln!("File not found: {}", text.sections[0].value);
                    }
                }
            }
            for (entity, path_modal_top) in query_top.iter() {
                if path_modal_cancel.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                    state.path_modal_id = None;
                }
            }
        }
    }
}

pub fn path_modal_keyboard_input_system(
    mut query: Query<(&mut Text, &PathModalTextInput), With<PathModalTextInput>>,
    state: Res<AppState>,
    input: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
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
        }
    }
}
