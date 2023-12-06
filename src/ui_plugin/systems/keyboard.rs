#![allow(clippy::duplicate_mod)]
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};

use bevy_cosmic_edit::{
    get_cosmic_text, get_text_spans, ActiveEditor, CosmicEdit, CosmicEditHistory, EditHistoryItem,
};
use bevy_prototype_lyon::prelude::{PathBuilder, ShapeBundle, Stroke};
use cosmic_text::Edit;
#[cfg(not(target_arch = "wasm32"))]
use image::*;

use std::{collections::VecDeque, convert::TryInto};
use uuid::Uuid;

use crate::{
    components::MainCamera,
    resources::{LoadTabRequest, SaveTabRequest},
    themes::Theme,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
    AddRect, JsonNode, JsonNodeText, NodeType, UiState,
};

use super::ui_helpers::{Drawing, EditableText, InteractiveNode, VeloNode};
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
    mut input: ResMut<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut editable_text_query: Query<
        (&EditableText, &mut CosmicEdit, &mut CosmicEditHistory),
        With<EditableText>,
    >,
    mut camera_proj_query: Query<&Transform, With<MainCamera>>,
    theme: Res<Theme>,
    mut copied_drawing: Local<Option<(Drawing<(String, Color)>, f32)>>,
    mut drawing_q: Query<
        (&Drawing<(String, Color)>, &GlobalTransform),
        With<Drawing<(String, Color)>>,
    >,
    velo_node_query: Query<(Entity, &VeloNode)>,
) {
    let camera_transform = camera_proj_query.single_mut();
    let x = camera_transform.translation.x;
    let y = camera_transform.translation.y;
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor();
    #[cfg(target_os = "macos")]
    let command = input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);
    #[cfg(not(target_os = "macos"))]
    let command = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if command && input.just_pressed(KeyCode::C) {
        if ui_state.entity_to_draw_selected.is_some() {
            for (drawing, gt) in &mut drawing_q.iter_mut() {
                if drawing.id == ui_state.entity_to_draw_selected.unwrap() {
                    *copied_drawing = Some((drawing.clone(), gt.affine().translation.z));
                }
            }
        } else {
            *copied_drawing = None;
        }
    } else if command && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut images, &mut events, x, y, scale_factor, &theme);

        if let Some((copied_drawing, z_index)) = copied_drawing.clone() {
            let mut path_builder = PathBuilder::new();
            let mut points_iter = copied_drawing.points.iter();
            let start = points_iter.next().unwrap();
            path_builder.move_to(*start);
            path_builder.line_to(*start);
            for point in points_iter {
                path_builder.line_to(*point);
            }
            let path = path_builder.build();
            commands.spawn((
                ShapeBundle {
                    path,
                    transform: Transform::from_xyz(x, y, z_index + 0.01),
                    ..Default::default()
                },
                Stroke::new(copied_drawing.drawing_color.1, 2.),
                Drawing {
                    id: ReflectableUuid::generate(),
                    points: copied_drawing.points.clone(),
                    drawing_color: copied_drawing.drawing_color,
                },
                InteractiveNode,
            ));
        }
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
    } else if command && input.just_pressed(KeyCode::P) {
        events.send(AddRect {
            node: JsonNode {
                id: Uuid::new_v4(),
                node_type: NodeType::Paper,
                x,
                y,
                width: theme.node_width,
                height: theme.node_height,
                text: JsonNodeText {
                    text: "".to_string(),
                    pos: crate::TextPos::Center,
                },
                bg_color: pair_struct!(theme.paper_node_bg),
                ..Default::default()
            },
            image: None,
        });
        input.release_all()
    } else if command && input.just_pressed(KeyCode::R) {
        events.send(AddRect {
            node: JsonNode {
                id: Uuid::new_v4(),
                node_type: NodeType::Rect,
                x,
                y,
                width: theme.node_width,
                height: theme.node_height,
                text: JsonNodeText {
                    text: "".to_string(),
                    pos: crate::TextPos::Center,
                },
                bg_color: pair_struct!(theme.node_bg),
                ..default()
            },
            image: None,
        });
        input.release_all()
    } else if command && input.just_pressed(KeyCode::O) {
        events.send(AddRect {
            node: JsonNode {
                id: Uuid::new_v4(),
                node_type: NodeType::Circle,
                x,
                y,
                width: theme.node_width,
                height: theme.node_height,
                text: JsonNodeText {
                    text: "".to_string(),
                    pos: crate::TextPos::Center,
                },
                bg_color: pair_struct!(theme.node_bg),
                ..default()
            },
            image: None,
        });
        input.release_all()
    } else if input.just_pressed(KeyCode::Delete) {
        if let Some(id) = ui_state.entity_to_edit {
            for (entity, node) in velo_node_query.iter() {
                if node.id == id {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
        input.release_all()
    } else {
        for (editable_text, mut cosmic_edit, mut cosmit_edit_history) in
            &mut editable_text_query.iter_mut()
        {
            if [ui_state.tab_to_edit, ui_state.doc_to_edit].contains(&Some(editable_text.id)) {
                if input.any_just_pressed([KeyCode::Escape, KeyCode::Return]) {
                    commands.insert_resource(ActiveEditor { entity: None });
                    cosmic_edit.readonly = true;
                    let mut current_cursor = cosmic_edit.editor.cursor();
                    if ui_state.doc_to_edit.is_some() {
                        current_cursor.color = Some(bevy_color_to_cosmic(theme.doc_list_bg));
                        cosmic_edit.editor.set_cursor(current_cursor);
                    }
                    if ui_state.tab_to_edit.is_some() {
                        current_cursor.color = Some(bevy_color_to_cosmic(theme.add_tab_bg));
                        cosmic_edit.editor.set_cursor(current_cursor);
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
    x: f32,
    y: f32,
    scale_factor: f64,
    theme: &Res<Theme>,
) {
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
                    visible: true,
                    id: Uuid::new_v4(),
                    node_type: crate::NodeType::Rect,
                    x,
                    y,
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
