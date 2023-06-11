use std::fmt::format;

use bevy_cosmic_edit::{
    spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditSprite, CosmicFont, CosmicMetrics,
    CosmicNode, CosmicText,
};
use bevy_markdown::{generate_markdown_lines, BevyMarkdown, BevyMarkdownTheme};
use bevy_prototype_lyon::prelude::{Fill, Stroke};
use bevy_smud::prelude::SdfAssets;
use bevy_smud::{Frame, ShapeBundle};
use bevy_ui_borders::{BorderColor, Outline};

use bevy::prelude::*;
use cosmic_text::AttrsOwned;

use crate::themes::Theme;
use crate::ui_plugin::NodeType;
use crate::TextPos;

use super::{
    create_arrow_marker, create_rectangle_btn, create_resize_marker, BevyMarkdownView, RawText,
    ResizeMarker, VeloNode, VeloNodeContainer,
};
use crate::canvas::arrow::components::{ArrowConnect, ArrowConnectPos};
use crate::utils::{
    bevy_color_to_cosmic, convert_from_val_px, to_cosmic_text_pos, ReflectableUuid,
};

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
    pub node_type: NodeType,
    pub size: (Val, Val),
    pub position: (Val, Val),
    pub text: String,
    pub bg_color: Color,
    pub image: Option<Handle<Image>>,
    pub text_pos: TextPos,
    pub z_index: i32,
    pub is_active: bool,
}

