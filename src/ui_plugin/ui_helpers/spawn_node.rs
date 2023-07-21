use bevy_cosmic_edit::{
    spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditSprite, CosmicFont, CosmicMetrics,
    CosmicNode, CosmicText,
};
use bevy_markdown::{generate_markdown_lines, BevyMarkdown, BevyMarkdownTheme};
use bevy_prototype_lyon::prelude::{Fill, Path, Stroke};

use bevy::prelude::*;
use cosmic_text::AttrsOwned;

use crate::canvas::shadows::systems::spawn_shadow;
use crate::canvas::shadows::CustomShadowMaterial;
use crate::themes::Theme;
use crate::ui_plugin::NodeType;
use crate::TextPos;

use super::{BevyMarkdownView, InteractiveNode, RawText, ResizeMarker, VeloNode, VeloShape};
use crate::canvas::arrow::components::{ArrowConnect, ArrowConnectPos};
use crate::utils::{bevy_color_to_cosmic, ReflectableUuid};

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
    pub node_type: NodeType,
    pub size: (f32, f32),
    pub position: (f32, f32, f32),
    pub text: String,
    pub pair_bg_color: (String, Color),
    pub image: Option<Handle<Image>>,
    pub text_pos: TextPos,
    pub is_active: bool,
}

pub fn spawn_sprite_node(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<CustomShadowMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    theme: &Res<Theme>,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    scale_factor: f32,
    item_meta: NodeMeta,
) -> Entity {
    let pos: Vec3 = Vec3::new(
        item_meta.position.0,
        item_meta.position.1,
        item_meta.position.2,
    );
    let width: f32 = item_meta.size.0;
    let height = item_meta.size.1;

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

    let path: Path = match item_meta.node_type {
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
    let has_border = item_meta.node_type != NodeType::Paper;
    let shape = commands
        .spawn((
            bevy_prototype_lyon::prelude::ShapeBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.001),
                    ..default()
                },
                path,
                ..default()
            },
            Stroke::new(
                if has_border {
                    theme.node_border
                } else {
                    Color::NONE
                },
                1.,
            ),
            Fill::color(item_meta.pair_bg_color.1),
            VeloShape {
                id: item_meta.id,
                node_type: item_meta.node_type.clone(),
                pair_color: item_meta.pair_bg_color,
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
        text_pos: item_meta.text_pos.clone().into(),
        size: Some((width, height)),
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.002),
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

    if item_meta.node_type == NodeType::Paper {
        let shadow: Entity = spawn_shadow(commands, materials, meshes, theme);
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

    commands.entity(top).add_child(shape);
    commands.entity(shape).add_child(cosmic_edit);
    commands.entity(top).add_child(arrow_marker_1);
    commands.entity(top).add_child(arrow_marker_2);
    commands.entity(top).add_child(arrow_marker_3);
    commands.entity(top).add_child(arrow_marker_4);
    commands.entity(top).add_child(resize_marker_1);
    commands.entity(top).add_child(resize_marker_2);
    commands.entity(top).add_child(resize_marker_3);
    commands.entity(top).add_child(resize_marker_4);
    top
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
                translation: Vec3 { x, y, z: 0.003 },
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
    let arrow_marker_container = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(Vec2::new(
                    6. * theme.arrow_connector_size,
                    6. * theme.arrow_connector_size,
                )),
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x, y, z: 0.003 },
                ..default()
            },
            ..default()
        })
        .id();
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
            ..default()
        })
        .id();
    commands
        .entity(arrow_marker_container)
        .add_child(arrow_marker);
    commands
        .entity(arrow_marker_container)
        .insert(ArrowConnect { pos, id })
        .insert(InteractiveNode);
    arrow_marker_container
}
