use bevy_ui_borders::{BorderColor, Outline};

use bevy::prelude::*;

use crate::TextPos;

use super::{
    create_arrow_marker, create_rectangle_btn, create_rectangle_txt, create_resize_marker,
    ArrowConnect, ArrowConnectPos, EditableText, Rectangle, ReflectableUuid, ResizeMarker,
};

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
    pub size: (Val, Val),
    pub position: (Val, Val),
    pub text: String,
    pub bg_color: Color,
    pub font: Handle<Font>,
    pub image: Option<UiImage>,
    pub tags: Vec<String>,
    pub text_pos: TextPos,
    pub z_index: i32,
}

pub fn spawn_node(commands: &mut Commands, item_meta: NodeMeta) -> Entity {
    commands
        .spawn((
            create_rectangle_btn(
                item_meta.size,
                item_meta.position,
                item_meta.bg_color,
                item_meta.image,
                item_meta.z_index,
                item_meta.text_pos,
            ),
            Rectangle { id: item_meta.id },
            Outline::all(Color::BLACK, Val::Px(1.)),
        ))
        .with_children(|builder| {
            builder.spawn((
                create_arrow_marker(50.0, 0., 0., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Top,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(0., 0., 50., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Left,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(50., 0., 100., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Bottom,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(100., 0., 50., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Right,
                    id: item_meta.id,
                },
            ));
            builder.spawn((create_resize_marker(0., 0., 0., 0.), ResizeMarker::TopLeft));
            builder.spawn((
                create_resize_marker(100., 0., 0., 0.),
                ResizeMarker::TopRight,
            ));
            builder.spawn((
                create_resize_marker(100., 0., 100., 0.),
                ResizeMarker::BottomRight,
            ));
            builder.spawn((
                create_resize_marker(0., 0., 100., 0.),
                ResizeMarker::BottomLeft,
            ));
            builder.spawn((
                create_rectangle_txt(item_meta.font, item_meta.text, Some(item_meta.size)),
                EditableText { id: item_meta.id },
            ));
        })
        .id()
}
