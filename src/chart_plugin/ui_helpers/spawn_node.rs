use bevy_markdown::{spawn_bevy_markdown, BevyMarkdown};
use bevy_ui_borders::{BorderColor, Outline};

use bevy::prelude::*;

use crate::TextPos;

use super::{
    create_arrow_marker, create_rectangle_btn, create_rectangle_txt, create_resize_marker,
    BevyMarkdownView, EditableText, RawText, ResizeMarker, VeloNode, VeloNodeContainer,
};
use crate::canvas::arrow::components::{ArrowConnect, ArrowConnectPos};
use crate::utils::ReflectableUuid;

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
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
    asset_server: &Res<AssetServer>,
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
                // background_color: Color::BLACK.with_a(0.5).into(),
                ..default()
            },
            VeloNodeContainer { id: item_meta.id },
        ))
        .id();
    let button = commands
        .spawn((
            create_rectangle_btn(
                item_meta.bg_color,
                item_meta.image,
                item_meta.z_index,
                item_meta.text_pos,
            ),
            VeloNode { id: item_meta.id },
            Outline::all(
                Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0),
                Val::Px(1.),
            ),
        ))
        .id();
    let arrow_marker1 = commands
        .spawn((
            create_arrow_marker(50.0, 0., 0., 0.),
            BorderColor(Color::BLUE.with_a(0.5)),
            ArrowConnect {
                pos: ArrowConnectPos::Top,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker2 = commands
        .spawn((
            create_arrow_marker(0., 0., 50., 0.),
            BorderColor(Color::BLUE.with_a(0.5)),
            ArrowConnect {
                pos: ArrowConnectPos::Left,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker3 = commands
        .spawn((
            create_arrow_marker(50., 0., 100., 0.),
            BorderColor(Color::BLUE.with_a(0.5)),
            ArrowConnect {
                pos: ArrowConnectPos::Bottom,
                id: item_meta.id,
            },
        ))
        .id();
    let arrow_marker4 = commands
        .spawn((
            create_arrow_marker(100., 0., 50., 0.),
            BorderColor(Color::BLUE.with_a(0.5)),
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
    let raw_text = commands
        .spawn((
            create_rectangle_txt(
                item_meta.text.clone(),
                Some(item_meta.size),
                item_meta.is_active,
            ),
            RawText { id: item_meta.id },
            EditableText { id: item_meta.id },
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
    commands.entity(button).add_child(raw_text);
    if !item_meta.is_active {
        let bevy_markdown = BevyMarkdown {
            text: item_meta.text.clone(),
            regular_font: Some(TextStyle::default().font),
            bold_font: Some(asset_server.load("fonts/SourceCodePro-Bold.ttf")),
            italic_font: Some(asset_server.load("fonts/SourceCodePro-Italic.ttf")),
            semi_bold_italic_font: Some(
                asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
            ),
            max_size: Some(item_meta.size),
        };
        let markdown_text = spawn_bevy_markdown(commands, bevy_markdown).unwrap();
        commands
            .get_entity(markdown_text)
            .unwrap()
            .insert(BevyMarkdownView { id: item_meta.id });
        commands.entity(button).add_child(markdown_text);
    }
    commands.entity(top).add_child(button);
    top
}
