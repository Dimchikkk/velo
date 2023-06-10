use bevy::prelude::*;
use bevy_cosmic_edit::{
    cosmic_edit_set_text, get_cosmic_text, ActiveEditor, CosmicEdit, CosmicFont, CosmicText,
};
use bevy_markdown::{generate_markdown_lines, BevyMarkdown, BevyMarkdownTheme};
use cosmic_text::{Cursor, Edit};

use crate::{
    resources::{AppState, SaveDocRequest},
    themes::Theme,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
};
use bevy_ui_borders::Outline;

use super::{BevyMarkdownView, NodeType, RawText, UiState, VeloNode};

pub fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    app_state: Res<AppState>,
    theme: Res<Theme>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_node_query: Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    mut raw_text_node_query: Query<(Entity, &mut RawText, &mut CosmicEdit), With<RawText>>,
    mut commands: Commands,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    if ui_state.is_changed() && ui_state.entity_to_edit != *last_entity_to_edit {
        match ui_state.entity_to_edit {
            Some(entity_to_edit) => {
                // Change border for selected node
                for (mut outline, node, _) in velo_node_query.iter_mut() {
                    if node.id == entity_to_edit {
                        outline.color = theme.selected_node_border;
                        outline.thickness = UiRect::all(Val::Px(2.));
                    } else {
                        match node.node_type {
                            NodeType::Rect => {
                                outline.color = theme.node_border;
                            }
                            NodeType::Circle => {
                                outline.color = theme.node_border.with_a(0.);
                            }
                        }

                        outline.thickness = UiRect::all(Val::Px(1.));
                    }
                }

                for (entity, mut raw_text, mut cosmic_edit) in raw_text_node_query.iter_mut() {
                    // cosmic-edit editing mode
                    if raw_text.id == entity_to_edit {
                        cosmic_edit.readonly = false;
                        cosmic_edit.editor.set_cursor(Cursor::default());
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
                        cosmic_edit.editor.set_cursor(Cursor::new_with_color(
                            0,
                            0,
                            bevy_color_to_cosmic(theme.node_bg),
                        ));
                        let text = get_cosmic_text(&cosmic_edit.editor);
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
                // Reset border colors and thickness for all nodes
                for (mut outline, node, _) in velo_node_query.iter_mut() {
                    match node.node_type {
                        NodeType::Rect => {
                            outline.color = theme.node_border;
                        }
                        NodeType::Circle => {
                            outline.color = theme.node_border.with_a(0.);
                        }
                    }
                    outline.thickness = UiRect::all(Val::Px(1.));
                }
                for (entity, mut raw_text, mut cosmic_edit) in raw_text_node_query.iter_mut() {
                    // cosmic-edit readonly mode
                    if Some(raw_text.id) == *last_entity_to_edit {
                        cosmic_edit.readonly = true;
                        cosmic_edit.editor.set_cursor(Cursor::new_with_color(
                            0,
                            0,
                            bevy_color_to_cosmic(theme.node_bg),
                        ));
                        let text = get_cosmic_text(&cosmic_edit.editor);
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
