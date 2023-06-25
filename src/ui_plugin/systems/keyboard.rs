#![allow(clippy::duplicate_mod)]
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};

use bevy_cosmic_edit::{
    get_cosmic_text, get_text_spans, CosmicEdit, CosmicEditHistory, EditHistoryItem,
};
use cosmic_text::Edit;
#[cfg(not(target_arch = "wasm32"))]
use image::*;

use std::{collections::VecDeque, convert::TryInto};
use uuid::Uuid;

use crate::{
    resources::{LoadTabRequest, SaveTabRequest},
    themes::Theme,
    utils::bevy_color_to_cosmic,
    AddRect, UiState,
};

use super::ui_helpers::EditableText;
use crate::resources::{AppState, SaveDocRequest};

#[path = "../../macros.rs"]
#[macro_use]
mod macros;

pub fn keyboard_input_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut events: EventWriter<AddRect<(String, Color)>>,
    input: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut editable_text_query: Query<
        (&EditableText, &mut CosmicEdit, &mut CosmicEditHistory),
        With<EditableText>,
    >,
    theme: Res<Theme>,
) {
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor();
    let command = input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if command && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut images, &mut events, scale_factor, &theme);
    } else if command && shift && input.just_pressed(KeyCode::S) {
        commands.insert_resource(SaveDocRequest {
            doc_id: app_state.current_document.unwrap(),
            path: None,
        });
    } else if command && input.just_pressed(KeyCode::S) {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(SaveTabRequest {
                    doc_id: app_state.current_document.unwrap(),
                    tab_id: active_tab.id,
                });
            }
        }
    } else if command && input.just_pressed(KeyCode::L) {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(LoadTabRequest {
                    doc_id: app_state.current_document.unwrap(),
                    tab_id: active_tab.id,
                    drop_last_checkpoint: true,
                });
            }
        }
    } else {
        for (editable_text, mut cosmic_edit, mut cosmit_edit_history) in
            &mut editable_text_query.iter_mut()
        {
            if vec![ui_state.tab_to_edit, ui_state.doc_to_edit].contains(&Some(editable_text.id)) {
                if input.any_just_pressed([KeyCode::Escape, KeyCode::Return]) {
                    commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                    cosmic_edit.readonly = true;
                    let mut current_cursor = cosmic_edit.editor.cursor();
                    if ui_state.doc_to_edit.is_some() {
                        current_cursor.color = Some(bevy_color_to_cosmic(theme.doc_list_bg));
                    }
                    if ui_state.tab_to_edit.is_some() {
                        current_cursor.color = Some(bevy_color_to_cosmic(theme.add_tab_bg));
                    }
                    let mut edits = VecDeque::new();
                    edits.push_back(EditHistoryItem {
                        cursor: current_cursor,
                        lines: get_text_spans(
                            cosmic_edit.editor.buffer(),
                            cosmic_edit.attrs.clone(),
                        ),
                    });
                    *cosmit_edit_history = CosmicEditHistory {
                        edits,
                        current_edit: 0,
                    };
                    cosmic_edit.editor.buffer_mut().set_redraw(true);
                    *ui_state = UiState::default();
                }
                if let Some(doc_id) = ui_state.doc_to_edit {
                    let doc = app_state.docs.get_mut(&doc_id).unwrap();
                    doc.name = get_cosmic_text(cosmic_edit.editor.buffer())
                }
                if let Some(tab_id) = ui_state.tab_to_edit {
                    if let Some(doc_id) = app_state.current_document {
                        let doc = app_state.docs.get_mut(&doc_id).unwrap();
                        if let Some(tab) = doc.tabs.iter_mut().find(|x| x.id == tab_id) {
                            tab.name = get_cosmic_text(cosmic_edit.editor.buffer())
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn insert_from_clipboard(
    images: &mut ResMut<Assets<Image>>,
    events: &mut EventWriter<AddRect<(String, Color)>>,
    scale_factor: f64,
    theme: &Res<Theme>,
) {
    use crate::JsonNode;

    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(image) = clipboard.get_image() {
            let image: RgbaImage = ImageBuffer::from_raw(
                image.width.try_into().unwrap(),
                image.height.try_into().unwrap(),
                image.bytes.into_owned(),
            )
            .unwrap();
            let width = image.width();
            let height = image.height();
            let size: Extent3d = Extent3d {
                width,
                height,
                ..Default::default()
            };
            let image = Image::new(
                size,
                TextureDimension::D2,
                image.to_vec(),
                TextureFormat::Rgba8UnormSrgb,
            );
            let image = images.add(image);
            events.send(AddRect {
                node: JsonNode {
                    id: Uuid::new_v4(),
                    node_type: crate::NodeType::Rect,
                    x: 0.0,
                    y: 0.0,
                    width: width as f32 / scale_factor as f32,
                    height: height as f32 / scale_factor as f32,
                    text: crate::JsonNodeText {
                        text: "".to_string(),
                        pos: crate::TextPos::Center,
                    },
                    bg_color: pair_struct!(theme.clipboard_image_bg),
                    z: 0.,
                },
                image: Some(image),
            });
        }
    }
}
