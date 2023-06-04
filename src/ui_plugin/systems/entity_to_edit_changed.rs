use bevy::prelude::*;
use bevy_cosmic_edit::{get_cosmic_text, ActiveEditor, CosmicEdit};
use bevy_markdown::{spawn_bevy_markdown, BevyMarkdown, BevyMarkdownFonts, BevyMarkdownTheme};

use crate::{
    resources::{AppState, SaveDocRequest},
    themes::Theme,
    utils::ReflectableUuid,
};
use bevy_ui_borders::Outline;

use super::{BevyMarkdownView, NodeType, RawText, UiState, VeloNode};

pub fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    app_state: Res<AppState>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_node_query: Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    mut raw_text_node_query: Query<
        (&mut Style, &RawText, &Parent, Entity, &CosmicEdit, &Node),
        With<RawText>,
    >,
    mut markdown_text_node_query: Query<(Entity, &BevyMarkdownView), With<BevyMarkdownView>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<Theme>,
) {
    if ui_state.is_changed() && ui_state.entity_to_edit != *last_entity_to_edit {
        match ui_state.entity_to_edit {
            Some(entity_to_edit) => {
                handle_entity_selection(
                    entity_to_edit,
                    &mut velo_node_query,
                    &mut raw_text_node_query,
                    &mut markdown_text_node_query,
                    &mut commands,
                    &asset_server,
                    &theme,
                );
            }
            None => {
                handle_no_entity_selection(
                    &mut velo_node_query,
                    &mut raw_text_node_query,
                    &mut commands,
                    &asset_server,
                    &theme,
                );
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

fn handle_entity_selection(
    entity_to_edit: ReflectableUuid,
    velo_node_query: &mut Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    raw_text_node_query: &mut Query<
        (&mut Style, &RawText, &Parent, Entity, &CosmicEdit, &Node),
        With<RawText>,
    >,
    markdown_text_node_query: &mut Query<(Entity, &BevyMarkdownView), With<BevyMarkdownView>>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    theme: &Res<Theme>,
) {
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

    // Hide raw text and have markdown view for all nodes (except selected)
    for (mut style, raw_text, parent, entity, editor, node) in raw_text_node_query.iter_mut() {
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
        let str = get_cosmic_text(&editor.editor);
        let fonts = BevyMarkdownFonts {
            regular_font: TextStyle::default().font,
            code_font: TextStyle::default().font,
            bold_font: asset_server.load("fonts/SourceCodePro-Bold.ttf"),
            italic_font: asset_server.load("fonts/SourceCodePro-Italic.ttf"),
            semi_bold_italic_font: asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
            extra_bold_font: asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
        };
        let theme = BevyMarkdownTheme {
            code_theme: theme.code_theme.clone(),
            code_default_lang: theme.code_default_lang.clone(),
            font: theme.font,
            link: theme.link,
            inline_code: theme.inline_code,
        };
        let bevy_markdown = BevyMarkdown {
            text: str,
            fonts,
            theme,
            size: Some((Val::Px(node.size().x), Val::Px(node.size().y))),
        };
        let markdown_text = spawn_bevy_markdown(commands, bevy_markdown)
            .expect("should handle markdown conversion");
        commands
            .get_entity(markdown_text)
            .unwrap()
            .insert(BevyMarkdownView { id: raw_text.id });
        let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
        commands.entity(entity).add_child(markdown_text);
    }

    // Remove markdown view for selected node
    for (entity, node) in markdown_text_node_query.iter_mut() {
        if node.id == entity_to_edit {
            commands.entity(entity).despawn_recursive();
            break;
        }
    }
}

fn handle_no_entity_selection(
    velo_node_query: &mut Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    raw_text_node_query: &mut Query<
        (&mut Style, &RawText, &Parent, Entity, &CosmicEdit, &Node),
        With<RawText>,
    >,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    theme: &Res<Theme>,
) {
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

    // Hide raw text and have markdown view for all nodes
    for (mut style, raw_text, parent, _, editor, node) in raw_text_node_query.iter_mut() {
        if style.display == Display::None {
            continue;
        }
        style.display = Display::None;
        let str = get_cosmic_text(&editor.editor);
        let fonts = BevyMarkdownFonts {
            regular_font: TextStyle::default().font,
            code_font: TextStyle::default().font,
            bold_font: asset_server.load("fonts/SourceCodePro-Bold.ttf"),
            italic_font: asset_server.load("fonts/SourceCodePro-Italic.ttf"),
            semi_bold_italic_font: asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
            extra_bold_font: asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
        };
        let theme = BevyMarkdownTheme {
            code_theme: theme.code_theme.clone(),
            code_default_lang: theme.code_default_lang.clone(),
            font: theme.font,
            link: theme.link,
            inline_code: theme.inline_code,
        };
        let bevy_markdown = BevyMarkdown {
            text: str,
            fonts,
            theme,
            size: Some((Val::Px(node.size().x), Val::Px(node.size().y))),
        };
        let markdown_text = spawn_bevy_markdown(commands, bevy_markdown).unwrap();
        commands
            .get_entity(markdown_text)
            .unwrap()
            .insert(BevyMarkdownView { id: raw_text.id });
        let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
        commands.entity(entity).add_child(markdown_text);
    }
}
