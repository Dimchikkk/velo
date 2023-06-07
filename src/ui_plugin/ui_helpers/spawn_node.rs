use bevy_cosmic_edit::{
    bevy_color_to_cosmic, spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditUi,
    CosmicFont, CosmicMetrics, CosmicNode, CosmicText,
};
use bevy_markdown::{spawn_bevy_markdown, BevyMarkdown, BevyMarkdownFonts, BevyMarkdownTheme};
use bevy_ui_borders::{BorderColor, Outline};

use bevy::prelude::*;

use crate::themes::Theme;
use crate::ui_plugin::NodeType;
use crate::TextPos;

use super::{
    create_arrow_marker, create_rectangle_btn, create_resize_marker, BevyMarkdownView, RawText,
    ResizeMarker, VeloNode, VeloNodeContainer,
};
use crate::canvas::arrow::components::{ArrowConnect, ArrowConnectPos};
use crate::utils::{convert_from_val_px, to_cosmic_text_pos, ReflectableUuid};

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
    pub node_type: NodeType,
    pub size: (Val, Val),
    pub position: (Val, Val),
    pub text: String,
    pub bg_color: Color,
    pub image: Option<UiImage>,
    pub text_pos: TextPos,
    pub z_index: i32,
    pub is_active: bool,
}

pub fn spawn_node(
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    scale_factor: f32,
    item_meta: NodeMeta,
) -> Entity {
    let top = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: item_meta.position.0,
                        bottom: item_meta.position.1,
                        ..Default::default()
                    },
                    size: Size::new(item_meta.size.0, item_meta.size.1),
                    ..default()
                },
                ..default()
            },
            VeloNodeContainer { id: item_meta.id },
        ))
        .id();
    let image = match item_meta.node_type {
        NodeType::Rect => item_meta.image,
        NodeType::Circle => {
            #[cfg(not(target_arch = "wasm32"))]
            let image = Some(asset_server.load("circle-node.basis").into());
            #[cfg(target_arch = "wasm32")]
            let image = Some(asset_server.load("circle-node.png").into());
            image
        }
    };
    let button = commands
        .spawn((
            create_rectangle_btn(
                item_meta.bg_color,
                image,
                item_meta.z_index,
                item_meta.text_pos.clone(),
            ),
            VeloNode {
                id: item_meta.id,
                node_type: item_meta.node_type.clone(),
            },
        ))
        .id();
    let outline_color = match item_meta.node_type {
        NodeType::Rect => theme.node_border,
        NodeType::Circle => theme.node_border.with_a(0.),
    };
    commands
        .entity(button)
        .insert(Outline::all(outline_color, Val::Px(1.)));
    let arrow_marker1 = commands
        .spawn((
            create_arrow_marker(50.0, 0., 0., 0.),
            BorderColor(theme.arrow_connector),
            ArrowConnect {
                pos: ArrowConnectPos::Top,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker2 = commands
        .spawn((
            create_arrow_marker(0., 0., 50., 0.),
            BorderColor(theme.arrow_connector),
            ArrowConnect {
                pos: ArrowConnectPos::Left,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker3 = commands
        .spawn((
            create_arrow_marker(50., 0., 100., 0.),
            BorderColor(theme.arrow_connector),
            ArrowConnect {
                pos: ArrowConnectPos::Bottom,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker4 = commands
        .spawn((
            create_arrow_marker(100., 0., 50., 0.),
            BorderColor(theme.arrow_connector),
            ArrowConnect {
                pos: ArrowConnectPos::Right,
                id: item_meta.id,
            },
        ))
        .id();
    let resize_marker1 = commands
        .spawn((create_resize_marker(0., 0., 0., 0.), ResizeMarker::TopLeft))
        .id();
    let resize_marker2 = commands
        .spawn((
            create_resize_marker(100., 0., 0., 0.),
            ResizeMarker::TopRight,
        ))
        .id();
    let resize_marker3 = commands
        .spawn((
            create_resize_marker(100., 0., 100., 0.),
            ResizeMarker::BottomRight,
        ))
        .id();
    let resize_marker4 = commands
        .spawn((
            create_resize_marker(0., 0., 100., 0.),
            ResizeMarker::BottomLeft,
        ))
        .id();
    commands.entity(button).add_child(arrow_marker1);
    commands.entity(button).add_child(arrow_marker2);
    commands.entity(button).add_child(arrow_marker3);
    commands.entity(button).add_child(arrow_marker4);
    commands.entity(button).add_child(resize_marker1);
    commands.entity(button).add_child(resize_marker2);
    commands.entity(button).add_child(resize_marker3);
    commands.entity(button).add_child(resize_marker4);
    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
    attrs = attrs.color(bevy_color_to_cosmic(theme.font));
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle((item_meta.text.clone(), attrs)),
        font_system_handle: cosmic_font_handle,
        text_pos: to_cosmic_text_pos(item_meta.text_pos),
        size: Some((
            convert_from_val_px(item_meta.size.0),
            convert_from_val_px(item_meta.size.1),
        )),
        node: CosmicNode::Ui(CosmicEditUi {
            display_none: !item_meta.is_active,
        }),
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor,
        },
        bg: theme.node_bg,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(RawText { id: item_meta.id });
    commands.entity(button).add_child(cosmic_edit);

    match item_meta.is_active {
        true => {
            commands.insert_resource(ActiveEditor {
                entity: Some(cosmic_edit),
            });
        }
        false => {
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
                text: item_meta.text.clone(),
                fonts,
                theme,
                size: Some(item_meta.size),
            };
            let markdown_text = spawn_bevy_markdown(commands, bevy_markdown)
                .expect("should handle markdown convertion");
            commands
                .get_entity(markdown_text)
                .unwrap()
                .insert(BevyMarkdownView { id: item_meta.id });
            commands.entity(button).add_child(markdown_text);
        }
    }
    commands.entity(top).add_child(button);
    top
}