pub fn spawn_sprite_node(
    shaders: &mut ResMut<Assets<Shader>>,
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    scale_factor: f32,
    item_meta: NodeMeta,
) -> Entity {
    let z_index = item_meta.z_index as f32;
    let scale = Vec3::new(1. / scale_factor, 1. / scale_factor, 1.);
    let pos: Vec3 = Vec3::new(0.0, 0.0, z_index);
    let width: f32 = convert_from_val_px(item_meta.size.0);
    let height = convert_from_val_px(item_meta.size.1);

    let top = commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..Default::default()
        })
        .id();

    let points = [
        Vec2::new(pos.x - width / 2., pos.y - height / 2.),
        Vec2::new(pos.x - width / 2., pos.y + height / 2.),
        Vec2::new(pos.x + width / 2., pos.y + height / 2.),
        Vec2::new(pos.x + width / 2., pos.y - height / 2.),
    ];

    let shape = bevy_prototype_lyon::shapes::RoundedPolygon {
        points: points.into_iter().collect(),
        radius: 10.,
        closed: true,
    };

    let border = commands
        .spawn((
            bevy_prototype_lyon::prelude::ShapeBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.1),
                    ..default()
                },
                path: bevy_prototype_lyon::prelude::GeometryBuilder::build_as(&shape),
                ..default()
            },
            Stroke::new(theme.node_border, 2.),
            Fill::color(theme.node_bg),
        ))
        .id();

    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
    attrs = attrs.color(bevy_color_to_cosmic(theme.font));
    let (text, span_metadata) = match item_meta.is_active {
        true => (CosmicText::OneStyle(item_meta.text.clone()), vec![]),
        false => {
            let markdown_theme = BevyMarkdownTheme {
                code_theme: theme.code_theme.clone(),
                code_default_lang: theme.code_default_lang.clone(),
                link: bevy_color_to_cosmic(theme.link),
                inline_code: bevy_color_to_cosmic(theme.inline_code),
            };
            let markdown_lines = generate_markdown_lines(BevyMarkdown {
                text: item_meta.text.clone(),
                attrs: AttrsOwned::new(attrs),
                markdown_theme,
            })
            .expect("should handle markdown convertion");
            (
                CosmicText::MultiStyle(markdown_lines.lines),
                markdown_lines.span_metadata,
            )
        }
    };

    let cosmic_edit_meta = CosmicEditMeta {
        text,
        font_system_handle: cosmic_font_handle,
        text_pos: to_cosmic_text_pos(item_meta.text_pos.clone()),
        size: Some((width, height)),
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.2),
                scale,
                ..default()
            },
        }),
        metrics: CosmicMetrics {
            font_size: theme.font_size,
            line_height: theme.line_height,
            scale_factor,
        },
        bg: Color::NONE,
        bg_image: item_meta.image,
        readonly: !item_meta.is_active,
        attrs: AttrsOwned::new(attrs),
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands.entity(cosmic_edit).insert(RawText {
        id: item_meta.id,
        last_text: item_meta.text.clone(),
    });

    match item_meta.is_active {
        true => {
            commands.insert_resource(ActiveEditor {
                entity: Some(cosmic_edit),
            });
        }
        false => {
            commands.entity(cosmic_edit).insert(BevyMarkdownView {
                id: item_meta.id,
                span_metadata,
            });
        }
    }

    let fill = shaders.add_fill_body(
        r"
        let size = 100.;
        let power = 3.0;
        var a = (size - d) / size;
        a = clamp(a, 0.0, 1.0);
        a = pow(a, power);
        return vec4<f32>(color.rgb, a * color.a);
    ",
    );
    let sdf_expr = format!("sd_box(p, vec2<f32>({:.2}, {:.2}))", width - 50., 10.);
    let sdf = shaders.add_sdf_expr(sdf_expr);

    let shadow = commands
        .spawn(ShapeBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -height / 2. + 15., 0.09),
                scale,
                ..default()
            },
            shape: bevy_smud::SmudShape {
                color: Color::BLACK,
                sdf,
                fill,
                frame: Frame::Quad(width + 2.),
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(top).add_child(border);
    commands.entity(top).add_child(shadow);
    commands.entity(border).add_child(cosmic_edit);
    border
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
        NodeType::Circle => Some(asset_server.load("circle-node.png")),
    };
    let button = commands
        .spawn((
            create_rectangle_btn(
                item_meta.bg_color,
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

    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
    attrs = attrs.color(bevy_color_to_cosmic(theme.font));
    let (text, span_metadata) = match item_meta.is_active {
        true => (CosmicText::OneStyle(item_meta.text.clone()), vec![]),
        false => {
            let markdown_theme = BevyMarkdownTheme {
                code_theme: theme.code_theme.clone(),
                code_default_lang: theme.code_default_lang.clone(),
                link: bevy_color_to_cosmic(theme.link),
                inline_code: bevy_color_to_cosmic(theme.inline_code),
            };
            let markdown_lines = generate_markdown_lines(BevyMarkdown {
                text: item_meta.text.clone(),
                attrs: AttrsOwned::new(attrs),
                markdown_theme,
            })
            .expect("should handle markdown convertion");
            (
                CosmicText::MultiStyle(markdown_lines.lines),
                markdown_lines.span_metadata,
            )
        }
    };

    let cosmic_edit_meta = CosmicEditMeta {
        text,
        font_system_handle: cosmic_font_handle,
        text_pos: to_cosmic_text_pos(item_meta.text_pos.clone()),
        size: Some((
            convert_from_val_px(item_meta.size.0),
            convert_from_val_px(item_meta.size.1),
        )),
        node: CosmicNode::Ui,
        metrics: CosmicMetrics {
            font_size: theme.font_size,
            line_height: theme.line_height,
            scale_factor,
        },
        bg: item_meta.bg_color,
        bg_image: image,
        readonly: !item_meta.is_active,
        attrs: AttrsOwned::new(attrs),
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands.entity(cosmic_edit).insert(RawText {
        id: item_meta.id,
        last_text: item_meta.text.clone(),
    });
    commands.entity(button).add_child(cosmic_edit);

    match item_meta.is_active {
        true => {
            commands.insert_resource(ActiveEditor {
                entity: Some(cosmic_edit),
            });
        }
        false => {
            commands.entity(cosmic_edit).insert(BevyMarkdownView {
                id: item_meta.id,
                span_metadata,
            });
        }
    }

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

    commands.entity(top).add_child(button);
    top
}
