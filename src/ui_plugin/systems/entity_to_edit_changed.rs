use bevy::prelude::*;
use bevy_cosmic_edit::ActiveEditor;
use bevy_markdown::{spawn_bevy_markdown, BevyMarkdown};

use bevy_ui_borders::Outline;

use crate::utils::ReflectableUuid;

use super::{BevyMarkdownView, NodeType, RawText, UiState, VeloNode};

pub fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_node_query: Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    mut raw_text_node_query: Query<(&mut Style, &RawText, &Parent, Entity), With<RawText>>,
    mut markdown_text_node_query: Query<(Entity, &BevyMarkdownView), With<BevyMarkdownView>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if ui_state.is_changed() && ui_state.entity_to_edit != *last_entity_to_edit {
        match ui_state.entity_to_edit {
            Some(entity_to_edit) => {
                // change border for selected node
                {
                    for (mut outline, node, _) in &mut velo_node_query.iter_mut() {
                        if node.id == entity_to_edit {
                            outline.color =
                                Color::rgba(33.0 / 255.0, 150.0 / 255.0, 243.0 / 255.0, 1.0);
                            outline.thickness = UiRect::all(Val::Px(2.));
                        } else {
                            match node.node_type {
                                NodeType::Rect => {
                                    outline.color =
                                        Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0);
                                }
                                NodeType::Circle => {
                                    outline.color =
                                        Color::rgba(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0, 0.);
                                }
                            }

                            outline.thickness = UiRect::all(Val::Px(1.));
                        }
                    }
                }
                // hide raw text and have markdown view for all nodes (except selected)
                {
                    for (mut style, raw_text, parent, entity) in &mut raw_text_node_query.iter_mut()
                    {
                        if raw_text.id == entity_to_edit {
                            commands.insert_resource(ActiveEditor {
                                entity: Some(entity),
                            });
                            style.display = Display::Flex;
                            continue;
                        }
                        if style.display == Display::None {
                            continue;
                        }
                        style.display = Display::None;
                        let str = "".to_string();
                        // TODO: imlement getting text
                        let bevy_markdown = BevyMarkdown {
                            text: str,
                            regular_font: Some(
                                asset_server.load("fonts/SourceCodePro-Regular.ttf"),
                            ),
                            code_font: Some(TextStyle::default().font),
                            bold_font: Some(asset_server.load("fonts/SourceCodePro-Bold.ttf")),
                            italic_font: Some(asset_server.load("fonts/SourceCodePro-Italic.ttf")),
                            semi_bold_italic_font: Some(
                                asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
                            ),
                            extra_bold_font: Some(
                                asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
                            ),
                            size: Some((style.max_size.width, style.max_size.height)),
                        };
                        let markdown_text = spawn_bevy_markdown(&mut commands, bevy_markdown)
                            .expect("should handle markdown convertion");
                        commands
                            .get_entity(markdown_text)
                            .unwrap()
                            .insert(BevyMarkdownView { id: raw_text.id });
                        let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
                        commands.entity(entity).add_child(markdown_text);
                    }
                }
                // remove markdown view for selected node
                {
                    for (entity, node) in &mut markdown_text_node_query.iter_mut() {
                        if node.id == entity_to_edit {
                            commands.entity(entity).despawn_recursive();
                            break;
                        }
                    }
                }
            }
            None => {
                {
                    commands.insert_resource(ActiveEditor { entity: None });
                }
                // change border
                {
                    for (mut outline, node, _) in &mut velo_node_query.iter_mut() {
                        match node.node_type {
                            NodeType::Rect => {
                                outline.color =
                                    Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0);
                            }
                            NodeType::Circle => {
                                outline.color =
                                    Color::rgba(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0, 0.);
                            }
                        }
                        outline.thickness = UiRect::all(Val::Px(1.));
                    }
                }
                for (mut style, raw_text, parent, _) in &mut raw_text_node_query.iter_mut() {
                    if style.display == Display::None {
                        continue;
                    }
                    style.display = Display::None;
                    let str = "".to_string();
                    // TODO: imlement getting text
                    let bevy_markdown = BevyMarkdown {
                        text: str,
                        regular_font: Some(TextStyle::default().font),
                        code_font: Some(TextStyle::default().font),
                        bold_font: Some(asset_server.load("fonts/SourceCodePro-Bold.ttf")),
                        italic_font: Some(asset_server.load("fonts/SourceCodePro-Italic.ttf")),
                        semi_bold_italic_font: Some(
                            asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
                        ),
                        extra_bold_font: Some(
                            asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
                        ),
                        size: Some((style.max_size.width, style.max_size.height)),
                    };
                    let markdown_text = spawn_bevy_markdown(&mut commands, bevy_markdown).unwrap();
                    commands
                        .get_entity(markdown_text)
                        .unwrap()
                        .insert(BevyMarkdownView { id: raw_text.id });
                    let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
                    commands.entity(entity).add_child(markdown_text);
                }
            }
        }
        *last_entity_to_edit = ui_state.entity_to_edit;
    }
}
