use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_cosmic_edit::{
    cosmic_edit_set_text, get_cosmic_text, get_text_spans, ActiveEditor, CosmicEdit,
    CosmicEditHistory, CosmicFont, CosmicText, EditHistoryItem,
};
use bevy_markdown::{generate_markdown_lines, BevyMarkdown, BevyMarkdownTheme};
use bevy_prototype_lyon::prelude::Stroke;
use cosmic_text::{Cursor, Edit};

use crate::{
    resources::{AppState, SaveDocRequest},
    themes::Theme,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
};

use super::{ui_helpers::VeloShape, BevyMarkdownView, NodeType, RawText, UiState};

pub fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    app_state: Res<AppState>,
    theme: Res<Theme>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_border: Query<(&mut Stroke, &VeloShape), With<VeloShape>>,
    mut raw_text_node_query: Query<
        (
            Entity,
            &mut RawText,
            &mut CosmicEdit,
            &mut CosmicEditHistory,
        ),
        With<RawText>,
    >,
    mut commands: Commands,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    if ui_state.is_changed() && ui_state.entity_to_edit != *last_entity_to_edit {
        match ui_state.entity_to_edit {
            Some(entity_to_edit) => {
                // Change border for selected node
                for (mut stroke, velo_border) in velo_border.iter_mut() {
                    if velo_border.id == entity_to_edit {
                        stroke.color = theme.selected_node_border;
                        stroke.options.line_width = 2.;
                    } else {
                        let has_border = velo_border.node_type.clone() != NodeType::Paper;
                        if has_border {
                            stroke.color = theme.node_border;
                        } else {
                            stroke.color = Color::NONE;
                        };
                        stroke.options.line_width = 1.;
                    }
                }

                for (entity, mut raw_text, mut cosmic_edit, mut cosmic_edit_history) in
                    raw_text_node_query.iter_mut()
                {
                    // cosmic-edit editing mode
                    if raw_text.id == entity_to_edit {
                        cosmic_edit.readonly = false;
                        let current_cursor = cosmic_edit.editor.cursor();
                        let new_cursor = Cursor::new_with_color(
                            current_cursor.line,
                            current_cursor.index,
                            bevy_color_to_cosmic(theme.font),
                        );
                        cosmic_edit.editor.set_cursor(new_cursor);
                        let text = raw_text.last_text.clone();
                        let font = cosmic_fonts
                            .get_mut(&cosmic_edit.font_system.clone())
                            .unwrap();
                        cosmic_edit_set_text(
                            CosmicText::OneStyle(text),
                            cosmic_edit.attrs.clone(),
                            &mut cosmic_edit.editor,
                            &mut font.0,
                        );
                        commands.insert_resource(ActiveEditor {
                            entity: Some(entity),
                        });
                        cosmic_edit.editor.buffer_mut().set_redraw(true);
                    }
                    // cosmic-edit readonly mode
                    if Some(raw_text.id) == *last_entity_to_edit {
                        cosmic_edit.readonly = true;
                        let cursor_color = cosmic_edit.bg;
                        let current_cursor = cosmic_edit.editor.cursor();
                        let new_cursor = Cursor::new_with_color(
                            current_cursor.line,
                            current_cursor.index,
                            bevy_color_to_cosmic(cursor_color),
                        );
                        cosmic_edit.editor.set_cursor(new_cursor);
                        let mut edits = VecDeque::new();
                        edits.push_back(EditHistoryItem {
                            cursor: new_cursor,
                            lines: get_text_spans(
                                cosmic_edit.editor.buffer(),
                                cosmic_edit.attrs.clone(),
                            ),
                        });
                        *cosmic_edit_history = CosmicEditHistory {
                            edits,
                            current_edit: 0,
                        };
                        let text = get_cosmic_text(cosmic_edit.editor.buffer());
                        raw_text.last_text = text.clone();
                        let markdown_theme = BevyMarkdownTheme {
                            code_theme: theme.code_theme.clone(),
                            code_default_lang: theme.code_default_lang.clone(),
                            link: bevy_color_to_cosmic(theme.link),
                            inline_code: bevy_color_to_cosmic(theme.inline_code),
                        };
                        let markdown_lines = generate_markdown_lines(BevyMarkdown {
                            text,
                            markdown_theme,
                            attrs: cosmic_edit.attrs.clone(),
                        })
                        .expect("should handle markdown convertion");
                        let font = cosmic_fonts
                            .get_mut(&cosmic_edit.font_system.clone())
                            .unwrap();
                        cosmic_edit_set_text(
                            CosmicText::MultiStyle(markdown_lines.lines),
                            cosmic_edit.attrs.clone(),
                            &mut cosmic_edit.editor,
                            &mut font.0,
                        );
                        commands.entity(entity).insert(BevyMarkdownView {
                            id: raw_text.id,
                            span_metadata: markdown_lines.span_metadata,
                        });
                        cosmic_edit.editor.buffer_mut().set_redraw(true);
                    }
                }
            }
            None => {
                for (mut stroke, velo_border) in velo_border.iter_mut() {
                    let has_border = velo_border.node_type.clone() != NodeType::Paper;
                    if has_border {
                        stroke.color = theme.node_border;
                    } else {
                        stroke.color = Color::NONE;
                    };
                    stroke.options.line_width = 1.;
                }
                for (entity, mut raw_text, mut cosmic_edit, mut cosmic_edit_history) in
                    raw_text_node_query.iter_mut()
                {
                    // cosmic-edit readonly mode
                    if Some(raw_text.id) == *last_entity_to_edit {
                        cosmic_edit.readonly = true;
                        let cursor_color = cosmic_edit.bg;
                        let current_cursor = cosmic_edit.editor.cursor();
                        let new_cursor = Cursor::new_with_color(
                            current_cursor.line,
                            current_cursor.index,
                            bevy_color_to_cosmic(cursor_color),
                        );
                        cosmic_edit.editor.set_cursor(new_cursor);
                        let mut edits = VecDeque::new();
                        edits.push_back(EditHistoryItem {
                            cursor: new_cursor,
                            lines: get_text_spans(
                                cosmic_edit.editor.buffer(),
                                cosmic_edit.attrs.clone(),
                            ),
                        });
                        *cosmic_edit_history = CosmicEditHistory {
                            edits,
                            current_edit: 0,
                        };
                        let text = get_cosmic_text(cosmic_edit.editor.buffer());
                        raw_text.last_text = text.clone();
                        let markdown_theme = BevyMarkdownTheme {
                            code_theme: theme.code_theme.clone(),
                            code_default_lang: theme.code_default_lang.clone(),
                            link: bevy_color_to_cosmic(theme.link),
                            inline_code: bevy_color_to_cosmic(theme.inline_code),
                        };
                        let markdown_lines = generate_markdown_lines(BevyMarkdown {
                            text,
                            markdown_theme,
                            attrs: cosmic_edit.attrs.clone(),
                        })
                        .expect("should handle markdown convertion");
                        let font = cosmic_fonts
                            .get_mut(&cosmic_edit.font_system.clone())
                            .unwrap();
                        cosmic_edit.attrs = cosmic_edit.attrs.clone();
                        cosmic_edit_set_text(
                            CosmicText::MultiStyle(markdown_lines.lines),
                            cosmic_edit.attrs.clone(),
                            &mut cosmic_edit.editor,
                            &mut font.0,
                        );
                        commands.entity(entity).insert(BevyMarkdownView {
                            id: raw_text.id,
                            span_metadata: markdown_lines.span_metadata,
                        });
                        cosmic_edit.editor.buffer_mut().set_redraw(true);
                    }
                }

                if let Some(current_document) = app_state.current_document {
                    commands.insert_resource(SaveDocRequest {
                        doc_id: current_document,
                        path: None,
                    });
                }
            }
        }
        *last_entity_to_edit = ui_state.entity_to_edit;
    }
}
