use bevy_cosmic_edit::{
    spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditSprite, CosmicFont, CosmicMetrics,
    CosmicNode, CosmicText,
};
use bevy_markdown::{generate_markdown_lines, BevyMarkdown, BevyMarkdownTheme};
use bevy_prototype_lyon::prelude::{Fill, Path, Stroke};
use bevy_smud::prelude::SdfAssets;
use bevy_smud::{Frame, ShapeBundle};
use bevy_ui_borders::{BorderColor, Outline};

use bevy::prelude::*;
use cosmic_text::AttrsOwned;

use crate::themes::Theme;
use crate::ui_plugin::NodeType;
use crate::TextPos;

use super::{
    create_arrow_marker, create_rectangle_btn, create_resize_marker, spawn_shadow,
    BevyMarkdownView, InteractiveNode, RawText, ResizeMarker, VeloBorder, VeloNode,
    VeloNodeContainer, VeloShadow,
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
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                ..Default::default()
            },
            VeloNode { id: item_meta.id },
        ))
        .id();

    let points = [
        Vec2::new(-width / 2., -height / 2.),
        Vec2::new(-width / 2., height / 2.),
        Vec2::new(width / 2., height / 2.),
        Vec2::new(width / 2., -height / 2.),
    ];

    let path: Path = match item_meta.node_type.clone() {
        NodeType::Rect => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
            &bevy_prototype_lyon::shapes::RoundedPolygon {
                points: points.into_iter().collect(),
                closed: true,
                radius: 10.,
            },
        ),
        NodeType::Paper => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
            &bevy_prototype_lyon::shapes::Polygon {
                points: points.into_iter().collect(),
                closed: true,
            },
        ),
        NodeType::Circle => bevy_prototype_lyon::prelude::GeometryBuilder::build_as(
            &bevy_prototype_lyon::shapes::Circle {
                radius: width / 2.,
                center: Vec2::new(0., 0.),
            },
        ),
    };
    let has_no_border = item_meta.node_type.clone() == NodeType::Paper;
    let border = commands
        .spawn((
            bevy_prototype_lyon::prelude::ShapeBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.1),
                    ..default()
                },
                path,
                ..default()
            },
            Stroke::new(
                if has_no_border {
                    item_meta.bg_color
                } else {
                    theme.node_border
                },
                1.,
            ),
            Fill::color(item_meta.bg_color),
            VeloBorder {
                id: item_meta.id,
                node_type: item_meta.node_type.clone(),
            },
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
    commands
        .entity(cosmic_edit)
        .insert(RawText {
            id: item_meta.id,
            last_text: item_meta.text.clone(),
        })
        .insert(InteractiveNode);

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

    let has_shadow = item_meta.node_type.clone() == NodeType::Paper;

    if has_shadow {
        let shadow = spawn_shadow(commands, shaders, width, height, theme.shadow, item_meta.id);
        commands.entity(top).add_child(shadow);
    }

    let arrow_marker_1 = spawn_arrow_marker(
        commands,
        theme,
        item_meta.id,
        width,
        height,
        ArrowConnectPos::Left,
    );
    let arrow_marker_2 = spawn_arrow_marker(
        commands,
        theme,
        item_meta.id,
        width,
        height,
        ArrowConnectPos::Right,
    );
    let arrow_marker_3 = spawn_arrow_marker(
        commands,
        theme,
        item_meta.id,
        width,
        height,
        ArrowConnectPos::Top,
    );
    let arrow_marker_4 = spawn_arrow_marker(
        commands,
        theme,
        item_meta.id,
        width,
        height,
        ArrowConnectPos::Bottom,
    );

    let resize_marker_1 =
        spawn_resize_marker(commands, theme, width, height, ResizeMarker::TopLeft);
    let resize_marker_2 =
        spawn_resize_marker(commands, theme, width, height, ResizeMarker::TopRight);
    let resize_marker_3 =
        spawn_resize_marker(commands, theme, width, height, ResizeMarker::BottomLeft);
    let resize_marker_4 =
        spawn_resize_marker(commands, theme, width, height, ResizeMarker::BottomRight);

    commands.entity(top).add_child(border);
    commands.entity(border).add_child(cosmic_edit);
    commands.entity(top).add_child(arrow_marker_1);
    commands.entity(top).add_child(arrow_marker_2);
    commands.entity(top).add_child(arrow_marker_3);
    commands.entity(top).add_child(arrow_marker_4);
    commands.entity(top).add_child(resize_marker_1);
    commands.entity(top).add_child(resize_marker_2);
    commands.entity(top).add_child(resize_marker_3);
    commands.entity(top).add_child(resize_marker_4);
    border
}

fn spawn_resize_marker(
    commands: &mut Commands,
    theme: &Res<Theme>,
    width: f32,
    height: f32,
    pos: ResizeMarker,
) -> Entity {
    let (x, y) = match pos {
        ResizeMarker::BottomLeft => (-width / 2., -height / 2.),
        ResizeMarker::BottomRight => (width / 2., -height / 2.),
        ResizeMarker::TopLeft => (-width / 2., height / 2.),
        ResizeMarker::TopRight => (width / 2., height / 2.),
    };
    let resize_marker = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(Vec2::new(
                    theme.resize_marker_size,
                    theme.resize_marker_size,
                )),
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x, y, z: 0.3 },
                ..default()
            },
            ..default()
        })
        .id();
    commands
        .entity(resize_marker)
        .insert(pos)
        .insert(InteractiveNode);
    resize_marker
}

fn spawn_arrow_marker(
    commands: &mut Commands,
    theme: &Res<Theme>,
    id: ReflectableUuid,
    width: f32,
    height: f32,
    pos: ArrowConnectPos,
) -> Entity {
    let (x, y) = match pos {
        ArrowConnectPos::Left => (-width / 2., 0.),
        ArrowConnectPos::Bottom => (0., -height / 2.),
        ArrowConnectPos::Top => (0., height / 2.),
        ArrowConnectPos::Right => (width / 2., 0.),
    };
    let arrow_marker = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: theme.arrow_connector,
                custom_size: Some(Vec2::new(
                    theme.arrow_connector_size,
                    theme.arrow_connector_size,
                )),
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x, y, z: 0.3 },
                ..default()
            },
            ..default()
        })
        .id();
    commands
        .entity(arrow_marker)
        .insert(ArrowConnect { pos, id })
        .insert(InteractiveNode);
    arrow_marker
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
    let button = commands
        .spawn((
            create_rectangle_btn(
                item_meta.bg_color,
                item_meta.z_index,
                item_meta.text_pos.clone(),
            ),
            VeloNode { id: item_meta.id },
        ))
        .id();
    let outline_color = match item_meta.node_type {
        NodeType::Rect => theme.node_border,
        NodeType::Circle => theme.node_border.with_a(0.),
        NodeType::Paper => todo!(),
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
        bg_image: item_meta.image,
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
